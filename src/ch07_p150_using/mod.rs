use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
use quote::ToTokens;
use syn::{
  Expr, ItemFn, ReturnType, Stmt, StmtMacro, spanned::Spanned, token::Semi,
};

fn extract_panic_content(expr_macro: &StmtMacro) -> Option<TokenStream2> {
  let does_panic = expr_macro
    .mac
    .path
    .segments
    .iter()
    .any(|v| v.ident.to_string().eq("panic"));

  if does_panic {
    Some(expr_macro.mac.tokens.clone())
  } else {
    None
  }
}

pub fn handle_expression_using(
  expression: Expr,
  token: Option<Semi>,
) -> Result<Stmt, syn::Error> {
  match expression {
    Expr::If(mut expr_if) => {
      let new_statements: Result<Vec<Stmt>, syn::Error> = expr_if
        .then_branch
        .stmts
        .into_iter()
        .map(|s: Stmt| match s {
          Stmt::Macro(ref expr_macro) => {
            let output: Option<TokenStream2> =
              extract_panic_content(expr_macro);

            if output.map(|v: TokenStream2| v.is_empty()).unwrap_or(false) {
              Err(syn::Error::new(
                expr_macro.span(),
                "Please make sure every panic in your function has a message",
              ))
            } else {
              Ok(
                extract_panic_content(expr_macro)
                  .map(|t: TokenStream2| {
                    quote! {
                      return Err(#t.to_string());
                    }
                  })
                  .map(syn::parse2)
                  .map(Result::unwrap)
                  .unwrap_or(s),
              )
            }
          },
          _ => Ok(s),
        })
        .collect();

      expr_if.then_branch.stmts = new_statements?;

      Ok(Stmt::Expr(Expr::If(expr_if), token))
    },
    _ => Ok(Stmt::Expr(expression, token)),
  }
}

pub fn signature_output_as_result_using(
  ast: &ItemFn
) -> Result<ReturnType, syn::Error> {
  let output: TokenStream2 = match ast.sig.output {
    ReturnType::Default => {
      quote! { -> Result<(), String>}
    },
    ReturnType::Type(_, ref ty) => {
      if ty.to_token_stream().to_string().contains("Result") {
        return Err(syn::Error::new(
          ast.sig.span(),
          format!(
            "This macro can only be applied to a function that does not return \
            a Result.  Signature: {}",
            quote!(#ty)
          ),
        ));
      }

      quote! {
        -> Result<#ty, String>
      }
    },
  };

  Ok(syn::parse2(output).unwrap())
}
