use std::any::Any;

use crate::Token;
mod error;
use error::ParseError;

mod instance;
use instance::ParseInstance;

use common::*;

mod state_machine;
use state_machine::StateMachine;

pub struct Parser<T: TableTrait> {
    state_machine: StateMachine,
    table: T,
}

impl<T: TableTrait> Parser<T> {
    pub fn new(table: T) -> Self {
        Self {
            state_machine: StateMachine::new(&table),
            table,
        }
    }


    pub fn parse<S: 'static>(&self, to_parse: Vec<Token>) -> Result<Box<S>, ParseError> {
        ParseInstance::new(&self.table, to_parse).parse()
    }
}