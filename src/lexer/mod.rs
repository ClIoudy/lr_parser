use std::error::Error;

use crate::{Token, TokenType};

mod error;
use error::LexingError;
use regex_lite::Regex;

/// Creates an AST from an input string via a given set of token types.
pub struct Lexer {
    token_types: Vec<TokenType>,
}

impl Lexer {
    pub fn new(token_types: Vec<impl Into<TokenType>>) -> Self {
        Self {
            token_types: crate::vec_into(token_types)
        }
    }

    pub fn add_token_type(&mut self, t: TokenType) {
        self.token_types.push(t);
    }

    pub fn lex(&self, input: &str) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut input = input;
        let mut res = vec![];

        while !input.is_empty() {
            let token = self.longest_find(&input)?;
            input = &input[token.len()..];    
            res.push(token);
        }

        Ok(res)

    }

    fn longest_find(&self, haystack: &str) -> Result<Token, Box<dyn Error>> {
        
        let mut longest_find: Option<Token> = None;

        for t in &self.token_types {
            let match_start_expr = "^".to_owned() + t.matcher.as_str();
            let match_start_regex = Regex::new(&match_start_expr)?;
            
            if let Some(find) = match_start_regex.find(haystack) {
                if longest_find.is_none() 
                    || longest_find.clone().is_some_and(|x| find.len() > x.len()) 
                {
                    let v = find.as_str().to_string();
                    let label = t.label.clone();
                    longest_find = Some(Token::new(v, label));
                }
            }
        }

        if longest_find.is_none() {
            return Err(Box::new(LexingError::new(format!("can't match {} to any regex", haystack))));
        }

        Ok(longest_find.unwrap())
    }

}



