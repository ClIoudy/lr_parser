use syn::{parse::Parse, Token};
use common::{Id, NonTerminal, Variant, VariantId};

// mod variant_parser;
// use variant_parser::VariantParser;

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


// pub struct Rule {
//     pub symbol: NonTerminal,
//     pub variant: Variant,
// }

// impl Parse for Rule {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let symbol = input.ident()?;
//         input.parse::<Token![:]>()?;
//         let variant = input.parse::<VariantParser>()?.0;

//         Ok(Self {
//             symbol: NonTerminal::new(symbol.to_string()),
//             variant,
//         })
//     }
// }