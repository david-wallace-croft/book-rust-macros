use quote::{ToTokens, quote};
use syn::{
  Ident, Token,
  parse::{Parse, ParseStream},
  punctuated::{Iter, Punctuated},
};

pub struct ComposeInput {
  expressions: Punctuated<Ident, Token!(.)>,
}

impl Parse for ComposeInput {
  fn parse(input: ParseStream) -> Result<Self, syn::Error> {
    Ok(ComposeInput {
      expressions: Punctuated::<Ident, Token!(.)>::parse_terminated(input)
        .unwrap(),
    })
  }
}

impl ToTokens for ComposeInput {
  fn to_tokens(
    &self,
    tokens: &mut proc_macro2::TokenStream,
  ) {
    let mut total: Option<proc_macro2::TokenStream> = None;

    let ident_iter: Iter<'_, Ident> = self.expressions.iter();

    let mut as_idents: Vec<&Ident> = ident_iter.collect();

    let last_ident: &Ident = as_idents.pop().unwrap();

    as_idents.iter().rev().for_each(|i: &&Ident| {
      if let Some(current_total) = &total {
        total = Some(quote!(
            compose_two(#i, #current_total)
        ));
      } else {
        total = Some(quote!(
            compose_two(#i, #last_ident)
        ))
      }
    });

    total.to_tokens(tokens);
  }
}
