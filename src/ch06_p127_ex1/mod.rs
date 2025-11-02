use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::{format_ident, quote};
use ::syn::{
  Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident,
  punctuated::Punctuated, token::Comma,
};
use syn::Type;

#[allow(dead_code)]
pub fn create_builder_ex1(item: TokenStream2) -> TokenStream2 {
  let ast: DeriveInput = syn::parse2(item).unwrap();

  let name: Ident = ast.ident;

  let builder: Ident = format_ident!("{}Builder", name);

  let fields: &Punctuated<Field, Comma> = match ast.data {
    Data::Struct(DataStruct {
      fields: Fields::Named(FieldsNamed {
        ref named,
        ..
      }),
      ..
    }) => named,
    _ => unimplemented!(),
  };

  let builder_fields = builder_field_definitions(fields);

  let builder_inits = builder_init_values(fields);

  let builder_methods = builder_methods(fields);

  let set_fields = original_struct_setters(fields);

  quote! {
    struct #builder {
      #(#builder_fields,)*
    }

    impl #builder {
      #(#builder_methods)*

      pub fn build(self) -> #name {
        #name {
          #(#set_fields,)*
        }
      }
    }

    impl #name {
      pub fn builder() -> #builder {
        #builder {
          #(#builder_inits,)*
        }
      }
    }
  }
}

fn builder_init_values(
  fields: &Punctuated<Field, Comma>
) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|f: &Field| {
    let field_name: &Option<Ident> = &f.ident;

    quote! { #field_name: None }
  })
}

fn builder_field_definitions(
  fields: &Punctuated<Field, Comma>
) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|f: &Field| {
    let (field_name, field_type) = get_name_and_type(f);

    quote! { #field_name: Option<#field_type> }
  })
}

fn builder_methods(
  fields: &Punctuated<Field, Comma>
) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|f: &Field| {
    let field_name: &Option<Ident> = &f.ident;

    let field_type: &Type = &f.ty;

    quote! {
      pub fn #field_name(mut self, input: #field_type) -> Self {
        self.#field_name = Some(input);

        self
      }
    }
  })
}

fn get_name_and_type(f: &Field) -> (&Option<Ident>, &Type) {
  let field_name: &Option<Ident> = &f.ident;

  let field_type: &Type = &f.ty;

  (field_name, field_type)
}

fn original_struct_setters(
  fields: &Punctuated<Field, Comma>
) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|f: &Field| {
    let field_name: &Option<Ident> = &f.ident;

    let field_name_as_string: String = field_name.as_ref().unwrap().to_string();

    quote! {
      #field_name: self.#field_name.expect(
        concat!("field not set: ", #field_name_as_string))
    }
  })
}

mod test {
  #![allow(unused_imports)]
  use super::*;
  use ::syn::parse2;

  #[test]
  fn test1() {
    let input: TokenStream2 = quote! {
      struct StructWithFields {
        alpha: String,
        beta: u32,
      }
    };

    let actual: TokenStream2 = create_builder_ex1(input);

    assert!(actual.to_string().contains("StructWithFieldsBuilder"));
  }

  #[test]
  fn test2() {
    let input: TokenStream2 = quote! {
      struct StructWithFields {
        alpha: String,
        beta: u32,
      }
    };

    let expected: TokenStream2 = quote! {
      struct StructWithFieldsBuilder {
        alpha: Option<String>,
        beta: Option<u32>,
      }

      impl StructWithFieldsBuilder {
        pub fn alpha (mut self, input : String) -> Self {
          self.alpha = Some (input);

          self
        }

        pub fn beta (mut self, input : u32) -> Self {
          self.beta = Some (input);

          self
        }

        pub fn build (self) -> StructWithFields {
          StructWithFields {
            alpha: self.alpha.expect(concat!("field not set: ", "alpha")),
            beta : self.beta.expect(concat!("field not set: ", "beta")),
          }
        }
      }

      impl StructWithFields {
        pub fn builder() -> StructWithFieldsBuilder {
          StructWithFieldsBuilder {
            alpha: None,
            beta: None,
          }
        }
      }
    };

    let actual: TokenStream2 = create_builder_ex1(input);

    assert_eq!(actual.to_string(), expected.to_string());
  }
}
