#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NonTerminal {
    pub symbol: String,
}

impl NonTerminal {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Terminal {
    EOF,
    Value(String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Id {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}