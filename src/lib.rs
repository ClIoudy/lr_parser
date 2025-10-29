#![allow(soft_unstable)]

mod parser;
mod tokens;


pub(crate) use tokens::Token;

pub use common::*;
pub use macros::build_parser;

use parser::{ParseError, ParseInstance};


#[cfg(feature = "manual_lexing")]
pub mod lexer;

#[cfg(not(feature = "manual_lexing"))]
mod lexer;

pub use lexer::LexError;


#[derive(Debug)]
pub enum Error {
    Parsing(ParseError),
    Lexing(LexError),
    Alphabet(regex::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
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
    /// Lexes/tokenizes the given string and then parses it.
    fn parse(to_parse: &str) -> Result<Box<T::StartSymbol>, Error> {
        let lexer = lexer::Lexer::from_alphabet(T::alphabet())?;
        let tokens = lexer.lex(&to_parse)?;
        Ok(ParseInstance::<T>::new(tokens)?.parse()?)
    }

    /// Parses the given tokens.
    fn parse_tokenized(to_parse: Vec<Token>) -> Result<Box<T::StartSymbol>, Error> {
        Ok(ParseInstance::<T>::new(to_parse)?.parse()?)
    }
}
