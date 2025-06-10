use std::fmt::Debug;

use crate::StateId;
use crate::VariantId;
use quote::{ToTokens, quote};

#[derive(Clone, PartialEq, Eq)]
pub enum Action {
    Shift(StateId),
    Reduce(VariantId)
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shift(x) => f.write_fmt(format_args!("Shift({})", x)),
            Self::Reduce(x) => f.write_fmt(format_args!("Reduce({:?}: {:?})", x.symbol(), x.name())),
        }
    }
}

impl ToTokens for Action {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let e = match self {
            Action::Reduce(x) => quote! { lr_parser::Action::Reduce(#x) },
            Action::Shift(x) => quote! { lr_parser::Action::Shift(#x) },
        };

        tokens.extend(e);
    }
}