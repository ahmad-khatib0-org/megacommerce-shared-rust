use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Path, Token};

#[proc_macro]
pub fn sanitize_app_error(input: TokenStream) -> TokenStream {
  // Parse: a list of paths separated by commas
  let args = parse_macro_input!(input with Punctuated::<Path, Token![,]>::parse_terminated);

  if args.len() != 2 {
    return syn::Error::new_spanned(args, "expected 2 arguments: (ResponseType, ResponseEnumPath)")
      .to_compile_error()
      .into();
  }

  let ty = &args[0]; // e.g. EmailConfirmationResponse
  let enum_ty = &args[1]; // e.g. email_confirmation_response::Response

  let expanded = quote! {
      impl SanitizeAppError for #ty {
          fn sanitize_app_error_fields(&mut self) {
              if let Some(#enum_ty::Error(err)) = self.response.take() {
                  self.response = Some(#enum_ty::Error(sanitize_app_error(&err)));
              }
          }
      }
  };

  expanded.into()
}
