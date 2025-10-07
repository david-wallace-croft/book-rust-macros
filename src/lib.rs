use ::proc_macro::TokenStream;
use ::quote::quote;
use ::std::sync::Once;
use ::syn::{DeriveInput, Ident, parse_macro_input};

mod ch02_p013_creating;
mod ch02_p019_varargs;
mod ch02_p023_newtypes;
mod ch02_p028_dsls;
mod ch02_p031_composing;
mod ch02_p038_ex1;
mod ch02_p038_ex2;
mod ch02_p038_ex3;
mod ch02_p038_ex4;
mod ch02_p038_ex5;
mod ch02_p038_ex6;

static TRACING_INIT: Once = Once::new();

#[allow(dead_code)]
fn init_tracing() {
  TRACING_INIT.call_once(|| {
    // https://www.reddit.com/r/rust/
    //  comments/18shil2/idiomatic_way_to_use_tracing_log_framework_in/

    tracing_subscriber::fmt::init();
  });
}

// For test_ch03_p048_generating
#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  let add_hello_world: proc_macro2::TokenStream = quote! {
    impl #name {
      fn hello_world(&self) -> String {
        "Hello, World!".into()
      }
    }
  };

  add_hello_world.into()
}
