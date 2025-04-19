use std::hash::Hash;

// use regex::Regex;
use regex_lite::Regex;

pub mod consts;

#[derive(Clone, Debug)]
pub struct TokenType {
    pub matcher: Regex,
    pub label: Option<String>,
}

impl TokenType {
    pub fn new(regex: &str) -> Self {
        Self {
            matcher: Regex::new(regex).unwrap(),
            label: None
        }
    }

    pub fn from_matcher(matcher: Regex) -> Self {
        Self {
            matcher,
            label: None
        }
    }

    pub fn labeld(pattern: &str, label: &str) -> Self {
        let mut res = Self::new(pattern);
        res.set_label(label);
        res
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }

    pub fn label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        Self {
            matcher: Regex::new(value).unwrap(),
            label: None,
        }
    }
}

impl Hash for TokenType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.matcher.as_str().hash(state);
        self.label.hash(state);
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        self.matcher.as_str() == other.matcher.as_str()
    }
}

impl Eq for TokenType {
    
}