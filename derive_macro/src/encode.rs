use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Error};

use crate::input::{parse_data_struct, Attribute, Input};

pub(crate) fn expand(derive_input: DeriveInput) -> syn::Result<TokenStream> {
    match derive_input.data {
        Data::Struct(data_struct) => Ok(generate_encode(parse_data_struct(
            derive_input.ident,
            data_struct,
        )?)),
        Data::Enum(_) => Err(Error::new(Span::call_site(), "Enums are not supported")),
        Data::Union(_) => Err(Error::new(Span::call_site(), "Unions are not supported")),
    }
}

fn generate_encode(input: Input) -> TokenStream {
    let fields = input.fields.into_iter().map(|(field, attr)| {
        let field_ident = field.ident;

        match attr {
            Some(attr) => {
                match attr {
                    Attribute::Inner(path) => {
                        let last_segment = path.segments.last().unwrap();
                        let other_segments = path.segments.iter().map(|segment| &segment.ident);

                        if !last_segment.arguments.is_empty() {
                            let generics = &last_segment.arguments;
                            quote! {
                                writer.encode(#(#other_segments)::*::#generics::from(self.#field_ident))?;
                            }
                        } else {
                            quote! {
                                writer.encode(#(#other_segments)::*::from(self.#field_ident))?;
                            }   
                        }
                    },
                    Attribute::Custom(custom_attribute) => {
                        let encode_path = custom_attribute.encode_path;
                        let last_segment = custom_attribute.ty_path.segments.last().unwrap();
                        let other_segments = custom_attribute.ty_path.segments.iter().map(|segment| &segment.ident);

                        if !last_segment.arguments.is_empty() {
                            let generics = &last_segment.arguments;
                            quote! {
                                writer.encode(#(#other_segments)::*::#generics::#encode_path(self.#field_ident))?;
                            }
                        } else {
                            quote! {
                                writer.encode(#(#other_segments)::*::#encode_path(self.#field_ident))?;
                            }
                        }
                    },
                }
            },
            None => quote! {
                writer.encode(self.#field_ident)?;
            },
        }
    });

    let input_ident = input.ident;
    quote! {
        impl crate::protocol::Encode for #input_ident {
            fn encode<W: std::io::Write + crate::protocol::EncodeExt>(self, writer: &mut W) -> Result<(), crate::error::Error> {
                #(#fields)*
                Ok(())
            }
        }
    }
}
