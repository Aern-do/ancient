
use indexmap::IndexMap;
use proc_macro2::Ident;
use syn::{
    parse::{Parse, ParseStream},
    DataStruct, Field, Path, Token,
};

mod kw {
    use syn::custom_keyword;
    custom_keyword!(ty);
    custom_keyword!(decode);
    custom_keyword!(encode);
}

#[derive(Clone, Debug)]
pub(crate) struct CustomAttribute {
    _ty_kw: kw::ty,
    _ty_colon: Token![:],
    pub(crate) ty_path: Path,
    _ty_close: Token![;],
    _decode_kw: kw::decode,
    _decode_colon: Token![:],
    pub(crate) decode_path: Path,
    _comma: Token![,],
    _encode_kw: kw::encode,
    _encode_colon: Token![:],

    pub(crate) encode_path: Path,
}
impl Parse for CustomAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _ty_kw: input.parse()?,
            _ty_colon: input.parse()?,
            ty_path: input.parse()?,
            _ty_close: input.parse()?,
            _decode_kw: input.parse()?,
            _decode_colon: input.parse()?,
            decode_path: input.parse()?,
            _comma: input.parse()?,
            _encode_kw: input.parse()?,
            _encode_colon: input.parse()?,
            encode_path: input.parse()?,
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Attribute {
    Inner(Path),
    Custom(CustomAttribute),
}

#[derive(Debug, Clone)]
pub(crate) struct Input {
    pub(crate) ident: Ident,
    pub(crate) fields: IndexMap<Field, Option<Attribute>>,
}

impl Input {
    pub(crate) fn new(ident: Ident) -> Self {
        Self {
            ident,
            fields: Default::default(),
        }
    }

    pub(crate) fn add_field(&mut self, field: Field, attribute: Option<Attribute>) {
        self.fields.insert(field, attribute);
    }
}

pub(crate) fn parse_data_struct(ident: Ident, data_struct: DataStruct) -> syn::Result<Input> {
    let mut input = Input::new(ident);

    for field in data_struct.fields {
        let attribute = field
            .attrs
            .iter()
            .find_map(|attribute| {
                if attribute.path.is_ident("inner") {
                    return Some(attribute.parse_args().map(Attribute::Inner));
                }
                if attribute.path.is_ident("custom") {
                    return Some(attribute.parse_args().map(Attribute::Custom));
                }
                None
            })
            .transpose()?;
        input.add_field(field, attribute);
    }
    Ok(input)
}
