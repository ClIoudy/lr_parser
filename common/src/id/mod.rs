use std::fmt::Debug;

#[derive(Clone, Hash, PartialEq, Eq)]
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
        NonTerminal::new("#S".to_string())
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

impl PartialEq<str> for Terminal {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::EOF => false,
            Self::Value(x) => x.as_str() == other
        }
    }
}


#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Terminal {
    EOF,
    Value(String),
}

impl From<&str> for Terminal {
    fn from(value: &str) -> Self {
        Self::Value(value.to_string())
    }
}

impl From<String> for Terminal {
    fn from(value: String) -> Self {
        Self::Value(value)
    }
}

impl Debug for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminal::Value(x) => f.write_str(x),
            Terminal::EOF => f.write_str("$"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Id {
    N(NonTerminal),
    T(Terminal),
}

