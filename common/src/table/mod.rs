use std::any::Any;
use crate::{Action, Id, RuleTrait, VariantId};

mod state;
pub use state::StateId;

pub trait TableTrait {
    fn start_state(&self) -> &StateId;
    fn action(&self, state: &StateId, token: &Id) -> Option<Action>;
    fn build_rule(&self, variant: VariantId, children: Vec<Box<dyn Any>>) -> Box<dyn RuleTrait>;
}