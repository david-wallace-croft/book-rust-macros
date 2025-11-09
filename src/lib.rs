// #![warn(clippy::pedantic)]

use self::ch04_p067_more::StructField;
use self::ch04_p069_parse::StructFieldParse;
use self::ch04_p071_going::StructFieldGoing;
use self::ch05_p083_generating::generated_methods;
use self::ch05_p090_composing::ComposeInput;
use self::ch05_p096_ex2::extract_field_names_as_tokens;
use self::ch05_p096_ex2::generated_fields;
use self::ch06_p101_fleshing::create_builder;
use self::ch06_p105_blackbox::create_builder_blackbox;
use self::ch06_p112_testing::create_builder_testing;
use self::ch06_p116_further::create_builder_further;
use self::ch06_p119_alternative::create_builder_alternative;
use self::ch07_p140_getting::{
  last_statement_as_result, signature_output_as_result,
};
use self::ch07_p145_changing::handle_expression;
use self::ch07_p149_error::signature_output_as_result_error;
use self::ch07_p150_using::{
  handle_expression_using, signature_output_as_result_using,
};
use self::ch07_p155_abort::{
  handle_expression_abort, signature_output_as_result_abort,
};
use ::proc_macro::TokenStream;
use ::proc_macro::TokenTree;
// https://osv.dev/vulnerability/RUSTSEC-2024-0370
// use ::proc_macro_error::proc_macro_error;
use ::quote::ToTokens;
use ::quote::quote;
use ::std::sync::Once;
use ::syn::punctuated::{IntoIter, Punctuated};
use ::syn::token::Comma;
use ::syn::{
  Data, DeriveInput, Field, Fields, Ident, ItemFn, Stmt, Type,
  parse_macro_input, parse2,
};
use ::venial::{Declaration, Enum, Struct, parse_declaration};

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
mod ch04_p067_more;
mod ch04_p069_parse;
mod ch04_p071_going;
mod ch05_p083_generating;
mod ch05_p090_composing;
mod ch05_p096_ex2;
mod ch06_p101_fleshing;
mod ch06_p103_adding;
mod ch06_p105_blackbox;
mod ch06_p112_testing;
mod ch06_p116_further;
mod ch06_p119_alternative;
mod ch06_p127_ex1;
mod ch07_p140_getting;
mod ch07_p145_changing;
mod ch07_p149_error;
mod ch07_p150_using;
mod ch07_p155_abort;

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

// For test_ch03_p052_same
#[proc_macro_derive(HelloAlt)]
pub fn hello_alt(item: TokenStream) -> TokenStream {
  fn ident_name(item: TokenTree) -> String {
    match item {
      TokenTree::Ident(i) => i.to_string(),
      _ => panic!("no ident"),
    }
  }

  let name: String = ident_name(item.into_iter().nth(1).unwrap());

  let formatted: String = format!(
    "impl {} {{ fn hello_alt(&self) -> String \
    {{ format!(\"Howdy, World!\") }} }} ",
    name
  );

  let parsed: Result<TokenStream, proc_macro::LexError> = formatted.parse();

  parsed.unwrap()
}

// For test_ch03_p053_venial
#[proc_macro_derive(HelloVenial)]
pub fn hello_venial(item: TokenStream) -> TokenStream {
  let declaration: Declaration = parse_declaration(item.into()).unwrap();

  let name: Ident = match declaration {
    Declaration::Struct(Struct {
      name,
      ..
    }) => name,
    Declaration::Enum(Enum {
      name,
      ..
    }) => name,
    _ => panic!("only implemented for struct and enum"),
  };

  let add_hello_world: proc_macro2::TokenStream = quote! {
    impl #name {
      fn hello_venial(&self) -> String {
        "Aloha, World!".into()
      }
    }
  };

  add_hello_world.into()
}

// For test_ch03_p055_ex1
// For test_ch03_p055_ex3
#[proc_macro_derive(UpperCaseName)]
pub fn uppercase(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  let uppercase_name: String = name.to_string().to_uppercase();

  let add_uppercase: proc_macro2::TokenStream = quote! {
    impl #name {
      fn uppercase(&self) -> String {
        format!("{}", #uppercase_name)
      }

      fn testing_testing() -> String {
        "one two three".into()
      }
    }
  };

  add_uppercase.into()
}

