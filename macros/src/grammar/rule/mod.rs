use common::{NonTerminal, Variant};
use syn::{parse::Parse, Ident, Token};

use crate::ParseShortcuts;

mod variant_parse;
use variant_parse::VariantParser;

#[derive(Debug)]
pub struct Rule {
    pub id: NonTerminal,
    pub variant: Variant,
}

impl Rule {
    pub fn symbol(&self) -> &str {
        &self.id.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.id = NonTerminal { symbol }
    }
}


impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        
        let symbol = input.ident()?.to_string();
        input.parse::<Token![:]>()?;
        // let name = input.ident()?;

        // input.parse::<Token![->]>()?;

        let variant_parse: VariantParser = input.parse()?;

        Ok(Self {
            id: NonTerminal::new(symbol),
            variant: variant_parse.0,
        })
    }
}