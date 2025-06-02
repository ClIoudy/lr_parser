use std::fmt::Debug;

use common::{Id, Variant};
use syn::{parse::Parse, Ident, Token};

use crate::{grammar::IdParse, ParseShortcuts};

pub (super) struct VariantParser(pub Variant);

impl Parse for VariantParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.ident()?;
        
        input.parse::<Token![->]>()?;
        let values = input.punctuated_vec::<IdParse, Token![,]>()?;
        let values = values.into_iter().map(|x| x.0).collect();
        
        let variant = Variant::new(name.to_string(), values);
        Ok(Self(variant))
    }
}