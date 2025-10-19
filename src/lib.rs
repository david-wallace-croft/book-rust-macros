// #![warn(clippy::pedantic)]

use ::proc_macro::TokenStream;
use ::proc_macro::TokenTree;
use ::quote::quote;
use ::std::sync::Once;
use ::syn::{DeriveInput, Ident, parse_macro_input};
use ::venial::{Declaration, Enum, Struct, parse_declaration};

mod ch02_p013_creating;
mod ch02_p019_varargs;
mod ch02_p023_newtypes;
mod ch02_p028_dsls;
mod ch02_p031_composing;
mod ch02_p038_ex1;
mod ch02_p038_ex2;
mod ch02_p038_ex3;
mod ch02_p038_ex4;
mod ch02_p038_ex5;
mod ch02_p038_ex6;

static TRACING_INIT: Once = Once::new();

#[allow(dead_code)]
fn init_tracing() {
  TRACING_INIT.call_once(|| {
    // https://www.reddit.com/r/rust/
    //  comments/18shil2/idiomatic_way_to_use_tracing_log_framework_in/

    tracing_subscriber::fmt::init();
  });
}

// For test_ch03_p048_generating
#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  let add_hello_world: proc_macro2::TokenStream = quote! {
    impl #name {
      fn hello_world(&self) -> String {
        "Hello, World!".into()
      }
    }
  };

  add_hello_world.into()
}

// For test_ch03_p052_same
#[proc_macro_derive(HelloAlt)]
pub fn hello_alt(item: TokenStream) -> TokenStream {
  fn ident_name(item: TokenTree) -> String {
    match item {
      TokenTree::Ident(i) => i.to_string(),
      _ => panic!("no ident"),
    }
  }

  let name: String = ident_name(item.into_iter().nth(1).unwrap());

  let formatted: String = format!(
    "impl {} {{ fn hello_alt(&self) -> String \
    {{ format!(\"Howdy, World!\") }} }} ",
    name
  );

  let parsed: Result<TokenStream, proc_macro::LexError> = formatted.parse();

  parsed.unwrap()
}

// For test_ch03_p053_venial
#[proc_macro_derive(HelloVenial)]
pub fn hello_venial(item: TokenStream) -> TokenStream {
  let declaration: Declaration = parse_declaration(item.into()).unwrap();

  let name: Ident = match declaration {
    Declaration::Struct(Struct {
      name,
      ..
    }) => name,
    Declaration::Enum(Enum {
      name,
      ..
    }) => name,
    _ => panic!("only implemented for struct and enum"),
  };

  let add_hello_world: proc_macro2::TokenStream = quote! {
    impl #name {
      fn hello_venial(&self) -> String {
        "Aloha, World!".into()
      }
    }
  };

  add_hello_world.into()
}

// For test_ch03_p055_ex1
// For test_ch03_p055_ex3
#[proc_macro_derive(UpperCaseName)]
pub fn uppercase(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  let uppercase_name: String = name.to_string().to_uppercase();

  let add_uppercase: proc_macro2::TokenStream = quote! {
    impl #name {
      fn uppercase(&self) -> String {
        format!("{}", #uppercase_name)
      }

      fn testing_testing() -> String {
        "one two three".into()
      }
    }
  };

  add_uppercase.into()
}

// For test_ch03_p055_ex4
#[proc_macro_derive(HelloInput)]
pub fn hello_input(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  let add_hello_world: proc_macro2::TokenStream = quote! {
    impl #name {
      fn hello_input(&self) -> String {
        format!("Hello, {}!", stringify!(#name))
      }
    }
  };

  add_hello_world.into()
}

// For test_ch04_p059_attribute
#[proc_macro_attribute]
pub fn public(
  _attr: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let _ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let public_version = quote! {};

  public_version.into()
}
