use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Field, Path, Result, Ident
};

pub(crate) struct Input {
    pub(crate) ident: Ident,
    pub(crate) fields: Vec<(Field, Option<Inner>)>,
}

impl Input {
    pub(crate) fn new(ident: Ident) -> Self {
        Self { ident, fields: Default::default() }
    }
}
pub(crate) struct Inner {
    _paren_token: Paren,
    pub(crate) path: Path,
}
impl Parse for Inner {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _paren_token: parenthesized!(content in input),
            path: content.parse()?,
        })
    }
}
