use std::collections::HashMap;

use common::{Action, Id, Terminal};

mod builder;
mod item;
mod state;
use state::State;

pub struct Table {
    // possible expected tokens given a state and a lookahead
    expected: HashMap<State, HashMap<Terminal, Vec<Terminal>>>,

    // action given a state and id
    action: HashMap<State, HashMap<Id, Action>>,
}