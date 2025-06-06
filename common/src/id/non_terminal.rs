use std::fmt::Debug;


#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonTerminal {
    pub x: String,
}

impl Debug for NonTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.x)
    }
}

impl NonTerminal {
    pub fn new(x: String) -> Self {
        Self { x }
    }
    
    pub fn start_symbol() -> NonTerminal {
        NonTerminal::new("S".to_string())
    }
}

impl From<&str> for NonTerminal {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

impl From<String> for NonTerminal {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl PartialEq<str> for NonTerminal {
    fn eq(&self, other: &str) -> bool {
        &self.x == other
    }
}