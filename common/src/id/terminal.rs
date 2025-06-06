use std::fmt::Debug;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Terminal {
    EOF,
    Value(String),
}

impl PartialEq<str> for Terminal {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::EOF => false,
            Self::Value(x) => x.as_str() == other
        }
    }
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
            Terminal::Value(x) => f.write_fmt(format_args!("\"{}\"", x)),
            Terminal::EOF => f.write_str("$"),
        }
    }
}