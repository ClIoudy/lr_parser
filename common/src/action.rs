use crate::Variant;

use crate::StateId;

pub struct Reduction {
    pub variant: Variant,
    pub length: usize,
}

pub enum Action {
    Shift(StateId),
    Reduce(Reduction)
}