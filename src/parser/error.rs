use std::{collections::HashSet, error::Error, fmt::{Debug, Display}};

use common::Terminal;

use crate::Token;

pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn expected(expected_keys: HashSet<Terminal>, found: &Token, pos: usize) -> Self {
        Self {
            message: format!("expected one of the labels {:?} but found: {:?} (at: {pos})", expected_keys, found)
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for ParseError {
    
}