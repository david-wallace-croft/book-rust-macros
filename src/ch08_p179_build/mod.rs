use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::{format_ident, quote};
use ::syn::{
  Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Type,
  punctuated::Punctuated, token::Comma,
};

#[allow(dead_code)]
pub fn create_builder_build(item: TokenStream2) -> TokenStream2 {
  let ast: DeriveInput = syn::parse2(item).unwrap();

  let name: Ident = ast.ident;

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

  let builder: TokenStream2 = builder_definition(&name, fields);

  let builder_method_for_struct: TokenStream2 =
    builder_impl_for_struct(&name, fields);

  let marker_and_structs: TokenStream2 =
    marker_trait_and_structs(&name, fields);

  let builder_methods: TokenStream2 = builder_methods(&name, fields);

  quote! {
    #builder

    #builder_method_for_struct

    #marker_and_structs

    #builder_methods
  }
}

////////////////////////////////////////////////////////////////////////////////
// private functions (in alphabetical order)
////////////////////////////////////////////////////////////////////////////////

fn builder_definition(
  name: &Ident,
  fields: &Punctuated<Field, Comma>,
) -> TokenStream2 {
  let builder_fields = fields.iter().map(|f: &Field| {
    let (field_name, field_type) = get_name_and_type(f);

    quote! { #field_name: Option<#field_type> }
  });

  let builder_name: Ident = create_builder_ident(name);

  quote! {
    pub struct #builder_name<T: MarkerTraitForBuilder> {
      marker: std::marker::PhantomData<T>,

      #(#builder_fields,)*
    }
  }
}

fn builder_impl_for_struct(
  struct_name: &Ident,
  fields: &Punctuated<Field, Comma>,
) -> TokenStream2 {
  let builder_inits = fields.iter().map(|f: &Field| {
    let field_name: &Option<Ident> = &f.ident;

    quote! { #field_name: None }
  });

  let first_field_name: Ident = fields
    .first()
    .map(|f: &Field| f.ident.clone().unwrap())
    .unwrap();

  let builder_name: Ident = create_builder_ident(struct_name);

  let generic: Ident =
    create_field_struct_name(&builder_name, &first_field_name);

  quote! {
    impl #struct_name {
      pub fn builder() -> #builder_name<#generic> {
        #builder_name {
          marker: Default::default(),
          #(#builder_inits,)*
        }
      }
    }
  }
}

// fn builder_init_values(
//   fields: &Punctuated<Field, Comma>
// ) -> impl Iterator<Item = TokenStream2> + '_ {
//   fields.iter().map(|f: &Field| {
//     let field_name: &Option<Ident> = &f.ident;

//     quote! { #field_name: None }
//   })
// }

// fn builder_field_definitions(
//   fields: &Punctuated<Field, Comma>
// ) -> impl Iterator<Item = TokenStream2> + '_ {
//   fields.iter().map(|f: &Field| {
//     let (field_name, field_type) = get_name_and_type(f);

//     quote! { #field_name: Option<#field_type> }
//   })
// }

fn builder_for_field(
  builder_name: &Ident,
  field_assignments: &Vec<TokenStream2>,
  current_field: &Field,
  next_field_in_list: &Field,
) -> TokenStream2 {
  let (field_name, field_type) = get_name_and_type(current_field);

  let (next_field_name, _) = get_name_and_type(next_field_in_list);

  let current_field_struct_name: Ident =
    create_field_struct_name(builder_name, field_name.as_ref().unwrap());

  let next_field_struct_name: Ident =
    create_field_struct_name(builder_name, next_field_name.as_ref().unwrap());

  quote! {
    impl #builder_name<#current_field_struct_name> {
      pub fn #field_name(mut self, input: #field_type) -> #builder_name<#next_field_struct_name> {
        self.#field_name = Some(input);

        #builder_name {
          marker: Default::default(),
          #(#field_assignments,)*
        }
      }
    }
  }
}

fn builder_for_final_field(
  builder_name: &Ident,
  field_assignments: &Vec<TokenStream2>,
  field: &Field,
) -> TokenStream2 {
  let (field_name, field_type) = get_name_and_type(field);

  let field_struct_name: Ident =
    create_field_struct_name(builder_name, field_name.as_ref().unwrap());

  quote! {
    impl #builder_name<#field_struct_name> {
      pub fn #field_name(mut self, input: #field_type) -> #builder_name<FinalBuilder> {
        self.#field_name = Some(input);

        #builder_name {
          marker: Default::default(),
          #(#field_assignments,)*
        }
      }
    }
  }
}

fn builder_methods(
  struct_name: &Ident,
  fields: &Punctuated<Field, Comma>,
) -> TokenStream2 {
  let builder_name: Ident = create_builder_ident(struct_name);

  let set_fields: Vec<TokenStream2> = original_struct_setters(fields, false);

  let assignments_for_all_fields: Vec<TokenStream2> =
    get_assignments_for_fields(fields);

  let mut previous_field: Option<&&Field> = None;

  let reversed_names_and_types: Vec<&Field> = fields.iter().rev().collect();

  let methods: Vec<TokenStream2> = reversed_names_and_types
    .iter()
    .map(|f| {
      if let Some(next_in_list) = previous_field {
        previous_field = Some(f);

        builder_for_field(
          &builder_name,
          &assignments_for_all_fields,
          f,
          next_in_list,
        )
      } else {
        previous_field = Some(f);

        builder_for_final_field(&builder_name, &assignments_for_all_fields, f)
      }
    })
    .collect();

  quote! {
    #(#methods)*

    impl #builder_name<FinalBuilder> {
      pub fn build(self) -> #struct_name {
        #struct_name {
          #(#set_fields,)*
        }
      }
    }
  }
}

