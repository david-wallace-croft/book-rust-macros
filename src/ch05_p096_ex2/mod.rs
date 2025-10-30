use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
use ::syn::punctuated::Punctuated;
use ::syn::*;

pub fn extract_field_names_as_tokens(ast: &DeriveInput) -> Vec<TokenStream2> {
  let named_fields: &Punctuated<Field, token::Comma> = match &ast.data {
    Data::Struct(data_struct) => {
      let fields: &Fields = &data_struct.fields;

      match fields {
        Fields::Named(fields_named) => &fields_named.named,
        Fields::Unnamed(_fields_unnamed) => unimplemented!(),
        Fields::Unit => unimplemented!(),
      }
    },
    Data::Enum(_data_enum) => unimplemented!(),
    Data::Union(_data_union) => unimplemented!(),
  };

  named_fields
    .iter()
    .map(|field| {
      let field_ident: &Option<Ident> = &field.ident;

      let field_name_option: Option<&Ident> = field_ident.as_ref();

      let field_name: &Ident = field_name_option.unwrap();

      quote! { #field_name }
    })
    .collect()
}

pub fn generated_fields(ast: &DeriveInput) -> Vec<TokenStream2> {
  let named_fields: &Punctuated<Field, token::Comma> = match &ast.data {
    Data::Struct(data_struct) => {
      let fields: &Fields = &data_struct.fields;

      match fields {
        Fields::Named(fields_named) => &fields_named.named,
        Fields::Unnamed(_fields_unnamed) => unimplemented!(),
        Fields::Unit => unimplemented!(),
      }
    },
    Data::Enum(_data_enum) => unimplemented!(),
    Data::Union(_data_union) => unimplemented!(),
  };

  named_fields
    .iter()
    .map(|field| {
      let field_name: &Ident = field.ident.as_ref().unwrap();

      let type_name: &Type = &field.ty;

      quote! {
        #field_name: #type_name
      }
    })
    .collect()
}
