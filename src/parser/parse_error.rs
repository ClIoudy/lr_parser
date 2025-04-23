use std::{error::Error, fmt::{Debug, Display}};

use crate::{tokens::Token, TokenIdent};

#[derive(Clone)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
    
    pub fn expected(expected: &Vec<TokenIdent>, actual: &Token) -> Self {
        Self {
            message: format!("expected one of: {:?}, found: {:?}", expected, actual)
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Error while parsing: {}", self.message))
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Error while parsing: {}", self.message))  
    }
}

impl Error for ParseError {

}