// fn builder_methods(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream2> {
//   fields
//     .iter()
//     .map(|f: &Field| {
//       let field_name: &Option<Ident> = &f.ident;

//       let field_type: &Type = &f.ty;

//       extract_attribute_from_field(f, "rename")
//         .map(|a: &Attribute| &a.meta)
//         .map(|m: &syn::Meta| match m {
//           Meta::Path(_) => {
//             panic!("expected brackets with name of prop")
//           },
//           Meta::List(nested) => {
//             let a: LitStr = nested.parse_args().unwrap();

//             Ident::new(&a.value(), a.span())
//           },
//           Meta::NameValue(MetaNameValue {
//             value:
//               Expr::Lit(ExprLit {
//                 lit: Lit::Str(literal_string),
//                 ..
//               }),
//             ..
//           }) => Ident::new(&literal_string.value(), literal_string.span()),
//           _ => panic!("expected something else"),
//         })
//         .map(|attr| {
//           quote! {
//             pub fn #attr(mut self, input: #field_type) -> Self {
//               self.#field_name = Some(input);

//               self
//             }
//           }
//         })
//         .unwrap_or_else(|| {
//           quote! {
//             pub fn #field_name(mut self, input: #field_type) -> Self {
//               self.#field_name = Some(input);

//               self
//             }
//           }
//         })
//     })
//     .collect()
// }

fn create_builder_ident(name: &Ident) -> Ident {
  format_ident!("{}Builder", name)
}

fn create_field_struct_name(
  builder_name: &Ident,
  field_name: &Ident,
) -> Ident {
  format_ident!("{}Of{}", field_name, builder_name)
}

fn default_fallback() -> TokenStream2 {
  quote! { unwrap_or_default() }
}

// fn extract_attribute_from_field<'a>(
//   f: &'a Field,
//   name: &'a str,
// ) -> Option<&'a syn::Attribute> {
//   f.attrs.iter().find(|&attr| attr.path().is_ident(name))
// }

fn get_assignments_for_fields(
  fields: &Punctuated<Field, Comma>
) -> Vec<TokenStream2> {
  fields
    .iter()
    .map(|f: &Field| {
      let (field_name, _) = get_name_and_type(f);

      quote! {
        #field_name: self.#field_name
      }
    })
    .collect()
}

fn get_name_and_type(f: &Field) -> (&Option<Ident>, &Type) {
  let field_name: &Option<Ident> = &f.ident;

  let field_type: &Type = &f.ty;

  (field_name, field_type)
}

fn marker_trait_and_structs(
  name: &Ident,
  fields: &Punctuated<Field, Comma>,
) -> TokenStream2 {
  let builder_name: Ident = create_builder_ident(name);

  let structs_and_impls = fields.iter().map(|f: &Field| {
    let field_name: &Ident = &f.ident.clone().unwrap();

    let struct_name: Ident =
      create_field_struct_name(&builder_name, field_name);

    quote! {
      pub struct #struct_name {}

      impl MarkerTraitForBuilder for #struct_name {}
    }
  });

  quote! {
    pub trait MarkerTraitForBuilder {}

    #(#structs_and_impls)*

    pub struct FinalBuilder {}

    impl MarkerTraitForBuilder for FinalBuilder {}
  }
}

// fn optional_default_asserts(
//   fields: &Punctuated<Field, Comma>
// ) -> Vec<TokenStream2> {
//   fields
//     .iter()
//     .map(|f: &Field| {
//       let name: &&Ident = &f.ident.as_ref().unwrap();

//       let ty: &Type = &f.ty;

//       let assertion_ident: Ident = format_ident!("__{}DefaultAssertion", name);

//       quote_spanned! {
//         ty.span() => struct #assertion_ident where #ty: core::default::Default;
//       }
//     })
//     .collect()
// }

fn original_struct_setters(
  fields: &Punctuated<Field, Comma>,
  use_defaults: bool,
) -> Vec<TokenStream2> {
  fields
    .iter()
    .map(|f: &Field| {
      let field_name: &Option<Ident> = &f.ident;

      let field_name_as_string: String =
        field_name.as_ref().unwrap().to_string();

      let handle_type: TokenStream2 = if use_defaults {
        default_fallback()
      } else {
        panic_fallback(field_name_as_string)
      };

      quote! { #field_name: self.#field_name.#handle_type }
    })
    .collect()
}

fn panic_fallback(field_name_as_string: String) -> TokenStream2 {
  quote! {
    expect(concat!("field not set: ", #field_name_as_string))
  }
}

// fn use_defaults(attrs: &[Attribute]) -> bool {
//   attrs.iter().any(|attribute: &Attribute| {
//     attribute.path().is_ident(ATTRIBUTE_NAME_DEFAULTS)
//   })
// }

////////////////////////////////////////////////////////////////////////////////
// unit tests
////////////////////////////////////////////////////////////////////////////////

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
