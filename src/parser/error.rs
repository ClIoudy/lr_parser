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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Token;

    #[test]
    fn test_parse_error_format() {
        let expected: HashSet<Terminal> = HashSet::from_iter([
            Terminal::Labeld("a".to_string()),
            Terminal::Labeld("b".to_string()),
        ]);
        let found = Token::Value {
            label: "c".to_string(),
            value: "c".to_string(),
        };
        let pos = 5;

        let error = ParseError::expected(expected, &found, pos);
        let error_msg = format!("{:?}", error);

        assert!(error_msg.contains("expected"), "Error message should contain 'expected'");
        assert!(error_msg.contains("but found"), "Error message should contain 'but found'");
        assert!(error_msg.contains("at: 5"), "Error message should contain position");
        assert!(error_msg.contains("a") || error_msg.contains("b"), "Error message should mention expected tokens");
    }

    #[test]
    fn test_parse_error_with_eof() {
        let expected: HashSet<Terminal> = HashSet::from_iter([
            Terminal::Labeld("a".to_string()),
        ]);
        let found = Token::EOF;
        let pos = 10;

        let error = ParseError::expected(expected, &found, pos);
        let error_msg = format!("{:?}", error);

        assert!(error_msg.contains("EOF"), "Error message should mention EOF");
        assert!(error_msg.contains("at: 10"), "Error message should contain position");
    }

    #[test]
    fn test_parse_error_display() {
        let expected: HashSet<Terminal> = HashSet::from_iter([
            Terminal::Labeld("x".to_string()),
        ]);
        let found = Token::Value {
            label: "y".to_string(),
            value: "y".to_string(),
        };
        let pos = 0;

        let error = ParseError::expected(expected, &found, pos);
        let debug_msg = format!("{:?}", error);
        let display_msg = format!("{}", error);

        assert_eq!(debug_msg, display_msg, "Display and Debug should produce same output");
    }

    #[test]
    fn test_parse_error_is_error() {
        let expected: HashSet<Terminal> = HashSet::new();
        let found = Token::EOF;
        let pos = 0;

        let error = ParseError::expected(expected, &found, pos);
        
        // Verify it implements Error trait
        let _: &dyn Error = &error;
    }
}