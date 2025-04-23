use std::collections::{HashMap, HashSet};

mod rule;
pub use rule::Rule;
use crate::{vec_into, Token};

#[derive(Debug, Clone)]
pub struct Grammar {
    start_symbol: Token,
    rules: HashMap<Token, HashSet<Rule>>,
}

impl Grammar {
    pub fn new(start_symbol: impl Into<Token>) -> Self {
        Self {
            start_symbol: start_symbol.into(),
            rules: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: impl Into<Token>, rule: impl Into<Rule>) {
        let t = key.into();
        if let Some(set) = self.rules.get_mut(&t) {
            set.insert(rule.into());
        } else {
            let mut set = HashSet::new();
            set.insert(rule.into());
            self.rules.insert(t, set);
        }
    }

    pub fn add_all(&mut self, key: impl Into<Token>, rules: Vec<impl Into<Rule>>) {
        let t = key.into();
        if let Some(set) = self.rules.get_mut(&t) {
            set.extend(vec_into(rules));
        } else {
            let set = HashSet::from_iter(vec_into(rules));
            self.rules.insert(t, set);
        }
    }

    pub fn rules(&self) -> &HashMap<Token, HashSet<Rule>> {
        &self.rules
    }

    pub fn rules_mut(&mut self) -> &mut HashMap<Token, HashSet<Rule>> {
        &mut self.rules
    }

    pub fn start_symbol(&self) -> &Token {
        &self.start_symbol
    }
}