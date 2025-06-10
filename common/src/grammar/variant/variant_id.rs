use crate::NonTerminal;
use quote::{ToTokens, quote};


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariantId {
    symbol: NonTerminal,
    name: String,
    length: usize,
}

impl VariantId {
    pub fn new(symbol: String, name: String, length: usize) -> Self {
        Self {
            symbol: NonTerminal::new(symbol),
            name: name,
            length,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn symbol(&self) -> &NonTerminal {
        &self.symbol
    }
}

impl ToTokens for VariantId {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let symbol = &self.symbol.x;
        let name = &self.name;
        let length = self.length;

        tokens.extend(quote! {
            lr_parser::VariantId::new(#symbol.to_string(), #name.to_string(), #length)     
        });
    }
}

