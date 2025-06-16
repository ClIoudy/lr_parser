use std::fmt::Debug;

use quote::{quote, ToTokens};

/// Representation of a non-terminal gramamr symbol.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonTerminal {
    pub x: String,
}

impl Debug for NonTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.x)
    }
}

impl NonTerminal {
    pub fn new(x: String) -> Self {
        Self { x }
    }
    
    pub fn start_symbol() -> NonTerminal {
        NonTerminal::new("S".to_string())
    }
}

impl From<&str> for NonTerminal {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

impl From<String> for NonTerminal {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl PartialEq<str> for NonTerminal {
    fn eq(&self, other: &str) -> bool {
        &self.x == other
    }
}


impl ToTokens for NonTerminal {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let x = &self.x;
        tokens.extend(quote! { NonTerminal::new(#x.to_string()) });
    }
}