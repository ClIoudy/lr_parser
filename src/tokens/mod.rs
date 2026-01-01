use std::fmt::Debug;

use common::Terminal;


/// Representation of a portion of text after lexing/tokenization.
#[derive(Clone, PartialEq, Eq)]
pub enum Token {
    /// EOF = End Of File
    EOF,
    Value { label: String, value: String }
}

impl Token {
    pub fn labeld(label: String, value: String) -> Self {
        Self::Value { label, value }
    }

    pub fn eof() -> Self {
        Self::EOF
    }

    pub fn id(&self) -> Terminal {
        match self {
            Self::EOF => Terminal::EOF,
            Self::Value { label, value: _ } => Terminal::Labeld(label.clone())
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::EOF => f.write_str("$ (EOF)"),
            Token::Value { label: _, value } => f.write_fmt(format_args!("\"{}\"", value))
        }
    }
}