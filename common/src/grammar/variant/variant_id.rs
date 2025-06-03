use crate::NonTerminal;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VariantId {
    symbol: NonTerminal,
    name: NonTerminal,
    length: usize,
}

impl VariantId {
    pub fn new(symbol: String, name: String, length: usize) -> Self {
        Self {
            symbol: NonTerminal::new(symbol),
            name: NonTerminal::new(name),
            length,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }
    
    pub fn name(&self) -> &NonTerminal {
        &self.name
    }

    pub fn symbol(&self) -> &NonTerminal {
        &self.symbol
    }
}