use syn::{parse::Parse, Token};
use common::{Id, Variant, VariantId};

use crate::{grammar::IdParse, ParseShortcuts};

pub (super) struct VariantParser(pub Variant);

impl Parse for VariantParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let symbol = input.ident()?.to_string();

        input.parse::<Token![:]>()?;

        let name = input.ident()?.to_string();
        
        input.parse::<Token![->]>()?;

        let values: Vec<Id> = input
            .punctuated_vec::<IdParse, Token![,]>()?
            .into_iter()
            .map(|x| x.0)
            .collect();
        
        // let id = VariantId::new(NonTerminal::new(name), values.len());
        let id = VariantId::new(symbol, name, values.len());
        let variant = Variant::new(values, id);
        Ok(Self(variant))
    }
}