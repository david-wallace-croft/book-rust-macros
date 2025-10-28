use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::syn::{DeriveInput, Ident};

pub fn create_builder(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse2(item).unwrap();

  let name: Ident = ast.ident;

  let builder: Ident = format_ident!("{}Builder", name);

  quote! {
    struct #builder {}
  }
}
