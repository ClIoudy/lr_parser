use std::collections::HashSet;

use crate::table_builder::table::item::StateItem;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct State {
    items: Vec<StateItem>
}

impl State {
    pub fn new(items: HashSet<StateItem>) -> Self {
        Self {
            items: items.into_iter().collect()
        }
    }
    
    pub fn items(&self) -> &Vec<StateItem> {
        &self.items
    }
}