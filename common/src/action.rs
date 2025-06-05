use crate::StateId;
use crate::VariantId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Shift(StateId),
    Reduce(VariantId)
}