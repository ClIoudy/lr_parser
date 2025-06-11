use std::fmt::Debug;

use common::{Id, NonTerminal, Variant};

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateItem {
    position: usize,
    variant: Variant,
}

impl StateItem {
    pub fn new(variant: Variant) -> Self {
        Self {
            position: 0,
            variant,
        }
    }

    pub fn symbol(&self) -> &NonTerminal {
        self.variant.symbol()
    }

    pub fn get(&self) -> Option<&Id> {
        self.variant.values().get(self.position)
    }

    pub fn variant(&self) -> &Variant {
        &self.variant
    }

    pub fn advance(&self) -> Self {       
        Self {
            variant: self.variant.clone(),
            position: self.position + 1,
        }
    }
}

impl Debug for StateItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (before_dot, after_dot) = self.variant.values().split_at(self.position);
        f.write_str(&format!("{:?} -> {:?}.{:?}", self.symbol(), before_dot, after_dot))
    }
}