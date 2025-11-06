use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
use syn::{ItemFn, ReturnType, Stmt};

pub fn last_statement_as_result(last_statement: Option<Stmt>) -> Stmt {
  let last_unwrapped: Stmt = last_statement.unwrap();

  let last_modified: TokenStream2 = quote! {
    Ok(#last_unwrapped)
  };

  Stmt::Expr(syn::parse2(last_modified).unwrap(), None)
}

pub fn signature_output_as_result(ast: &ItemFn) -> ReturnType {
  let output: TokenStream2 = match ast.sig.output {
    ReturnType::Default => {
      quote! { -> Result<(), String>}
    },
    ReturnType::Type(_, ref ty) => {
      quote! { -> Result<#ty, String> }
    },
  };

  syn::parse2(output).unwrap()
}