// For test_ch03_p055_ex4
#[proc_macro_derive(HelloInput)]
pub fn hello_input(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  let add_hello_world: proc_macro2::TokenStream = quote! {
    impl #name {
      fn hello_input(&self) -> String {
        format!("Hello, {}!", stringify!(#name))
      }
    }
  };

  add_hello_world.into()
}

// For test_ch04_p059_attribute
#[proc_macro_attribute]
pub fn public(
  _attr: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let _ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let public_version: proc_macro2::TokenStream = quote! {};

  public_version.into()
}

// For test_ch04_p060_first
#[proc_macro_attribute]
pub fn public_first(
  _attr: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  let public_version: proc_macro2::TokenStream = quote! {
    pub struct #name {
      pub first: String,
      pub second: u32,
    }
  };

  public_version.into()
}

// For test_ch04_p061_getting
#[proc_macro_attribute]
pub fn public_getting(
  _attr: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  let named_fields: Punctuated<Field, Comma> = match ast.data {
    Data::Struct(data_struct) => {
      let fields: Fields = data_struct.fields;

      match fields {
        Fields::Named(fields_named) => fields_named.named,
        Fields::Unnamed(_fields_unnamed) => unimplemented!(),
        Fields::Unit => unimplemented!(),
      }
    },
    Data::Enum(_data_enum) => unimplemented!(),
    Data::Union(_data_union) => unimplemented!(),
  };

  let builder_fields = named_fields.iter().map(|f: &Field| {
    let name: &Option<Ident> = &f.ident;

    let ty: &Type = &f.ty;

    quote! { pub #name: #ty }
  });

  let public_version: proc_macro2::TokenStream = quote! {
    pub struct #name {
      #(#builder_fields,)*
    }
  };

  public_version.into()
}

// For test_ch04_p067_more
#[proc_macro_attribute]
pub fn public_more(
  _attr: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  let named_fields: Punctuated<Field, Comma> = match ast.data {
    Data::Struct(data_struct) => {
      let fields: Fields = data_struct.fields;

      match fields {
        Fields::Named(fields_named) => fields_named.named,
        Fields::Unnamed(_fields_unnamed) => unimplemented!(),
        Fields::Unit => unimplemented!(),
      }
    },
    Data::Enum(_data_enum) => unimplemented!(),
    Data::Union(_data_union) => unimplemented!(),
  };

  let builder_fields = named_fields.iter().map(StructField::new);

  let public_version: proc_macro2::TokenStream = quote! {
    pub struct #name {
      #(#builder_fields,)*
    }
  };

  public_version.into()
}

// For test_ch04_p069_parse
#[proc_macro_attribute]
pub fn public_parse(
  _attr: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  let named_fields: Punctuated<Field, Comma> = match ast.data {
    Data::Struct(data_struct) => {
      let fields: Fields = data_struct.fields;

      match fields {
        Fields::Named(fields_named) => fields_named.named,
        Fields::Unnamed(_fields_unnamed) => unimplemented!(),
        Fields::Unit => unimplemented!(),
      }
    },
    Data::Enum(_data_enum) => unimplemented!(),
    Data::Union(_data_union) => unimplemented!(),
  };

  let builder_fields = named_fields
    .iter()
    .map(|f| parse2::<StructFieldParse>(f.to_token_stream()).unwrap());

  let public_version: proc_macro2::TokenStream = quote! {
    pub struct #name {
      #(#builder_fields,)*
    }
  };

  public_version.into()
}

// For test_ch04_p071_going
#[proc_macro_attribute]
pub fn public_going(
  _attr: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  eprintln!("{:#?}", &ast);

  let name: Ident = ast.ident;

  let named_fields: Punctuated<Field, Comma> = match ast.data {
    Data::Struct(data_struct) => {
      let fields: Fields = data_struct.fields;

      match fields {
        Fields::Named(fields_named) => fields_named.named,
        Fields::Unnamed(_fields_unnamed) => unimplemented!(),
        Fields::Unit => unimplemented!(),
      }
    },
    Data::Enum(_data_enum) => unimplemented!(),
    Data::Union(_data_union) => unimplemented!(),
  };

  let builder_fields = named_fields
    .iter()
    .map(|f| parse2::<StructFieldGoing>(f.to_token_stream()).unwrap());

  let public_version: proc_macro2::TokenStream = quote! {
    pub struct #name {
      #(#builder_fields,)*
    }
  };

  public_version.into()
}

// For test_ch04_p076_ex1
#[proc_macro_attribute]
pub fn delete(
  _attr: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let _ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let public_version: proc_macro2::TokenStream = quote! {};

  public_version.into()
}

