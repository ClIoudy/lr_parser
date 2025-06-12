mod parser;
// pub use parser::;

mod tokens;

pub(crate) use tokens::Token;

pub use common::*;
pub use macros::build_parser;

use parser::{ParseError, ParseInstance};

pub mod lexer;

#[cfg(test)]
mod tests;

pub trait ParserTrait<T: TableTrait> {
    fn parse(to_parse: Vec<Token>) -> Result<Box<T::StartSymbol>, ParseError> {
        ParseInstance::<T>::new(to_parse)?.parse()
    }
}
