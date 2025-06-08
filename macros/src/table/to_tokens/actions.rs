use std::collections::HashMap;

use common::{Action, Id, StateId};
use proc_macro2::TokenStream;
use quote::quote;

use super::reprenstations::MapRepr;

pub fn actions_tokens(actions: HashMap<StateId, HashMap<Id, Action>>) -> TokenStream {

    // actions.into_iter().map(|(k, v)| {
    //     (k, v.into_iter().map(|(id, a)| {}))
    // })
    
    let map = actions.into_iter().map(|(k, v)| {
        (k, MapRepr::new(v, "id".to_string()))
    }).collect();

    let repr = MapRepr::new(map, "state_id".to_string());

    quote! { #repr } 
}