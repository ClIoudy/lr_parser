use std::collections::HashSet;

use crate::{Id, IdTrait, Rule, VariantId};

mod item;
mod action;
pub use item::StateItem;
pub use action::Action;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct State<R: IdTrait, T: IdTrait> {
    items: Vec<StateItem<R, T>>,
}

impl<R: IdTrait, T: IdTrait> State<R, T> {
    pub fn new(items: impl IntoIterator<Item=StateItem<R, T>>) -> Self {
        Self {
            items: Vec::from_iter(items)
        }
    }

    pub fn items(&self) -> &Vec<StateItem<R, T>> {
        &self.items
    }
}

impl<R: IdTrait, T: IdTrait> Extend<StateItem<R, T>> for State<R, T> {
    fn extend<I: IntoIterator<Item = StateItem<R, T>>>(&mut self, iter: I) {
        self.items.extend(iter)
    }
}