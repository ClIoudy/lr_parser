use std::any::Any;

use crate::Token;

use common::*;

use super::{error::ParseError, StateMachine};

pub struct ParseInstance<'a, T: TableTrait> {
    state_machine: StateMachine,
    table: &'a T,
    result_stack: Vec<Box<dyn Any>>,
    to_parse: Vec<Token>,
}

impl<'a, T: TableTrait> ParseInstance<'a, T> {
    pub fn new(table: &'a T, to_parse: Vec<Token>) -> Self {
        Self {
            state_machine: StateMachine::new(table),
            table,
            to_parse,
            result_stack: vec![],
        }
    }

    pub fn parse<S: 'static>(mut self) -> Result<Box<S>, ParseError> {
        loop {
            let lookahead = self.next();
            let state = self.state_machine.state();
            
            if self.table.is_end_state(state) && self.result_stack.len() == 1 {
                break;
            }
            
            let action = self.table.action(state, &Id::Terminal(lookahead.id()));

            if action.is_none() {
                return Err(ParseError::expected());
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
            .unwrap_or(Token::EOF)
    }

    fn shift(&mut self, new_state: StateId, lookahead: Token) {
        self.state_machine.advance(new_state);

        match lookahead {
            Token::EOF => (),
            Token::Value(x) => self.result_stack.push(Box::new(x)),
        }
    }

    fn reduce(&mut self, variant: VariantId) {
        let l = variant.length();
        let n = self.result_stack.len();

        // go back to state where rule originated
        self.state_machine.revert(l);

        // get children
        let children = self.result_stack.split_off(n - l);

        // create new rule
        let new_rule = self.table.build_rule(variant, children);

        // advance state
        let transition = self.table.action(self.state_machine.state(), &Id::NonTerminal(new_rule.id()));

        match transition {
            Some(Action::Shift(new_state)) => self.state_machine.advance(new_state),
            _ => unreachable!("Table should always ensure that there is a transition after a reduction.")
        }
    }
}