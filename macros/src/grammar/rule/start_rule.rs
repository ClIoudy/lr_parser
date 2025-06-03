use crate::{grammar::rule::VariantParser, ParseShortcuts};
use common::{NonTerminal, Variant};
use syn::{parse::Parse, Token};

pub struct StartRule(pub Vec<Variant>);

impl Parse for StartRule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        
        input.parse::<Token![#]>()?;

        let variant = input.parse::<VariantParser>()?.0;
        input.parse::<Token![;]>()?;
        let mut res = vec![variant];
        
        while input.peek(Token![#]) {
            input.parse::<Token![#]>()?;

            let variant = input.parse::<VariantParser>()?.0;
            input.parse::<Token![;]>()?;
            res.push(variant);
        }

        Ok(Self(res))
    }
}