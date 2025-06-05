use std::collections::HashMap;

use common::{Action, Id, StateId, Terminal};
use proc_macro::TokenStream;
use quote::quote;

use crate::grammar::Grammar;

mod builder;

pub struct Table {
    // possible expected tokens given a state and a lookahead
    expected: HashMap<StateId, HashMap<Id, Vec<Terminal>>>,

    // action given a state and id
    actions: HashMap<StateId, HashMap<Id, Action>>,
}

impl Table {
    pub fn new(expected: HashMap<StateId, HashMap<Id, Vec<Terminal>>>, actions: HashMap<StateId, HashMap<Id, Action>>) -> Self {
        Self {
            expected,
            actions
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