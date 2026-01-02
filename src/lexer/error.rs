use std::{error::Error, fmt::{Debug, Display}};

pub struct LexError {
    message: String,
}

impl LexError {
    pub fn no_match_while_lexing(remaing_haystack: &str) -> Self {
        Self {
            message: format!("lexing found no pattern match for the remaining string: {:?}", remaing_haystack),
        }
    }
}

impl Debug for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.message))
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for LexError {

}