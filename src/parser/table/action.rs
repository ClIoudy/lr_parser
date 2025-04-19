use super::{State, StateElement};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Action {
    // Shift(State),
    // Goto(State),
    Transition(State),
    Reduce(StateElement),
}