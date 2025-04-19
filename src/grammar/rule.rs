use crate::Token;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Rule {
    values: Vec<Token>,
}

impl Rule {
    pub fn new() -> Self {
        Self {
            values: vec![],
        }
    }

    pub fn add(&mut self, token: impl Into<Token>) {
        self.values.push(token.into());
    }

    pub fn values(&self) -> &Vec<Token> {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut Vec<Token> {
        &mut self.values
    }    
}

impl<T: Into<Token>> From<Vec<T>> for Rule {
    fn from(value: Vec<T>) -> Self {
        Self {
            values: super::vec_into(value),
        }
    }
}