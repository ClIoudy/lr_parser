use crate::{Id, NonTerminal};

mod variant_id;
pub use variant_id::VariantId;



#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Variant {
    values: Vec<Id>,
    id: VariantId,
}

impl Variant {
    pub fn new(values: Vec<Id>, id: VariantId) -> Self {
        Self {
            values,
            id
        }
    }

    pub fn id(&self) -> &VariantId {
        &self.id
    }

    pub fn name(&self) -> &NonTerminal {
        self.id.name()
    }

    pub fn values(&self) -> &Vec<Id> {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut Vec<Id> {
        &mut self.values
    }
    
    pub fn symbol(&self) -> &NonTerminal {
        self.id.symbol()
    }
}