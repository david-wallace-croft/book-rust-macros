#![allow(dead_code)]

use ::quote::{ToTokens, quote};
use ::syn::parse::{Parse, ParseStream};
use syn::buffer::Cursor;
use syn::{Ident, Result};

pub struct StructFieldGoing {
  pub name: Ident,
  pub ty: Ident,
}

impl Parse for StructFieldGoing {
  fn parse(input: ParseStream) -> Result<Self> {
    let first: (Ident, Cursor<'_>) = input.cursor().ident().unwrap();

    let res: Result<StructFieldGoing> = if first.0.to_string().contains("pub") {
      let second: (Ident, Cursor<'_>) = first.1.ident().unwrap();

      let third: (Ident, Cursor<'_>) =
        second.1.punct().unwrap().1.ident().unwrap();

      Ok(StructFieldGoing {
        name: second.0,
        ty: third.0,
      })
    } else {
      let second: (Ident, Cursor<'_>) =
        first.1.punct().unwrap().1.ident().unwrap();

      Ok(StructFieldGoing {
        name: first.0,
        ty: second.0,
      })
    };

    let _: Result<proc_macro2::TokenStream> = input.parse();

    res
  }
}

impl ToTokens for StructFieldGoing {
  fn to_tokens(
    &self,
    tokens: &mut proc_macro2::TokenStream,
  ) {
    let n: &Ident = &self.name;

    let t: &Ident = &self.ty;

    quote!(pub #n: #t).to_tokens(tokens)
  }
}
