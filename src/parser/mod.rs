use std::{any::Any, marker::PhantomData};

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
    _phantom: PhantomData<T>, 
}

impl<T: TableTrait> Parser<T> {
    pub fn new() -> Self {
        Self {
            state_machine: StateMachine::new::<T>(),
            _phantom: PhantomData,
        }
    }


    pub fn parse<S: 'static>(&self, to_parse: Vec<Token>) -> Result<Box<S>, ParseError> {
        ParseInstance::<T>::new(to_parse).parse()
    }
}