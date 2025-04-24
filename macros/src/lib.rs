use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod attrs;
mod expand;

#[proc_macro_derive(ErrorResponse, attributes(response))]
pub fn derive_error_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand::expand(input) {
        Ok(stream) => stream,
        Err(error) => error.to_compile_error().into(),
    }
}
