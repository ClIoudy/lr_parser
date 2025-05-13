use std::any::Any;

pub mod rule;
pub use rule::*;

use crate::{Id, IdTrait};
pub type Grammar<R, T, V> = Box<dyn GrammarTrait<R, T, V>>;


pub trait GrammarTrait<R: IdTrait, T: IdTrait, V: VariantId> {
    fn rule(&self, id: &R) -> Vec<Rule<R, T, V>>;
    fn build_rule(&self, id: &Id<R, T>, children: Vec<Box<dyn Any>>) -> Option<Rule<R, T, V>>;
    fn start_symbol(&self) -> R;
    fn all_rules(&self) -> Vec<Rule<R, T, V>>;
}