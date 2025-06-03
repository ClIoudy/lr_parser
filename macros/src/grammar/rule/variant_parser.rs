use std::fmt::Debug;

use common::{Id, NonTerminal, Variant, VariantId};
use syn::{parse::Parse, Ident, Token};

use crate::{grammar::IdParse, ParseShortcuts};

pub (super) struct VariantParser(pub Variant);

impl Parse for VariantParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.ident()?.to_string();
        
        input.parse::<Token![->]>()?;
        let values = input.punctuated_vec::<IdParse, Token![,]>()?;
        let values: Vec<Id> = values.into_iter().map(|x| x.0).collect();
        
        // let id = VariantId::new(NonTerminal::new(name), values.len());
        let id = VariantId::new(name, values.len());
        let variant = Variant::new(values, id);
        Ok(Self(variant))
    }
}