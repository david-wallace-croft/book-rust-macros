use ::proc_macro2::Span;
use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
use ::std::{collections::HashMap, fs::File, io};
use ::syn::{
  LitStr, Token,
  parse::{Parse, ParseStream},
};

pub fn find_yaml_values(
  config_input: ConfigInput
) -> Result<HashMap<String, String>, syn::Error> {
  let file_name: String = config_input
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

pub fn generate_config_struct(
  yaml_values: HashMap<String, String>
) -> TokenStream2 {
  let inserts: Vec<TokenStream2> = generate_inserts(yaml_values);

  quote! {
    pub struct Config(pub std::collections::HashMap<String, String>);

    impl Config {
      pub fn new() -> Self {
        let mut map = std::collections::HashMap::new();

        #(#inserts)*

        Config(map)
      }
    }
  }
}

fn generate_inserts(yaml_values: HashMap<String, String>) -> Vec<TokenStream2> {
  yaml_values
    .iter()
    .map(|v: (&String, &String)| {
      let key: &String = v.0;

      let value: &String = v.1;

      quote!(
        map.insert(#key.to_string(), #value.to_string());
      )
    })
    .collect()
}

#[derive(Debug)]
pub struct ConfigInput {
  pub path: Option<String>,
}

impl Parse for ConfigInput {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.is_empty() {
      return Ok(ConfigInput {
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

    Ok(ConfigInput {
      path: Some(value.value()),
    })
  }
}

mod kw {
  use ::syn::custom_keyword;

  custom_keyword!(path);
}
