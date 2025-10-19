#![allow(dead_code)]

use ::quote::{ToTokens, quote};
use ::syn::{Field, Ident, Type};

pub struct StructField {
  pub name: Ident,
  pub ty: Type,
}

impl StructField {
  pub fn new(field: &Field) -> Self {
    let ident_option_ref: &Option<Ident> = &field.ident;

    let ident_option_as_ref: Option<&Ident> = ident_option_ref.as_ref();

    let ident_ref: &Ident = ident_option_as_ref.unwrap();

    let ident_clone: Ident = ident_ref.clone();

    let ty_ref: &Type = &field.ty;

    let ty_clone: Type = ty_ref.clone();

    Self {
      name: ident_clone,
      ty: ty_clone,
    }
  }
}

impl ToTokens for StructField {
  fn to_tokens(
    &self,
    tokens: &mut proc_macro2::TokenStream,
  ) {
    let n: &Ident = &self.name;

    let t: &Type = &self.ty;

    quote!(pub #n: #t).to_tokens(tokens)
  }
}
