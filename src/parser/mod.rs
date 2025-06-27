use crate::Token;

mod error;
pub(crate) use error::ParseError;

use common::*;

mod state_machine;
use state_machine::StateMachine;

use std::{any::Any, collections::HashSet, marker::PhantomData};

pub(super) struct ParseInstance<T: TableTrait> {
    current_position: usize,
    state_machine: StateMachine,
    result_stack: Vec<Box<dyn Any>>,
    to_parse: Vec<Token>,
    _phantom: PhantomData<T>,
    debug_result_stack: Vec<String>,
    next_id: Id,
}

impl<T: TableTrait> ParseInstance<T> {
    
    pub fn new(mut to_parse: Vec<Token>) -> Result<Self, ParseError> {
        to_parse.reverse();
        
        let next_id = Id::T(to_parse.last().unwrap_or(&Token::EOF).id());

        Ok(Self {
            current_position: 0,
            state_machine: StateMachine::new::<T>(),
            to_parse,
            result_stack: vec![],
            _phantom: PhantomData,
            debug_result_stack: vec![],
            next_id
        })
    }

    pub fn parse(mut self) -> Result<Box<T::StartSymbol>, ParseError> {
        loop {
            if self.result_stack.len() == 1 && self.to_parse.len() == 0 && self.result_stack[0].is::<T::StartSymbol>() {
                break;
            }

            
            let id = &self.next_id;
            let state = self.state_machine.state();
            
            let action = T::action(state, id);

            if action.is_none() {
                return Err(ParseError::expected(T::expected(state).unwrap_or(HashSet::new()), &self.to_parse.pop().unwrap(), self.current_position));
            }

            match action.unwrap() {
                Action::Shift(next_state) => self.shift(next_state),
                Action::Reduce(reduction) => self.reduce(reduction),
                Action::Goto(next_state) => {
                    self.state_machine.advance(next_state);
                    let next_id = self.to_parse.last().unwrap_or(&Token::EOF).id();
                    self.next_id = Id::T(next_id);
                }
            };

        }

        Ok(self
            .result_stack
            .pop().unwrap()
            .downcast().unwrap()
        )
    }

    /// returns the current token and advances next_id
    fn next(&mut self) -> Token {
        let res = self
            .to_parse
            .pop()
            .unwrap_or(Token::eof());

        match &res {
            Token::EOF => (),
            Token::Value { label: _, value } => self.current_position += value.len(),
        };


        let next_id = self.to_parse.last().unwrap_or(&Token::EOF).id();
        self.next_id = Id::T(next_id);

        res
    }

    fn shift(&mut self, next_state: StateId) {
        self.state_machine.advance(next_state);
        
        let token = self.next();

        match token {
            Token::EOF => (),
            Token::Value { label: _, value }  => {
                self.debug_result_stack.push(value.clone());

                self.result_stack.push(Box::new(value))
            },
        }
    }

    fn reduce(&mut self, variant: VariantId) {
        let l = variant.length();
        let n = self.result_stack.len();

        // go back to state where rule originated
        self.state_machine.revert(l);

        self.debug_result_stack.truncate(n - l);
        // get children
        let mut children = self.result_stack.split_off(n - l);
        children.reverse();
        // get symbol for transitioning further from it
        let id = variant.symbol().clone();

        self.debug_result_stack.push(variant.symbol().clone().x);

        // create new rule
        let new_rule = T::build_rule(variant, children);

        if new_rule.is_none() {
            self.state_machine.state();
            unreachable!("Couldn't build rule");
        }

        self.result_stack.push(new_rule.unwrap());

        self.next_id = Id::N(id);
    }
}