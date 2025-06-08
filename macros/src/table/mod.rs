use std::collections::{HashMap, HashSet};

use common::{Action, Id, NonTerminal, StateId, Terminal, Variant};
use proc_macro::TokenStream;
use quote::quote;

use crate::grammar::Grammar;

mod builder;
mod to_tokens;

#[derive(Debug, Clone)]
pub struct TableMacroInfo {
    // possible expected tokens given a state and a lookahead
    expected: HashMap<StateId, HashSet<Terminal>>,

    // action given a state and id
    actions: HashMap<StateId, HashMap<Id, Action>>,

    // rules for build_rules()
    rules: HashMap<NonTerminal, Vec<Variant>>,
}

impl TableMacroInfo {
    pub fn new(expected: HashMap<StateId, HashSet<Terminal>>, actions: HashMap<StateId, HashMap<Id, Action>>, rules: HashMap<NonTerminal, Vec<Variant>>) -> Self {
        Self {
            expected,
            actions,
            rules
        }
    }
}


pub fn table(grammar: Grammar) -> TokenStream {

    // let build_rule: TokenStream build_rule::build_rules_function(&grammar);



    // let expanded = quote! {
    //     struct Table {

    //     }

    //     impl TableTrait for Table {
    //         fn start_state(&self) -> &State {

    //         }

    //         fn action(&self, state: &State, token: &Id) -> Option<Action> {

    //         }
        
    //         fn is_end_state(&self, state: &State) -> bool {

    //         }
        
    //         fn build_rule(&self, children: Vec<Box<dyn Any>>, variat: Variant) -> Box<dyn RuleTrait> {
    //             #build_rule
    //         }
    //     }
    // };

    todo!()
}