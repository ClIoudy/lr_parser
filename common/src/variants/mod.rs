use crate::{Id, NonTerminal};

mod id;
pub use id::VariantId;


/// Representation of a grammar rule. The name variant refers to the values of the automatically generated enums for non-terminals in build_parser!.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variant {
    values: Vec<Id>,
    id: VariantId,
}

impl Variant {
    pub fn new(values: Vec<Id>, id: VariantId) -> Self {
        assert!(values.len() == id.length());
        Self {
            values,
            id
        }
    }

    pub fn id(&self) -> &VariantId {
        &self.id
    }

    pub fn name(&self) -> &String {
        self.id.name()
    }

    pub fn values(&self) -> &Vec<Id> {
        &self.values
    }
    
    pub fn symbol(&self) -> &NonTerminal {
        self.id.symbol()
    }

    pub fn length(&self) -> usize {
        self.id.length()
    }
}