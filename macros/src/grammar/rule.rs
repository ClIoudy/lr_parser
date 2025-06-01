use syn::{parse::Parse, Ident, Token};

use super::{variant::Variant, ParseShortcuts};

#[derive(Debug)]
pub struct Rule {
    pub symbol: String,
    pub variant: Variant,
}


impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        
        let symbol = input.ident()?.to_string();
        input.parse::<Token![:]>()?;
        // let name = input.ident()?;

        // input.parse::<Token![->]>()?;

        let variant = input.parse()?;

        Ok(Self {
            symbol,
            variant,
        })
    }
}