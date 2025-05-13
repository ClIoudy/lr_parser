use crate::IdTrait;

use super::{State, StateItem};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Action<R: IdTrait, T: IdTrait> {
    SHIFT(State<R, T>),
    REDUCE(StateItem<R, T>)
}