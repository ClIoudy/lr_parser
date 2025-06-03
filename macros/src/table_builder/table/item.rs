use common::{Id, NonTerminal, Variant};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StateItem {
    position: usize,
    variant: Variant,
}

impl StateItem {
    pub fn new(variant: Variant) -> Self {
        Self {
            position: 0,
            variant,
        }
    }

    pub fn symbol(&self) -> &NonTerminal {
        self.variant.symbol()
    }

    pub fn get(&self) -> Option<&Id> {
        self.variant.values().get(self.position)
    }

    pub fn variant(&self) -> &Variant {
        &self.variant
    }

    pub fn advance(&self) -> Option<Self> {
        
        if self.position + 1 >= self.variant.values().len() {
            return None
        }
        
        Some(Self {
            variant: self.variant.clone(),
            position: self.position + 1,
        })
    }
}