use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::syn::{DeriveInput, Ident};

#[allow(dead_code)]
pub fn create_builder(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse2(item).unwrap();

  let name: Ident = ast.ident;

  let builder: Ident = format_ident!("{}Builder", name);

  quote! {
    struct #builder {}
  }
}

mod test {
  #![allow(unused_imports)]
  use super::*;
  use ::syn::parse2;

  #[test]
  fn test1() {
    let input: TokenStream = quote! {
      struct StructWithNoFields {}
    };

    let actual: TokenStream = create_builder(input);

    assert!(actual.to_string().contains("StructWithNoFieldsBuilder"));
  }

  #[test]
  fn test2() {
    let input: TokenStream = quote! {
      struct StructWithNoFields {}
    };

    let expected: TokenStream = quote! {
      struct StructWithNoFieldsBuilder {}
    };

    let actual: TokenStream = create_builder(input);

    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn test3() {
    let input: TokenStream = quote! {
      struct StructWithNoFields {}
    };

    let actual: TokenStream = create_builder(input);

    let derived: DeriveInput = parse2(actual).unwrap();

    let name: Ident = derived.ident;

    assert_eq!(name.to_string(), "StructWithNoFieldsBuilder");
  }
}
