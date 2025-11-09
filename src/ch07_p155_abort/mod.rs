use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
// use proc_macro_error::abort;
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

pub fn handle_expression_abort(
  expression: Expr,
  token: Option<Semi>,
) -> Stmt {
  match expression {
    Expr::If(mut expr_if) => {
      let new_statements: Vec<Stmt> = expr_if
        .then_branch
        .stmts
        .into_iter()
        .map(|s: Stmt| match s {
          Stmt::Macro(ref expr_macro) => {
            let output: Option<TokenStream2> =
              extract_panic_content(expr_macro);

            if output.map(|v: TokenStream2| v.is_empty()).unwrap_or(false) {
              unimplemented!()
              // abort!(
              //   expr_macro,
              //   "panic needs a message!".to_string();
              //   help = "try to add a message: panic!(\"Example\".to_string())";
              //   note = "we will add the message to Result's Err"
              // );
            } else {
              extract_panic_content(expr_macro)
                .map(|t: TokenStream2| {
                  quote! {
                    return Err(#t.to_string());
                  }
                })
                .map(syn::parse2)
                .map(Result::unwrap)
                .unwrap_or(s)
            }
          },
          _ => s,
        })
        .collect();

      expr_if.then_branch.stmts = new_statements;

      Stmt::Expr(Expr::If(expr_if), token)
    },
    _ => Stmt::Expr(expression, token),
  }
}

pub fn signature_output_as_result_abort(
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
