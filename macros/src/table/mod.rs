use std::collections::{HashMap, HashSet};

use common::{Action, Id, StateId, Terminal};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{grammar::Grammar, table::to_tokens::reprenstations::SetRepr};

mod builder;
use builder::TableBuilder;
mod to_tokens;
use to_tokens::build_fns::*;

#[derive(Debug, Clone)]
pub struct TableMacroInfo {
    // possible expected tokens given a state and a lookahead
    expected: HashMap<StateId, HashSet<Terminal>>,

    // action given a state and id
    actions: HashMap<StateId, HashMap<Id, Action>>,
}

impl TableMacroInfo {
    pub fn new(expected: HashMap<StateId, HashSet<Terminal>>, actions: HashMap<StateId, HashMap<Id, Action>>) -> Self {
        Self {
            expected,
            actions,
        }
    }
}


pub fn table(grammar: &Grammar) -> TokenStream {
    let table = TableBuilder::new(grammar).build();
    
    let start_state = 0usize;
    let build_rule_fn = build_rule_fn(grammar.all_rules());
    let expected_fn = expected_fn(&table.expected);
    let action_fn = action_fn(table.actions);

    let alphabet = table.expected
        .into_iter()
        .fold(HashSet::new(), |mut acc, (_, set)| {
            acc.extend(set);
            acc
        })
        .into_iter()
        .filter_map(|x| match x {
            Terminal::EOF => None,
            Terminal::Labeld(label) => Some(label)
        })
        .collect();

    let alphabet_repr = SetRepr(&alphabet);

    quote! {
        use lr_parser::TableTrait;

        struct Table;

        impl TableTrait for Table {
            type StartSymbol = S;
            fn start_state() -> usize {
                #start_state
            }

            fn action(state: &usize, token: &lr_parser::Id) -> Option<lr_parser::Action> {
                #action_fn
            }

            fn expected(state: &usize) -> Option<std::collections::HashSet<lr_parser::Terminal>> {
                #expected_fn
            }
        
            fn build_rule(variant: lr_parser::VariantId, mut children: Vec<Box<dyn std::any::Any>>) -> Option<Box<dyn std::any::Any>> {
                #build_rule_fn
            }

            fn alphabet() -> std::collections::HashSet<&'static str> {
                #alphabet_repr
            }
        }
    }
}