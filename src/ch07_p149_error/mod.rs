use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
use quote::ToTokens;
use syn::{ItemFn, ReturnType};

pub fn signature_output_as_result_error(ast: &ItemFn) -> ReturnType {
  let output: TokenStream2 = match ast.sig.output {
    ReturnType::Default => {
      quote! { -> Result<(), String>}
    },
    ReturnType::Type(_, ref ty) => {
      if ty.to_token_stream().to_string().contains("Result") {
        unimplemented!(
          "cannot use macro on a function with Result as return type!"
        );
      }

      quote! {
        -> Result<#ty, String>
      }
    },
  };

  syn::parse2(output).unwrap()
}
