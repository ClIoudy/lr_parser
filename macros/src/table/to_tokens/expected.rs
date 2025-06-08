use std::collections::{HashMap, HashSet};

use common::Terminal;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::reprenstations::{MapRepr, SetRepr};

pub fn expected(expected: HashMap<usize, HashSet<Terminal>>) -> TokenStream {
    let expected: std::collections::HashMap<usize, SetRepr<Terminal>> = expected
        .into_iter()
        .map(|(k, v)| (k, SetRepr(v)))
        .collect();
    
    let expected = MapRepr::new(expected, "state_id".to_string());

    expected.into_token_stream()
}
