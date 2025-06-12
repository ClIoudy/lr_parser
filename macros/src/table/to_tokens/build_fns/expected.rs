use std::collections::{HashMap, HashSet};

use common::Terminal;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::super::reprenstations::{MapRepr, SetRepr};

pub fn expected_fn(expected: &HashMap<usize, HashSet<Terminal>>) -> TokenStream {    
    let expected: std::collections::HashMap<&usize, TokenStream> = expected
        .into_iter()
        .map(|(k, v)| {
            let set = SetRepr(v);
            (k, quote! { Some(#set) })
        })
        .collect();

    let expected = MapRepr::new(expected.iter(), quote! { state }, Some(quote! {None}));

    expected.into_token_stream()
}
