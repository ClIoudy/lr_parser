use crate::NonTerminal;
use quote::{ToTokens, quote};


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariantId {
    symbol: NonTerminal,
    name: NonTerminal,
    length: usize,
}

impl VariantId {
    pub fn new(symbol: String, name: String, length: usize) -> Self {
        Self {
            symbol: NonTerminal::new(symbol),
            name: NonTerminal::new(name),
            length,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }
    
    pub fn name(&self) -> &NonTerminal {
        &self.name
    }

    pub fn symbol(&self) -> &NonTerminal {
        &self.symbol
    }
}

impl ToTokens for VariantId {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let symbol = &self.symbol.x;
        let name = &self.name.x;
        let length = self.length;

        tokens.extend(quote! {
            VariantId::new(#symbol.to_string(), #name.to_string(), #length)     
        });
    }
}

