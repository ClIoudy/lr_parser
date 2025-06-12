mod parser;
// pub use parser::;

mod tokens;

use std::fmt::{Debug, Display};

pub(crate) use tokens::Token;

pub use common::*;
pub use macros::build_parser;

use parser::{ParseError, ParseInstance};



mod lexer;
use lexer::Lexer;
pub use lexer::LexError;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Error {
    Parsing(ParseError),
    Lexing(LexError),
    Alphabet(regex::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for Error {}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::Parsing(value)
    }
}

impl From<LexError> for Error {
    fn from(value: LexError) -> Self {
        Self::Lexing(value)
    }
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Self::Alphabet(value)
    }
}

pub trait ParserTrait<T: TableTrait> {
    fn parse(to_parse: &str) -> Result<Box<T::StartSymbol>, Error> {
        let lexer = Lexer::from_alphabet(T::alphabet())?;
        let tokens = lexer.lex(&to_parse)?;
        Ok(ParseInstance::<T>::new(tokens)?.parse()?)
    }
}
