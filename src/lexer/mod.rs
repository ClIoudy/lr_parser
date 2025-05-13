use std::error::Error;


mod error;
use error::LexingError;
use regex_lite::Regex;

use crate::{IdTrait, Token, TokenType};

/// Creates an AST from an input string via a given set of token types.
pub struct Lexer<T: IdTrait> {
    token_types: Vec<TokenType<T>>,
}

impl<T: IdTrait> Lexer<T> {
    pub fn new(token_types: Vec<impl Into<TokenType<T>>>) -> Self {
        Self {
            token_types: crate::vec_into(token_types)
        }
    }

    pub fn add_token_type(&mut self, t: TokenType<T>) {
        self.token_types.push(t);
    }

    pub fn lex(&self, input: &str) -> Result<Vec<Token<T>>, Box<dyn Error>> {
        let mut input = input;
        let mut res = vec![];

        while !input.is_empty() {
            let token = self.longest_find(&input)?;
            input = &input[token.len()..];    
            res.push(token);
        }

        Ok(res)

    }

    fn longest_find(&self, haystack: &str) -> Result<Token<T>, Box<dyn Error>> {
        
        let mut longest_find: Option<Token<T>> = None;

        for t in &self.token_types {            
            if let Some(find) = t.match_start(haystack) {
                if longest_find.is_none() 
                    || longest_find.clone().is_some_and(|x| find.len() > x.len()) 
                {
                    longest_find = Some(Token::new(t.id(), find));
                }
            }
        }

        if longest_find.is_none() {
            return Err(Box::new(LexingError::new(format!("can't match {} to any regex", haystack))));
        }

        Ok(longest_find.unwrap())
    }

}



