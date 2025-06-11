use std::hash::Hash;

use regex::{Match, Regex};

#[derive(Debug)]
pub struct Pattern {
    regex: Regex,
    label: String,
}

impl Pattern {
    pub fn new(matcher: &str) -> Result<Self, regex::Error> {
        let m = "^".to_string() + matcher;

        Ok(Self {
            regex: Regex::new(&m)?,
            label: matcher.to_string(),
        })
    }

    pub fn match_start<'h>(&self, haystack: &'h str) -> Option<Match<'h>> {
        self.regex.find(haystack)
    }

    pub fn label(&self) -> &String {
        &self.label
    }
}

impl Hash for Pattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.regex.as_str().hash(state);
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.regex.as_str() == other.regex.as_str()
    }
}

impl Eq for Pattern {

}

impl TryFrom<&str> for Pattern {
    type Error = regex::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

