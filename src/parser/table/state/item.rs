use std::fmt::Debug;

use crate::{Id, IdTrait, Rule, VariantId};


#[derive(PartialEq, Eq, Hash, Clone)]
pub struct StateItem<R: IdTrait, T: IdTrait> {
    position: usize,
    values: Vec<Id<R, T>>,
    start_symbol: R,
}

impl<R: IdTrait, T: IdTrait> StateItem<R, T> {
    pub fn get(&self) -> Option<&Id<R, T>> {
        self.values.get(self.position)
    }

    pub fn start_symbol(&self) -> &R {
        &self.start_symbol
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn new<V: VariantId>(rule: Rule<R, T, V>) -> Self {
        Self {
            position: 0,
            values: rule.values(),
            start_symbol: rule.start_symbol(),
        }
    }

    pub fn advance(&self) -> Option<Self> {
        if self.position < self.values.len() {
            return Some(Self {
                position: self.position + 1,
                values: self.values.clone(),
                start_symbol: self.start_symbol.clone(),
            });
        }
        
        None
    }
}


impl<R: IdTrait, T: IdTrait> Debug for StateItem<R, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (before_dot, after_dot) = self.values.split_at(self.position);

        f.write_str(&format!("{:?} -> {:?}.{:?}", self.start_symbol, before_dot, after_dot))
    }
}