// For test_ch04_p076_ex5
#[proc_macro_attribute]
pub fn public_ex5(
  _attr: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  let attributes: &Vec<syn::Attribute> = &ast.attrs;

  let basic_output: proc_macro2::TokenStream = match ast.data {
    Data::Struct(data_struct) => {
      let fields: Fields = data_struct.fields;

      match fields {
        Fields::Named(fields_named) => {
          let punctuated: Punctuated<Field, Comma> = fields_named.named;

          let builder_fields = punctuated.iter().map(|f: &Field| {
            let field_name: &Option<Ident> = &f.ident;

            let ty: &Type = &f.ty;

            quote! { pub #field_name: #ty }
          });

          quote! {
            pub struct #name {
              #(#builder_fields,)*
            }
          }
        },
        Fields::Unnamed(fields_unnamed) => {
          let punctuated: Punctuated<Field, Comma> = fields_unnamed.unnamed;

          let builder_fields = punctuated.iter().map(|f| {
            let ty: &Type = &f.ty;

            quote! { pub #ty }
          });

          quote! {
            pub struct #name(
              #(#builder_fields,)*
            );
          }
        },
        Fields::Unit => unimplemented!(),
      }
    },
    Data::Enum(data_enum) => {
      let punctuated: Punctuated<syn::Variant, Comma> = data_enum.variants;

      let as_iter: IntoIter<syn::Variant> = punctuated.into_iter();

      quote!(
        pub enum #name {
          #(#as_iter,)*
        }
      )
    },
    Data::Union(_data_union) => unimplemented!(),
  };

  let token_stream: proc_macro2::TokenStream = quote! {
    #(#attributes)*
    #basic_output
  };

  token_stream.into()
}

// For test_ch05_p081_recreating
#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
  let item_as_stream: proc_macro2::TokenStream = item.clone().into();

  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  quote!(
    #item_as_stream

    impl #name {}
  )
  .into()
}

// For test_ch05_p083_generating
#[proc_macro]
pub fn private_generating(item: TokenStream) -> TokenStream {
  let item_as_stream: proc_macro2::TokenStream = item.clone().into();

  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: &Ident = &ast.ident;

  let methods: Vec<proc_macro2::TokenStream> = generated_methods(&ast);

  quote!(
    #item_as_stream

    impl #name {
      #(#methods)*
    }
  )
  .into()
}

// For test_ch05_p090_composing
#[proc_macro]
pub fn compose(item: TokenStream) -> TokenStream {
  let ci: ComposeInput = parse_macro_input!(item);

  quote!(
    {
      fn compose_two<FIRST, SECOND, THIRD, F, G>(
        first: F,
        second: G,
      ) -> impl Fn(FIRST) -> THIRD
      where F: Fn(FIRST) -> SECOND, G: Fn(SECOND) -> THIRD,
      {
        move |x| second(first(x))
      }

      #ci
    }
  )
  .into()
}

// For test_ch05_p096_ex1
#[proc_macro]
pub fn hello_world(item: TokenStream) -> TokenStream {
  let ident: Ident = parse_macro_input!(item as Ident);

  quote!(
    impl #ident {
      fn say_hello(&self) -> String {
        "Hello, World!".into()
      }
    }
  )
  .into()
}

// For test_ch05_p096_ex2
#[proc_macro]
pub fn private_ex2(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: &Ident = &ast.ident;

  let field_names: Vec<proc_macro2::TokenStream> =
    extract_field_names_as_tokens(&ast);

  let fields: Vec<proc_macro2::TokenStream> = generated_fields(&ast);

  let methods: Vec<proc_macro2::TokenStream> = generated_methods(&ast);

  quote!(
    struct #name {
      #(#fields,)*
    }

    impl #name {
      pub fn new(#(#fields,)*) -> Self {
        Self {
          #(#field_names,)*
        }
      }

      #(#methods)*
    }
  )
  .into()
}

// For test_ch06_p101_fleshing
#[proc_macro_derive(Builder)]
pub fn builder(item: TokenStream) -> TokenStream {
  create_builder(item.into()).into()
}

// For test_ch06_p105_blackbox
#[proc_macro_derive(BuilderBlackbox)]
pub fn builder_blackbox(item: TokenStream) -> TokenStream {
  create_builder_blackbox(item.into()).into()
}

// For test_ch06_p112_testing
#[proc_macro_derive(BuilderTesting)]
pub fn builder_testing(item: TokenStream) -> TokenStream {
  create_builder_testing(item.into()).into()
}

// For test_ch06_p116_further
#[proc_macro_derive(BuilderFurther)]
pub fn builder_further(item: TokenStream) -> TokenStream {
  create_builder_further(item.into()).into()
}

// For test_ch06_p119_alternative
#[proc_macro_derive(BuilderAlternative)]
pub fn builder_alternative(item: TokenStream) -> TokenStream {
  create_builder_alternative(item.into()).into()
}

