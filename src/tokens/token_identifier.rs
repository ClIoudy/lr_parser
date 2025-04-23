use std::fmt::Debug;

use crate::{Token, TokenType};

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum TokenIdent {
    LABEL(String),
    VALUE(String),
}

impl TokenIdent {
    pub fn label(label: &str) -> Self {
        Self::LABEL(label.to_string())
    }
}

impl From<TokenType> for TokenIdent {
    fn from(value: TokenType) -> Self {
        Self::LABEL(value.label.unwrap())
    }
}

impl From<Token> for TokenIdent {
    fn from(value: Token) -> Self {
        match value.label {
            Some(l) => Self::LABEL(l),
            None => Self::VALUE(value.value.to_string()),
        }
    }
}

impl From<&str> for TokenIdent {
    fn from(value: &str) -> Self {
        Self::VALUE(value.to_string())
    }
}

impl PartialEq<Token> for TokenIdent {
    fn eq(&self, other: &Token) -> bool {
        match self {
            TokenIdent::LABEL(label) => other.label.as_ref().is_some_and(|x| *x == *label),
            TokenIdent::VALUE(value) => other.value == *value,
        }
    }
}

impl TryInto<Token> for TokenIdent {
    type Error = String;

    fn try_into(self) -> Result<Token, Self::Error> {
        match self {
            TokenIdent::LABEL(_) => Err("TokenIdent has to be of value type to allow for conversion into token.".to_string()),
            TokenIdent::VALUE(value) => Ok(Token::new(value, None)),
        }
    }
}

impl Debug for TokenIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenIdent::LABEL(label) => f.write_fmt(format_args!("label: {}", label)),
            TokenIdent::VALUE(value) => f.write_str(&value)
        }
    }
}