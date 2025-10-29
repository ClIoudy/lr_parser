use std::collections::HashMap;

use common::{Action, Id, StateId, Terminal};
use proc_macro2::TokenStream;
use quote::quote;

use super::super::reprenstations::MapRepr;

pub fn action_fn(actions: HashMap<StateId, HashMap<Id, Action>>) -> TokenStream {
    let map = actions.into_iter().map(|(state, v)| (state, MapRepr::new(
            v
                .into_iter()
                .map(|(id, y)| (id_match(id), quote! {Some(#y)}))
                .collect::<Vec<_>>(),
            quote!(token),
            Some(quote! {None})
    ))).collect::<Vec<_>>();

    let repr = MapRepr::new(map, quote!(state), Some(quote! { None }));

    quote! { #repr }
}

fn id_match(id: Id) -> TokenStream {
    match id {
        Id::N(n) => {
            let x = n.x;
            quote! { lr_parser::Id::N(nt) if nt.x == #x}
        }
        Id::T(Terminal::EOF) => quote! { #id },
        Id::T(Terminal::Labeld(label)) => quote! {
            lr_parser::Id::T(lr_parser::Terminal::Labeld(label)) if label == #label 
        },
    }
}