// For test_ch07_p137_setup
#[proc_macro_attribute]
pub fn panic_to_result(
  _a: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let ast: ItemFn = syn::parse(item).unwrap();

  ast.to_token_stream().into()
}

// For test_ch07_p140_getting
#[proc_macro_attribute]
pub fn panic_to_result_getting(
  _a: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let mut ast: ItemFn = syn::parse(item).unwrap();

  ast.sig.output = signature_output_as_result(&ast);

  let last_statement_option: Option<Stmt> = ast.block.stmts.pop();

  let last_modified_as_expr: Stmt =
    last_statement_as_result(last_statement_option);

  ast.block.stmts.push(last_modified_as_expr);

  ast.to_token_stream().into()
}

// For test_ch07_p145_changing
#[proc_macro_attribute]
pub fn panic_to_result_changing(
  _a: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let mut ast: ItemFn = syn::parse(item).unwrap();

  ast.sig.output = signature_output_as_result(&ast);

  let new_statements: Vec<Stmt> = ast
    .block
    .stmts
    .into_iter()
    .map(|s| match s {
      Stmt::Expr(e, t) => handle_expression(e, t),
      _ => s,
    })
    .collect();

  ast.block.stmts = new_statements;

  let last_statement_option: Option<Stmt> = ast.block.stmts.pop();

  let last_modified_as_expr: Stmt =
    last_statement_as_result(last_statement_option);

  ast.block.stmts.push(last_modified_as_expr);

  ast.to_token_stream().into()
}

// For test_ch07_p149_error
#[proc_macro_attribute]
pub fn panic_to_result_error(
  _a: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let mut ast: ItemFn = syn::parse(item).unwrap();

  ast.sig.output = signature_output_as_result_error(&ast);

  let new_statements: Vec<Stmt> = ast
    .block
    .stmts
    .into_iter()
    .map(|s| match s {
      Stmt::Expr(e, t) => handle_expression(e, t),
      _ => s,
    })
    .collect();

  ast.block.stmts = new_statements;

  let last_statement_option: Option<Stmt> = ast.block.stmts.pop();

  let last_modified_as_expr: Stmt =
    last_statement_as_result(last_statement_option);

  ast.block.stmts.push(last_modified_as_expr);

  ast.to_token_stream().into()
}

// For test_ch07_p150_using
#[proc_macro_attribute]
pub fn panic_to_result_using(
  _a: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let mut ast: ItemFn = syn::parse(item).unwrap();

  let signature_output: Result<syn::ReturnType, syn::Error> =
    signature_output_as_result_using(&ast);

  let statements_output: Result<Vec<Stmt>, syn::Error> = ast
    .block
    .stmts
    .into_iter()
    .map(|s| match s {
      Stmt::Expr(e, t) => handle_expression_using(e, t),
      _ => Ok(s),
    })
    .collect();

  match (statements_output, signature_output) {
    (Ok(new), Ok(output)) => {
      ast.block.stmts = new;

      ast.sig.output = output;
    },
    (Ok(_), Err(signature_err)) => {
      return signature_err.to_compile_error().into();
    },
    (Err(statement_err), Ok(_)) => {
      return statement_err.to_compile_error().into();
    },
    (Err(mut statement_err), Err(signature_err)) => {
      statement_err.combine(signature_err);

      return statement_err.to_compile_error().into();
    },
  }

  let last_statement_option: Option<Stmt> = ast.block.stmts.pop();

  let last_modified_as_expr: Stmt =
    last_statement_as_result(last_statement_option);

  ast.block.stmts.push(last_modified_as_expr);

  ast.to_token_stream().into()
}

// For test_ch07_p155_proc
// #[proc_macro_error]
#[proc_macro_attribute]
pub fn panic_to_result_abort(
  _a: TokenStream,
  item: TokenStream,
) -> TokenStream {
  let mut ast: ItemFn = syn::parse(item).unwrap();

  let signature_output: syn::ReturnType =
    signature_output_as_result_abort(&ast).unwrap();

  ast.sig.output = signature_output;

  let new_statements = ast
    .block
    .stmts
    .into_iter()
    .map(|s| match s {
      Stmt::Expr(e, t) => handle_expression_abort(e, t),
      _ => s,
    })
    .collect();

  ast.block.stmts = new_statements;

  let last_statement_option: Option<Stmt> = ast.block.stmts.pop();

  let last_modified_as_expr: Stmt =
    last_statement_as_result(last_statement_option);

  ast.block.stmts.push(last_modified_as_expr);

  ast.to_token_stream().into()
}
