use std::any::Any;
use crate::{Action, Id, RuleTrait, Variant};

mod state;
pub use state::State;

pub trait TableTrait {
    fn start_state(&self) -> &State;

    fn action(&self, state: &State, token: &Id) -> Option<Action>;

    fn is_end_state(&self, state: &State) -> bool;

    fn build_rule(&self, children: Vec<Box<dyn Any>>, variat: Variant) -> Box<dyn RuleTrait>;
}