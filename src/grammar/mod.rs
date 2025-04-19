use std::collections::{HashMap, HashSet};
use crate::Token;

mod rule;
pub use rule::Rule;
use crate::vec_into;

#[derive(Debug, Clone)]
pub struct Grammar {
    rules: HashMap<Token, HashSet<Rule>>,
}

impl Grammar {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add(&mut self, token: impl Into<Token>, rule: Rule) {
        let t = token.into();
        if let Some(set) = self.rules.get_mut(&t) {
            set.insert(rule);
        } else {
            let mut set = HashSet::new();
            set.insert(rule);
            self.rules.insert(t, set);
        }
    }

    pub fn add_all(&mut self, token: impl Into<Token>, rules: Vec<impl Into<Rule>>) {
        let t = token.into();
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
}