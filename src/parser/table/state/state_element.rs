use std::fmt::Debug;

use crate::Token;

use crate::Rule;

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct StateElement {
    start_symbol: Token,
    rule: Rule,
    position: usize,
}

impl StateElement {
    pub fn new(start_symbol: Token, rule: Rule) -> Self {
        Self {
            start_symbol,
            rule,
            position: 0
        }
    }

    pub fn get(&self) -> Option<Token> {
        self.rule
        .values()
        .get(self.position)
        .and_then(|x| Some(x.clone()))
    }

    pub fn advance(&self) -> Self {
        let mut res = self.clone();
        res.position += 1;
        res
    }

    pub fn rule(&self) -> &Rule {
        &self.rule
    }

    pub fn start_symbol(&self) -> Token {
        self.start_symbol.clone()
    }
}

impl Debug for StateElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (before_dot, after_dot) = self.rule.values().split_at(self.position);

        f.write_str(&format!("{:?} -> {:?}.{:?}", self.start_symbol, before_dot, after_dot))
    }
}