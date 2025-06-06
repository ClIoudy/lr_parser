use std::collections::HashSet;

use super::StateItem;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct State {
    items: Vec<StateItem>
}

impl State {
    pub fn new(items: HashSet<StateItem>) -> Self {
        let mut items: Vec<StateItem> = items.into_iter().collect();
        items.sort();
        
        Self {
            items,
        }
    }
    
    pub fn items(&self) -> &Vec<StateItem> {
        &self.items
    }
}