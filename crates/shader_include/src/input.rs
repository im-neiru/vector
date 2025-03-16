use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, LitStr, Token};

pub(crate) struct IncludeSpirvInput {
    pub(crate) id: Expr,
    pub(crate) _comma: Token![,],
    pub(crate) path: LitStr,
}

impl Parse for IncludeSpirvInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(IncludeSpirvInput {
            id: input.parse()?,
            _comma: input.parse()?,
            path: input.parse()?,
        })
    }
}
