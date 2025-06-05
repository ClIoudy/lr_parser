use std::collections::HashSet;

use super::StateItem;

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