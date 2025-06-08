use std::{collections::{HashMap, HashSet}, hash::Hash};
use common::{Action, Id, NonTerminal, Terminal, VariantId};
use quote::{quote, ToTokens};

use super::super::reprenstations::*;



#[test]
fn map() {
    let map = [(0, "0")].into_iter().collect::<HashMap<_, _>>();
    let map = MapRepr::new(map, "abc".to_string());
    let repr = map.into_token_stream();

    let quoted = quote! {
        match abc {
            0i32 => "0"
        }
    };

    assert_eq!(repr.to_string(), quoted.to_string());
}

#[test]
fn set() {
    use crate::set;

    let set = set!(0);
    let repr = SetRepr(set).into_token_stream(); 

    let quoted = quote! {
        std::collections::HashSet::from_iter(vec![0i32].into_iter())
    };

    assert_eq!(quoted.to_string(), repr.to_string())
}

#[test]
fn id() {
    let id = Id::T(Terminal::EOF);
    let repr = id.into_token_stream();

    let quoted = quote! {
        Id::T(Terminal::EOF)
    };

    assert_eq!(quoted.to_string(), repr.to_string());
}

#[test]
fn action() {
    let reduce = Action::Reduce(VariantId::new("S".to_string(), "A".to_string(), 2));

    let quoted_reduce = quote! {
        Action::Reduce(VariantId::new("S".to_string(), "A".to_string(), 2usize))
    };

    assert_eq!(reduce.into_token_stream().to_string(), quoted_reduce.to_string());

    let shift = Action::Shift(2);

    let quoted_shift = quote! {
        Action::Shift(2usize)
    };

    assert_eq!(shift.into_token_stream().to_string(), quoted_shift.to_string());


}