#![allow(dead_code)]

use ::quote::{ToTokens, quote};
use ::syn::Result;
use ::syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::{Ident, Visibility};

pub struct StructFieldParse {
  pub name: Ident,
  pub ty: Ident,
}

impl Parse for StructFieldParse {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let _vis: Result<Visibility> = input.parse();

    let list: Punctuated<Ident, Colon> =
      Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();

    Ok(StructFieldParse {
      name: list.first().unwrap().clone(),
      ty: list.last().unwrap().clone(),
    })
  }
}

impl ToTokens for StructFieldParse {
  fn to_tokens(
    &self,
    tokens: &mut proc_macro2::TokenStream,
  ) {
    let n: &Ident = &self.name;

    let t: &Ident = &self.ty;

    quote!(pub #n: #t).to_tokens(tokens)
  }
}
