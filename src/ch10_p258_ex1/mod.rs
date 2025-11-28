use ::proc_macro2::Span;
use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
use ::std::{collections::HashMap, fs::File, io};
use ::syn::{
  LitStr, Token,
  parse::{Parse, ParseStream},
};
use syn::DeriveInput;
use syn::Ident;

pub fn find_yaml_values_ex1(
  config_input_features: ConfigInputEx1
) -> Result<HashMap<String, String>, syn::Error> {
  let file_name: String = config_input_features
    .path
    .unwrap_or_else(|| "./configuration/config.yaml".to_string());

  let file: File = File::open(&file_name).map_err(|err: io::Error| {
    syn::Error::new(
      Span::call_site(),
      format!("could not read config with path {}: {}", &file_name, err),
    )
  })?;

  serde_yaml::from_reader(file).map_err(|e: serde_yaml::Error| {
    syn::Error::new(Span::call_site(), e.to_string())
  })
}

pub fn generate_annotation_ex1(
  derive_input: DeriveInput,
  yaml_values: HashMap<String, String>,
  exclude_from_method: &bool,
) -> TokenStream2 {
  let attributes: &Vec<syn::Attribute> = &derive_input.attrs;

  let name: &Ident = &derive_input.ident;

  let fields: Vec<TokenStream2> = generate_fields(&yaml_values);

  let inits: Vec<TokenStream2> = generate_inits(&yaml_values);

  let from: TokenStream2 = if !exclude_from_method {
    generate_from_method(name, &yaml_values)
  } else {
    quote!()
  };

  quote! {
    #(#attributes)*
    pub struct #name {
      #(#fields,)*
    }

    impl #name {
      pub fn new() -> Self {
        #name {
          #(#inits,)*
        }
      }
    }

    #from
  }
}

fn generate_fields(yaml_values: &HashMap<String, String>) -> Vec<TokenStream2> {
  yaml_values
    .iter()
    .map(|v: (&String, &String)| {
      let key: Ident = Ident::new(v.0, Span::call_site());

      quote! {
        pub #key: String
      }
    })
    .collect()
}

fn generate_from_method(
  name: &Ident,
  yaml_values: &HashMap<String, String>,
) -> TokenStream2 {
  let inserts: Vec<TokenStream2> = generate_inserts_for_from(yaml_values);

  quote! {
    impl From<#name> for std::collections::HashMap<String, String> {
      fn from(value: #name) -> Self {
        let mut map = std::collections::HashMap::new();

        #(#inserts)*

        map
      }
    }
  }
}

fn generate_inits(yaml_values: &HashMap<String, String>) -> Vec<TokenStream2> {
  yaml_values
    .iter()
    .map(|v: (&String, &String)| {
      let key: Ident = Ident::new(v.0, Span::call_site());

      let value: &String = v.1;

      quote! {
        #key: #value.to_string()
      }
    })
    .collect()
}

fn generate_inserts_for_from(
  yaml_values: &HashMap<String, String>
) -> Vec<TokenStream2> {
  yaml_values
    .iter()
    .map(|v: (&String, &String)| {
      let key: &String = v.0;

      let key_as_ident = Ident::new(key, Span::call_site());

      quote!(
        map.insert(#key.to_string(), value.#key_as_ident);
      )
    })
    .collect()
}

#[derive(Debug)]
pub struct ConfigInputEx1 {
  pub exclude_from: bool,
  pub path: Option<String>,
}

impl Parse for ConfigInputEx1 {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut exclude_from: bool = false;

    let mut path: Option<String> = None;

    loop {
      if input.is_empty() {
        return Ok(ConfigInputEx1 {
          exclude_from,
          path,
        });
      }

      if input.peek(kw::exclude) {
        let _: kw::exclude = input.parse().expect("checked that this exists");

        let _: Token!(=) = input.parse().map_err(|_| {
          syn::Error::new(input.span(), "expected equals sign after exclude")
        })?;

        let value: LitStr = input.parse().map_err(|_| {
          syn::Error::new(input.span(), "expected value after the equals sign")
        })?;

        exclude_from = value.value() == "from";
      } else if input.peek(kw::path) {
        let _: kw::path = input.parse().expect("checked that this exists");

        let _: Token!(=) = input.parse().map_err(|_| {
          syn::Error::new(input.span(), "expected equals sign after path")
        })?;

        let value: LitStr = input.parse().map_err(|_| {
          syn::Error::new(input.span(), "expected value after the equals sign")
        })?;

        path = Some(value.value());
      } else if input.peek(Token!(,)) {
        let _: Token!(,) = input.parse().expect("checked that this exists");
      } else {
        return Err(syn::Error::new(
          input.span(),
          "config macro only allows for 'exclude' or 'path' input",
        ));
      }
    }
  }
}

mod kw {
  use ::syn::custom_keyword;

  custom_keyword!(exclude);
  custom_keyword!(path);
}
