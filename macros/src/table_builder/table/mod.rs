use std::collections::HashMap;

use common::{Action, Id, Terminal};

mod builder;
mod item;
mod state;
use state::State;

#[cfg(test)]
mod tests;

pub struct Table {
    // possible expected tokens given a state and a lookahead
    expected: HashMap<State, HashMap<Terminal, Vec<Terminal>>>,

    // action given a state and id
    actions: HashMap<State, HashMap<Id, Action>>,
}

impl Table {
    pub fn new(expected: HashMap<State, HashMap<Terminal, Vec<Terminal>>>, actions: HashMap<State, HashMap<Id, Action>>) -> Self {
        Self {
            expected,
            actions
        }
    }
}