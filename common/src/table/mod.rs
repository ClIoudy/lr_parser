use std::{any::Any, collections::HashSet};
use crate::{Action, Id, Terminal, VariantId};

mod state;
pub use state::StateId;

pub trait TableTrait {
    type StartSymbol: 'static;
    fn start_state() -> StateId;
    fn action(state: &StateId, token: &Id) -> Option<Action>;
    fn build_rule(variant: VariantId, children: Vec<Box<dyn Any>>) -> Option<Box<dyn Any>>;
    fn expected(state: &StateId) -> Option<HashSet<Terminal>>;
    fn alphabet() -> std::collections::HashSet<&'static str>;
}