pub(crate) mod input;

use input::{Inner, Input};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, parse_macro_input, Data, DeriveInput, Path};

#[proc_macro_derive(Decode, attributes(inner))]
pub fn decode(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let input = generate_input(input);
    let generated = generate_decode(input);
    proc_macro::TokenStream::from(generated)
}
#[proc_macro_derive(Encode, attributes(inner))]
pub fn encode(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let input = generate_input(input);
    let generated = generate_encode(input);
    proc_macro::TokenStream::from(generated)
}
pub(crate) fn generate_encode(input: Input) -> TokenStream {
    let mut fields = vec![];
    for (field, inner) in input.fields {
        fields.push(match inner {
            Some(inner) => {
                let mut path = inner.path.to_token_stream().into_iter();
                let ident = path.next().unwrap();
                let path = path.collect::<TokenStream>();
                let name = field.ident.unwrap();
                // Rewrite this code
                if [String::from("VarInt"), String::from("VarLong")].contains(&ident.to_string()) {
                    quote! {
                        writer.encode(#ident::from(self.#name))?;
                    }
                } else {
                    quote! {
                        writer.encode(#ident::#path::from(self.#name))?;
                    }
                }
            }
            None => {
                let name = field.ident.unwrap();
                quote! {
                    writer.encode(self.#name)?;
                }
            }
        })
    }
    let name = input.ident;
    quote! {
        impl crate::protocol::Encode for #name {
            fn encode<W: std::io::Write + crate::protocol::EncodeExt>(self, writer: &mut W) -> Result<(), crate::error::Error> {
                #(#fields)*
                Ok(())
            }
        }
    }
}
pub(crate) fn generate_decode(input: Input) -> TokenStream {
    let mut fields = vec![];
    for (field, inner) in input.fields {
        fields.push(match inner {
            Some(inner) => {
                let name = field.ident;
                let field_type = field.ty;
                let Path { segments, .. } = inner.path;
                let last_arguments = &segments.last().unwrap().arguments;
                let all_idents = segments.iter().map(|ident| &ident.ident);
                quote! {
                    #name: <#field_type>::from(reader.decode::<#(#all_idents)::*#last_arguments>()?)
                }
            }
            None => {
                let name = field.ident.unwrap();
                quote! {
                    #name: reader.decode()?
                }
            }
        })
    }
    let ident = input.ident;
    quote! {
        impl crate::protocol::Decode for #ident {
            fn decode<R: std::io::Read + crate::protocol::DecodeExt>(reader: &mut R) -> Result<Self, crate::error::Error> {
                Ok(Self {
                    #(#fields),*
                })
            }
        }
    }
}
pub(crate) fn generate_input(derive_input: DeriveInput) -> Input {
    let mut input = Input::new(derive_input.ident);
    match derive_input.data {
        Data::Struct(data_struct) => {
            for field in data_struct.fields {
                match field
                    .attrs
                    .iter()
                    .find(|attribute| attribute.path.is_ident("inner"))
                {
                    Some(attribute) => {
                        let attribute = parse2::<Inner>(attribute.tokens.to_owned()).unwrap();
                        input.fields.push((field, Some(attribute)))
                    }
                    None => input.fields.push((field, None)),
                };
            }
            input
        }
        _ => todo!(),
    }
}
