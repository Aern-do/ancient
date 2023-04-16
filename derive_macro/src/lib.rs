mod decode;
mod encode;
mod input;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

#[proc_macro_derive(Decode, attributes(inner, custom))]
pub fn decode(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    decode::expand(derive_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Encode, attributes(inner, custom))]
pub fn encode(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    encode::expand(derive_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
