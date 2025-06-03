#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NonTerminal {
    pub x: String,
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


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Id {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}

