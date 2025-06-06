use std::fmt::Debug;

use crate::StateId;
use crate::VariantId;

#[derive(Clone, PartialEq, Eq)]
pub enum Action {
    Shift(StateId),
    Reduce(VariantId)
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shift(x) => f.write_fmt(format_args!("Shift({})", x)),
            Self::Reduce(x) => f.write_fmt(format_args!("Reduce({:?}: {:?})", x.symbol(), x.name())),
        }
    }
}