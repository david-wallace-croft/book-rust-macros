// #![warn(clippy::pedantic)]

use crate::ch04_p069_parse::StructFieldParse;
use crate::ch04_p071_going::StructFieldGoing;

use self::ch04_p067_more::StructField;
use ::proc_macro::TokenStream;
use ::proc_macro::TokenTree;
use ::quote::ToTokens;
use ::quote::quote;
use ::std::sync::Once;
use ::syn::punctuated::{IntoIter, Punctuated};
use ::syn::token::Comma;
use ::syn::{
  Data, DeriveInput, Field, Fields, Ident, Type, parse_macro_input, parse2,
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
  let item_as_stream: quote::__private::TokenStream = item.clone().into();

  let ast: DeriveInput = parse_macro_input!(item as DeriveInput);

  let name: Ident = ast.ident;

  quote!(
    #item_as_stream

    impl #name {}
  )
  .into()
}
