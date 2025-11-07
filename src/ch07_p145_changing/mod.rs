use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
use syn::{Expr, Stmt, StmtMacro, token::Semi};

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

pub fn handle_expression(
  expression: Expr,
  token: Option<Semi>,
) -> Stmt {
  match expression {
    Expr::If(mut expr_if) => {
      let new_statements: Vec<Stmt> = expr_if
        .then_branch
        .stmts
        .into_iter()
        .map(|s| match s {
          Stmt::Macro(ref expr_macro) => extract_panic_content(expr_macro)
            .map(|t: TokenStream2| {
              quote! {
                return Err(#t.to_string());
              }
            })
            .map(syn::parse2)
            .map(Result::unwrap)
            .unwrap_or(s),
          _ => s,
        })
        .collect();

      expr_if.then_branch.stmts = new_statements;

      Stmt::Expr(Expr::If(expr_if), token)
    },
    _ => Stmt::Expr(expression, token),
  }
}
