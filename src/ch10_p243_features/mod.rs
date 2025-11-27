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

pub fn find_yaml_values_features(
  config_input_features: ConfigInputFeatures
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

pub fn generate_annotation_features(
  derive_input: DeriveInput,
  yaml_values: HashMap<String, String>,
) -> TokenStream2 {
  let attributes: &Vec<syn::Attribute> = &derive_input.attrs;

  let name: &Ident = &derive_input.ident;

  let fields: Vec<TokenStream2> = generate_fields(&yaml_values);

  let inits: Vec<TokenStream2> = generate_inits(&yaml_values);

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

#[derive(Debug)]
pub struct ConfigInputFeatures {
  pub path: Option<String>,
}

impl Parse for ConfigInputFeatures {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.is_empty() {
      return Ok(ConfigInputFeatures {
        path: None,
      });
    }

    if !input.peek(kw::path) {
      return Err(syn::Error::new(
        input.span(),
        "config macro only allows for 'path' input",
      ));
    }

    let _: kw::path = input.parse().expect("checked that this exists");

    let _: Token!(=) = input.parse().map_err(|_| {
      syn::Error::new(input.span(), "expected equals sign after path")
    })?;

    let value: LitStr = input.parse().map_err(|_| {
      syn::Error::new(input.span(), "expected value after the equals sign")
    })?;

    Ok(ConfigInputFeatures {
      path: Some(value.value()),
    })
  }
}

mod kw {
  use ::syn::custom_keyword;

  custom_keyword!(path);
}
