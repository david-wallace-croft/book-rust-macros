use ::proc_macro2::Span;
use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
use ::syn::punctuated::Punctuated;
use ::syn::*;

pub fn generated_methods(ast: &DeriveInput) -> Vec<TokenStream2> {
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

      let method_name_string: String = format!("get_{field_name}");

      let method_name: Ident =
        Ident::new(&method_name_string, Span::call_site());

      quote! {
          fn #method_name(&self) -> &#type_name {
              &self.#field_name
          }
      }
    })
    .collect()
}
