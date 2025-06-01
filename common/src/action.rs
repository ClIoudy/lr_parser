use crate::Variant;

use crate::State;

pub struct Reduction {
    pub variant: Variant,
    pub length: usize,
}

pub enum Action {
    Shift(State),
    Reduce(Reduction)
}