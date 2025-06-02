use std::any::Any;
use crate::{Action, Id, RuleTrait, Variant};

mod state;
pub use state::StateId;

pub trait TableTrait {
    fn start_state(&self) -> &StateId;

    fn action(&self, state: &StateId, token: &Id) -> Option<Action>;

    fn is_end_state(&self, state: &StateId) -> bool;

    fn build_rule(&self, children: Vec<Box<dyn Any>>, variat: Variant) -> Box<dyn RuleTrait>;
}