mod table;
use parse_error::ParseError;
use table::{TableBuilder, Table};
use crate::{GrammarTrait, IdTrait, Token, VariantId};
mod ast;
pub use ast::Ast;
mod parse_error;
mod parse_instance;
use parse_instance::ParseInstance;

pub struct Parser<R: IdTrait, T: IdTrait, V: VariantId, G: GrammarTrait<R, T, V>> {
    table: Table<R, T, V, G>,
}

impl<R: IdTrait, T: IdTrait, V: VariantId, G: GrammarTrait<R, T, V>> Parser<R, T, V, G> {
    pub fn new(grammar: G) -> Self {
        let table = TableBuilder::new(grammar).build();

        Self {
            table,
        }
    }

    pub fn table(&self) -> &Table<R, T, V, G> {
        &self.table
    }

    pub fn parse(&self, to_parse: Vec<Token<T>>) -> Result<Ast<'_, T>, ParseError> {
        ParseInstance::new(
            to_parse, 
            &self.table, 
        ).parse()
    }
}