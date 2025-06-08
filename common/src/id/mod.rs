use std::fmt::Debug;

mod non_terminal;
pub use non_terminal::NonTerminal;

mod terminal;
pub use terminal::Terminal;

use quote::{quote, ToTokens};
use proc_macro2::TokenStream;


#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Id {
    N(NonTerminal),
    T(Terminal),
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Id::T(x) => x.fmt(f),
            Id::N(x) => x.fmt(f),
        }
    }
}

impl ToTokens for Id {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::T(t) => quote! { Id::T(#t) },
            Self::N(n) => quote! { Id::N(#n) },
        });
    }
}