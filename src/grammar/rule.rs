use crate::TokenIdent;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Rule {
    values: Vec<TokenIdent>,
}

impl Rule {
    pub fn new() -> Self {
        Self {
            values: vec![],
        }
    }

    pub fn add(&mut self, token: impl Into<TokenIdent>) {
        self.values.push(token.into());
    }

    pub fn values(&self) -> &Vec<TokenIdent> {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut Vec<TokenIdent> {
        &mut self.values
    }
}

impl FromIterator<TokenIdent> for Rule {
    fn from_iter<T: IntoIterator<Item = TokenIdent>>(iter: T) -> Self {
        Self {
            values: iter.into_iter().collect()
        }
    }
}

impl<T: Into<TokenIdent>> From<Vec<T>> for Rule {
    fn from(value: Vec<T>) -> Self {
        Self {
            values: super::vec_into(value),
        }
    }
}

impl Extend<TokenIdent> for Rule {
    fn extend<T: IntoIterator<Item = TokenIdent>>(&mut self, iter: T) {
        self.values.extend(iter);
    }
}