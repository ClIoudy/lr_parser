use crate::{tokens::Token, TokenIdent};
use super::{parse_error::ParseError, table::{State, StateElement, Table}, Ast};

pub struct ParseInstance<'a> {
    to_parse: Vec<Token>, 
    table: &'a Table,
    lookahead: Token,
    ast_stack: Vec<Ast>,
    stack: Vec<Token>,
    state_history: Vec<&'a State>,
}

impl<'a> ParseInstance<'a> {
    pub fn new(to_parse: Vec<Token>, table: &'a Table) -> Self {
        let mut to_parse = to_parse;
        to_parse.reverse();

        Self {
            lookahead: to_parse.pop().unwrap_or(Token::eof()),
            to_parse,
            table,
            ast_stack: vec![],
            stack: vec![],
            state_history: vec![table.start_state()],
        }
    }

    pub fn parse(mut self) -> Result<Ast, ParseError> {
        loop {
            let state = self.state();
            let identifier = TokenIdent::from(self.lookahead.clone());

            if self.table.empty(state, &identifier) {
                let expected = self.table.keys(&state);
                return Err(ParseError::expected(&expected, &self.lookahead));
            }

            if let Some(new_state) = self.table.transition(state, &identifier) {
                self.state_history.push(&new_state);
                self.ast_stack.push(Ast::new(self.lookahead.clone()));
                self.stack.push(self.lookahead.clone());
                self.next();
            } else if let Some(item) = self.table.reduction(state, &identifier) {
                let updated_state = self.reduction(&item);
                
                if self.stack.len() == 1 && self.stack[0] == *self.table.start_symbol()  {
                    break;
                }

                let new_state = self.table.transition(updated_state, &item.start_symbol().into()).unwrap();
                self.state_history.push(new_state);
            }
        }     

        let mut res = self.ast_stack.remove(0);

        // remove "eof" from ast
        res.children.pop();
        Ok(res)
    }

    fn state(&self) -> &State {
        self.state_history.last().unwrap()
    }

    fn reduction(&mut self, item: &StateElement) -> &'a Vec<StateElement> {
        // n := item rule length
        let n = item.rule().values().len();

        // rewind state by n elements
        self.state_history.truncate(self.state_history.len() - n);

        // replace last n stack elements with item start symbol 
        self.stack.truncate(self.stack.len() - n);
        self.stack.push(item.start_symbol());

        // replace last n ast stack elements with new ast (children := removed elements)
        let children = self.ast_stack.split_off(self.ast_stack.len() - n);
        let ast = Ast::with_children(item.start_symbol(), children);
        self.ast_stack.push(ast);

        self.state_history.last().unwrap()
    }

    fn next(&mut self) {
        self.lookahead = self.to_parse.pop().unwrap_or(Token::eof());
    }
}