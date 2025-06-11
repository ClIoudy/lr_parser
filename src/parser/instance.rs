use std::{any::Any, collections::HashSet, marker::PhantomData};

use crate::Token;

use common::*;

use super::{error::ParseError, StateMachine};

pub struct ParseInstance<T: TableTrait> {
    state_machine: StateMachine,
    result_stack: Vec<Box<dyn Any>>,
    to_parse: Vec<Token>,
    _phantom: PhantomData<T>,
}

impl<T: TableTrait> ParseInstance<T> {
    
    pub fn new(mut to_parse: Vec<Token>) -> Self {
        to_parse.reverse();

        Self {
            state_machine: StateMachine::new::<T>(),
            to_parse,
            result_stack: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn parse(mut self) -> Result<Box<T::StartSymbol>, ParseError> {
        loop {

            if self.result_stack.len() == 1 && self.result_stack[0].is::<T::StartSymbol>() {
                break;
            }
            
            let lookahead = self.next();
            let state = self.state_machine.state();
            
            dbg!(state);
            let action = T::action(state, &Id::T(dbg!(lookahead.id())));
            dbg!(&action);
            println!();
            if action.is_none() {
                return Err(ParseError::expected(T::expected(state).unwrap_or(HashSet::new()), lookahead));
            }

            match action.unwrap() {
                Action::Shift(new_state) => self.shift(new_state, lookahead),
                Action::Reduce(reduction) => self.reduce(reduction),
            };
        }

        Ok(self
            .result_stack
            .pop().unwrap()
            .downcast().unwrap()
        )
    }

    /// return next element to parse or special EOF (end of file) symbol
    fn next(&mut self) -> Token {
        self
            .to_parse
            .pop()
            .unwrap_or(Token::eof())
    }

    fn shift(&mut self, new_state: StateId, lookahead: Token) {
        self.state_machine.advance(new_state);

        match lookahead {
            Token::EOF => (),
            // Token::Value(_) => self.result_stack.push(Box::new(lookahead.value().to_string())),
            Token::Value { label, value }  => self.result_stack.push(Box::new(dbg!(value.clone()))),
        }
    }

    fn reduce(&mut self, variant: VariantId) {
        let l = variant.length();
        let n = self.result_stack.len();

        // go back to state where rule originated
        self.state_machine.revert(l);

        // get children
        let mut children = self.result_stack.split_off(n - l);
        children.reverse();
        // get symbol for transitioning further from it
        let id = variant.symbol().clone();

        // create new rule
        let new_rule = T::build_rule(dbg!(variant), dbg!(children));

        if new_rule.is_none() {
            dbg!(self.state_machine.state());
            unreachable!("Couldn't build rule");
        }

        self.result_stack.push(new_rule.unwrap());

        // advance state
        let transition = T::action(self.state_machine.state(), &Id::N(id));

        match transition {
            Some(Action::Shift(new_state)) => self.state_machine.advance(new_state),
            // _ => unreachable!("Table should always ensure that there is a transition after a reduction.")
            _ => (),
        }
    }
}