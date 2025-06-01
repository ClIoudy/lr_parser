use std::fmt::Debug;

use syn::{parse::Parse, Ident, Token};

use super::{rule_element::RuleElement, ParseShortcuts};

#[derive(Debug)]
pub struct Variant {
    name: Ident,
    values: Vec<RuleElement>,
}

impl Parse for Variant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.ident()?;
        input.parse::<Token![->]>()?;
        let values = input.punctuated_vec::<_, Token![,]>()?;
        
        Ok(Self {
            name,
            values,
        })
    }
}