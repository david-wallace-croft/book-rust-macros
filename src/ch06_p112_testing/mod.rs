use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::{format_ident, quote};
use ::syn::{
  Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident,
  punctuated::Punctuated, token::Comma,
};
use syn::Type;

#[allow(dead_code)]
pub fn create_builder_testing(item: TokenStream2) -> TokenStream2 {
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

      pub fn build(&self) -> #name {
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
      pub fn #field_name(&mut self, input: #field_type) -> &mut Self {
        self.#field_name = Some(input);

        self
      }
    }
  })
}

fn get_name_and_type<'a>(f: &'a Field) -> (&'a Option<Ident>, &'a Type) {
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
      #field_name: self.#field_name.as_ref()
        .expect(&format!("field {} not set", #field_name_as_string))
        .to_string()
    }
  })
}

mod test {
  #![allow(unused_imports)]
  use super::*;
  use proc_macro2::{Ident, Span};
  use syn::{
    Field, FieldMutability, Path, PathSegment, Type, TypePath, Visibility,
    punctuated::Punctuated, token::PathSep,
  };

  #[test]
  fn get_name_and_type_give_back_name() {
    let p: PathSegment = PathSegment {
      ident: Ident::new("String", Span::call_site()),
      arguments: Default::default(),
    };

    let mut pun: Punctuated<PathSegment, PathSep> = Punctuated::new();

    pun.push(p);

    let ty: Type = Type::Path(TypePath {
      qself: None,
      path: Path {
        leading_colon: None,
        segments: pun,
      },
    });

    let f: Field = Field {
      attrs: vec![],
      vis: Visibility::Inherited,
      mutability: FieldMutability::None,
      ident: Some(Ident::new("example", Span::call_site())),
      colon_token: None,
      ty,
    };

    let (actual_name, _) = get_name_and_type(&f);

    assert_eq!(
      actual_name.as_ref().unwrap().to_string(),
      "example".to_string()
    );
  }
}
