use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Error};

use crate::input::{parse_data_struct, Attribute, Input};

pub(crate) fn expand(derive_input: DeriveInput) -> syn::Result<TokenStream> {
    match derive_input.data {
        Data::Struct(data_struct) => Ok(generate_decode(parse_data_struct(
            derive_input.ident,
            data_struct,
        )?)),
        Data::Enum(_) => Err(Error::new(Span::call_site(), "Enums are not supported")),
        Data::Union(_) => Err(Error::new(Span::call_site(), "Unions are not supported")),
    }
}

fn generate_decode(input: Input) -> TokenStream {
    let fields = input.fields.into_iter().map(|(field, attr)| {
        let field_ident = field.ident;
        let field_ty = field.ty;

        match attr {
            Some(attr) => match attr {
                Attribute::Inner(path) => {
                    let generics = &path.segments.last().unwrap().arguments;
                    let ty_segments = path.segments.iter().map(|segment| &segment.ident);
                    
                    quote! {
                        #field_ident: <#field_ty>::from(reader.decode::<#(#ty_segments)::*#generics>()?)
                    }
                },
                Attribute::Custom(custom_attr) => {
                    let decode_path = custom_attr.decode_path;
                    let inner_ty = custom_attr.ty_path;

                    quote! {
                        #field_ident: reader.decode::<#inner_ty>()?.#decode_path()
                    }
                },
            },
            None => quote! {
                #field_ident: reader.decode()?
            },
        }
    });

    let input_ident = input.ident;
    quote! {
        impl crate::protocol::Decode for #input_ident {
            fn decode<R: std::io::Read + crate::protocol::DecodeExt>(reader: &mut R) -> Result<Self, crate::error::Error> {
                Ok(Self {
                    #(#fields),*
                })
            }
        }
    }
}
