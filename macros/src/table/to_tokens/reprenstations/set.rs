use std::{collections::HashSet, fmt::Debug, hash::Hash};

use quote::{quote, ToTokens};

#[derive(Debug, PartialEq, Eq, Clone)]
/// Represents a set by constructing it anew from an iterator.
/// This means that quote! { let x = #setRepr } will simply result in x being equal to the set being represented. 
pub struct SetRepr<'a, T: Eq + Hash + Debug>(pub &'a HashSet<T>);

impl<'a, T: ToTokens + Eq + Hash + Debug> ToTokens for SetRepr<'a, T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let set = self.0.iter();

        let e = quote! {
            std::collections::HashSet::from_iter([#(#set),*].into_iter())
        };

        tokens.extend(e);
    }
}