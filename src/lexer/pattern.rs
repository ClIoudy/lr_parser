use std::hash::Hash;
use regex::{Match, Regex};

/// Regex pattern for lexing/tokenizing. Always acts as a matcher for ONLY the start of an input.
#[derive(Debug)]
pub struct Pattern {
    regex: Regex,
    label: String,
}

impl Pattern {
    /// Creates a new pattern from a given regex string.
    pub fn new(matcher: &str) -> Result<Self, regex::Error> {
        // try creating regex from matcher, if invalid syntax, return error
        Regex::new(matcher)?;

        let m = "^".to_string() + matcher;
        let regex = Regex::new(&m);
        
        if regex.is_err() {
            unreachable!("pattern building error. Patterns need to be compatible with preceding with '^'. This is used internally for lexing");
        }

        Ok(Self {
            regex: Regex::new(&m)?,
            label: matcher.to_string(),
        })
    }

    /// find the longest match from the start of the haystack.
    pub fn match_start<'h>(&self, haystack: &'h str) -> Option<Match<'h>> {
        self.regex.find(haystack)
    }

    /// This pattern's label. This is always equal to the regex str it was created from.
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

impl FromIterator<Pattern> for Pattern {
    fn from_iter<T: IntoIterator<Item = Pattern>>(iter: T) -> Self {
        iter.into_iter().next().unwrap()
    }
}