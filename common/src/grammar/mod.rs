mod variant;
use crate::NonTerminal;
pub use variant::{Variant, VariantId};

pub trait RuleTrait {
    fn id(&self) -> NonTerminal;
}