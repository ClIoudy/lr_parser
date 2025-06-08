use std::fmt::Debug;

use quote::{ToTokens, quote};

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Terminal {
    EOF,
    Labeld(String),
}

impl PartialEq<str> for Terminal {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::EOF => false,
            Self::Labeld(x) => x.as_str() == other
        }
    }
}



impl From<&str> for Terminal {
    fn from(value: &str) -> Self {
        Self::Labeld(value.to_string())
    }
}

impl From<String> for Terminal {
    fn from(value: String) -> Self {
        Self::Labeld(value)
    }
}

impl Debug for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Labeld(x) => f.write_fmt(format_args!("\"{}\"", x)),
            Self::EOF => f.write_str("$"),
        }
    }
}

impl ToTokens for Terminal {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let t = match &self {
            Self::EOF => quote! { Terminal::EOF },
            Self::Labeld(x) => quote! { Terminal::labeld(#x.to_string()) }, 
        };

        tokens.extend(t);
    }
}
