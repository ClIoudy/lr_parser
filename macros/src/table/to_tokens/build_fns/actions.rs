use std::collections::HashMap;

use common::{Action, Id, NonTerminal, StateId, Terminal};
use proc_macro2::TokenStream;
use quote::quote;

use super::super::reprenstations::MapRepr;

pub fn action_fn(actions: HashMap<StateId, HashMap<Id, Action>>) -> TokenStream {
    // match id_match => action

    // let a: Vec<(StateId, _)> = actions
    //     .into_iter()
    //     .map(|(state, v)| (state, MapRepr::new(
    //         v.into_iter().map(|(id, a)| { (id_match(id), a) }),
    //         "token".into(),
    //         Some( quote! { None } )
    //         ))
    //     ).collect();

    // let repr = MapRepr::new(a.into_iter(), "state".into(), Some( quote! {None} ));
    
    // let m = actions.into_iter().map(|(state, v)| (state, MapRepr::new(
    //     v.into_iter().map(|(id, action)| (id_match(id), quote! {Some(#action)})).collect::<Vec<_>>(), 
    //     "token".into(), 
    //     Some(quote! {None})
    // )));

    // let repr = MapRepr::new(m.collect::<Vec<_>>(), "state".into(), Some(quote! {None}));

    // for (state, map) in actions {
    //     for (id, action) in map {
            
    //     }
    // }

    let map = actions.into_iter().map(|(state, v)| (state, MapRepr::new(
            v
                .into_iter()
                .map(|(id, y)| (id_match(id), quote! {Some(#y)}))
                .collect::<Vec<_>>(),
            quote!(token),
            Some(quote! {None})
    ))).collect::<Vec<_>>();

    let repr = MapRepr::new(map, quote!(state), Some(quote! { None }));

    // let m = actions.into_iter().map(|(state, map)| {
    //     let map = map.into_iter().map(|(id, a)| (id_match(id), quote! { Some(#a) }));
        
    //     let inner_repr = MapRepr::new(map, "token".to_string(), Some(quote! {None}));

    //     (state, inner_repr)
    // });

    // let repr = MapRepr::new(m, "state".into(), Some(quote! {None}));

    

    quote! { #repr }
    // quote! { todo!() }
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
