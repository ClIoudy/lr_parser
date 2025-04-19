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
    start_symbol: Token,
}

impl Parser {
    pub fn new(grammar: Grammar, start_symbol: &str) -> Self {
        let table = TableBuilder::new(&grammar, &start_symbol.into()).build();
        
        Self {
            table,
            start_symbol: start_symbol.into(),
        }
    }

    pub fn table(&self) -> &Table {
        &self.table
    }

    // pub fn parse(&self, to_parse: Vec<Token>) -> Result<Ast, ParseError> {
    //     let mut stack = Vec::new();
    //     let mut ast_stack = Vec::new();
               
    //     let mut state_history = Vec::new();
    //     state_history.push(self.table.start_state());

    //     let mut to_parse = to_parse.into_iter().rev().collect::<Vec<_>>();

    //     loop {       
    //         let state = state_history.last().unwrap();

    //         if let Some(item) = self.table.reduction(state) {

    //             let mut ast = Ast::new(item.start_symbol().clone());

    //             for t in item.rule().iter().rev() {
    //                 let p = stack.pop().unwrap();

    //                 if p != *t {
    //                     return Err(ParseError::expected(&vec![t.clone()], &p));
    //                 }

    //                 state_history.pop().unwrap();
    //                 ast.add_child(ast_stack.pop().unwrap());
    //             }

    //             ast.children.reverse();
    //             ast_stack.push(ast);
    //             stack.push(item.start_symbol());
    //             continue;
    //         }

    //         if let Some(t) = to_parse.pop() {
    //             ast_stack.push(Ast::new(t.clone()));
    //             stack.push(t);
    //         }

    //         let token = stack.last().unwrap();

    //         if let Some(new_state) = self.table.transition(state, token) {
    //             state_history.push(new_state);
    //             continue;
    //         }

    //         if stack.len() == 1 && stack[0] == self.start_symbol  {
    //             break;
    //         }

    //         let expected = self.table.keys(&state);
    //         return Err(ParseError::expected(&expected, token));
    //     }

    //     Ok(ast_stack[0].clone())

    // }

    pub fn parse(&self, to_parse: Vec<Token>) -> Result<Ast, ParseError> {
        ParseInstance::new(to_parse, &self.table, self.start_symbol.clone()).parse()
    }
}