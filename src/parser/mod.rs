mod table;
use parse_error::ParseError;
use table::{TableBuilder, Table};
use crate::{tokens::Token, Grammar};
mod ast;
pub use ast::Ast;
mod parse_error;
mod parse_instance;
use parse_instance::ParseInstance;

pub struct Parser {
    table: Table,
}

impl Parser {
    pub fn new(grammar: Grammar) -> Self {
        let table = TableBuilder::new(grammar).build();

        Self {
            table,
        }
    }

    pub fn table(&self) -> &Table {
        &self.table
    }

    pub fn parse(&self, to_parse: Vec<Token>) -> Result<Ast, ParseError> {
        ParseInstance::new(
            to_parse, 
            &self.table, 
        ).parse()
    }
}