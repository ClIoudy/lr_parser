use std::collections::{HashMap, HashSet};

use common::{Action, Id, NonTerminal, StateId, TableTrait, Terminal, Variant, VariantId};
use proc_macro2::TokenStream;
use quote::quote;

use crate::grammar::Grammar;

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

    // rules for build_rules()
    rules: HashMap<NonTerminal, Vec<VariantId>>,
}

impl TableMacroInfo {
    pub fn new(expected: HashMap<StateId, HashSet<Terminal>>, actions: HashMap<StateId, HashMap<Id, Action>>, rules: HashMap<NonTerminal, Vec<VariantId>>) -> Self {
        Self {
            expected,
            actions,
            rules
        }
    }
}


pub fn table(grammar: &Grammar) -> TokenStream {

    // let build_rule: TokenStream build_rule::build_rules_function(&grammar);
    let table = TableBuilder::new(grammar).build();

    let build_rule_fn = build_rule_fn(grammar.all_rules());
    let start_state = 0usize;
    let expected_fn = expected_fn(table.expected);
    let action_fn = action_fn(table.actions);


    quote! {
        struct Table;

        impl lr_parser::TableTrait for Table {
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
        }
    }
}