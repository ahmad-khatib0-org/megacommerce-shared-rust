use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SanitizeAppError)]
pub fn sanitize_app_error(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = input.ident;

  // By convention we assume each response has a `response` oneof with an Error variant.
  // Prost generates an enum called `<lowercase_struct_name>::Response`
  // where one variant is `Error(AppError)`.
  let mod_name =
    syn::Ident::new(&format!("{}::response", name.to_string().to_lowercase()), name.span());
  let enum_name = syn::Ident::new("Response", name.span());

  let expanded = quote! {
      impl SanitizeAppError for #name {
          fn sanitize_app_error_fields(&mut self) {
              if let Some(#mod_name::#enum_name::Error(err)) = self.response.take() {
                  self.response =
                      Some(#mod_name::#enum_name::Error(sanitize_app_error(&err)));
              }
          }
      }
  };

  TokenStream::from(expanded)
}
