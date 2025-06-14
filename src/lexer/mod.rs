mod pattern;
use std::collections::HashSet;

pub use pattern::Pattern;
use regex::Match;

mod error;
pub use error::LexError;

use crate::Token;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Lexer {
    patterns: HashSet<Pattern>,
}

impl Lexer {
    // pub fn new(patterns: HashSet<Pattern>) -> Self {
    //     Self {
    //         patterns
    //     }
    // }

    pub fn from_alphabet(alphabet: impl IntoIterator<Item = &'static str>) -> Result<Self, regex::Error> {
        let patterns: Result<HashSet<Pattern>, regex::Error> = alphabet
            .into_iter()
            .map(|x| Pattern::new(x))
            .collect();

        Ok(Self { patterns: patterns? })
    }

    // pub fn empty() -> Self {
    //     Self {
    //         patterns: HashSet::new(),
    //     }
    // }

    // pub fn add(&mut self, pattern: impl Into<Pattern>) -> bool {
    //     self.patterns.insert(pattern.into())
    // }

    // pub fn try_add<T: TryInto<Pattern>>(&mut self, pattern: T) -> Result<bool, T::Error>{
    //     Ok(self.patterns.insert(pattern.try_into()?))
    // }

    /// Lexes/tokenizes a given string based on the lexer's token-types/patterns. 
    /// If successful, returns the tokenized vector of the found token types.
    /// else, meaning if there was a portion of the 
    pub fn lex(&self, string: &str) -> Result<Vec<Token>, LexError> {
        let mut haystack = string;
        let mut res = vec![];
        
        while !haystack.is_empty() {
            if let Some((find, label)) = self.find_longest(haystack) {
                res.push(
                    Token::labeld(label, find.as_str().to_string())
                );

                haystack = &haystack[find.len()..]; 
            } else {
                return Err(LexError::no_match_while_lexing(haystack));
            }
        }

        Ok(res)
    }

    fn find_longest<'h>(&self, haystack: &'h str) -> Option<(Match<'h>, String)> {
        let mut longest_find = None;
        let mut label = None;

        for pattern in &self.patterns {
            if let Some(x) = pattern.match_start(haystack) {
                if longest_find.is_none() || longest_find.is_some_and(|l: Match<'_>| x.len() > l.len()) {
                    longest_find = Some(x);
                    label = Some(pattern.label().clone());
                }
            }
        }

        longest_find.zip(label)
    }
}

impl FromIterator<Pattern> for Lexer {
    fn from_iter<T: IntoIterator<Item = Pattern>>(iter: T) -> Self {
        Self {
            patterns: HashSet::from_iter(iter)
        }
    }
}