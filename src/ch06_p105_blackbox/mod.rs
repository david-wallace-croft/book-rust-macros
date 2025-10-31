use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::syn::{
  Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident,
  punctuated::Punctuated, token::Comma,
};
use syn::Type;

#[allow(dead_code)]
pub fn create_builder_blackbox(item: TokenStream) -> TokenStream {
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

  let builder_fields = fields.iter().map(|f: &Field| {
    let field_name: &Option<Ident> = &f.ident;

    let field_type: &Type = &f.ty;

    quote! { #field_name: Option<#field_type> }
  });

  let builder_inits = fields.iter().map(|f: &Field| {
    let field_name: &Option<Ident> = &f.ident;

    quote! { #field_name: None }
  });

  let builder_methods = fields.iter().map(|f: &Field| {
    let field_name: &Option<Ident> = &f.ident;

    let field_type: &Type = &f.ty;

    quote! {
      pub fn #field_name(&mut self, input: #field_type) -> &mut Self {
        self.#field_name = Some(input);

        self
      }
    }
  });

  let set_fields = fields.iter().map(|f: &Field| {
    let field_name: &Option<Ident> = &f.ident;

    let field_name_as_string: String = field_name.as_ref().unwrap().to_string();

    quote! {
      #field_name: self.#field_name.as_ref()
        .expect(&format!("field {} not set", #field_name_as_string))
        .to_string()
    }
  });

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
