use crate::StateId;
use crate::VariantId;

pub enum Action {
    Shift(StateId),
    Reduce(VariantId)
}