use std::{collections::HashSet, fmt::Debug, hash::Hash};

use quote::{quote, ToTokens};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SetRepr<T: Eq + Hash + Debug>(pub HashSet<T>);

impl<T: ToTokens + Eq + Hash + Debug> ToTokens for SetRepr<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let set = self.0.iter();

        let e = quote! {
            std::collections::HashSet::from_iter([#(#set),*].into_iter())
        };

        tokens.extend(e);
    }
}