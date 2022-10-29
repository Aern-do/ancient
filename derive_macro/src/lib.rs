pub(crate) mod input;

use input::{Inner, Input};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Readable, attributes(inner))]
pub fn readable(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let input = generate_input(input);
    let generated = generate_readable(input);
    proc_macro::TokenStream::from(generated)
}
#[proc_macro_derive(Writeable, attributes(inner))]
pub fn writeable(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let input = generate_input(input);
    let generated = generate_writeable(input);
    proc_macro::TokenStream::from(generated)
}
pub(crate) fn generate_writeable(input: Input) -> TokenStream {
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
                        writer.writeable(#ident::from(self.#name))?;
                    }
                } else {
                    quote! {
                        writer.writeable(#ident::#path::from(self.#name))?;
                    }
                }
            }
            None => {
                let name = field.ident.unwrap();
                quote! {
                    writer.writeable(self.#name)?;
                }
            }
        })
    }
    let name = input.ident;
    quote! {
        impl crate::protocol::Writeable for #name {
            fn write<W: std::io::Write + crate::protocol::WriteExt>(self, writer: &mut W) -> Result<(), crate::error::Error> {
                #(#fields)*
                Ok(())
            }
        }
    }
}
pub(crate) fn generate_readable(input: Input) -> TokenStream {
    let mut fields = vec![];
    for (field, inner) in input.fields {
        fields.push(match inner {
            Some(inner) => {
                let path = inner.path;
                let ident = field.ident.unwrap();
                let ident_target = field.ty.to_token_stream().into_iter().next().unwrap();
                quote! {
                    #ident: #ident_target::from(reader.readable::<#path>()?)
                }
            }
            None => {
                let name = field.ident.unwrap();
                quote! {
                    #name: reader.readable()?
                }
            }
        })
    }
    let ident = input.ident;
    quote! {
        impl crate::protocol::Readable for #ident {
            fn read<R: std::io::Read + crate::protocol::ReadExt>(reader: &mut R) -> Result<Self, crate::error::Error> {
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
