use std::collections::HashMap;

use crate::grammar::Grammar;

pub struct TableBuilder {
    grammar: Grammar,
    // closures: HashMap<>
}

impl TableBuilder {
    pub fn new(grammar: Grammar) -> Self {
        Self {
            grammar,
            // closures: HashMap<>,
            // follows: HashMap<>,
            // actions: HashMap<>
        }
    }
}