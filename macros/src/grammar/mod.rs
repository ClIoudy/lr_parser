use std::collections::HashMap;

use common::{Id, NonTerminal, Variant};
use syn::{parse::{discouraged::Speculative, Parse, ParseStream}, punctuated::Punctuated, token::Token, Ident, Token};
mod rule;
pub use rule::Rule;

mod id_parse;
use id_parse::IdParse;

use crate::ParseShortcuts;

#[derive(Debug)]
pub struct Grammar {
    rules: HashMap<NonTerminal, Vec<Variant>>,
}

impl Grammar {
    pub fn new(rules: HashMap<NonTerminal, Vec<Variant>>) -> Self {
        Self {
            rules
        }
    }

    fn parse_start_rule(input: syn::parse::ParseStream) -> syn::Result<Rule> {
        let fork = input.fork();

        let pound_missing = fork.parse::<Token![#]>().is_err();
        let mut rule = fork.parse::<Rule>()?;

        if pound_missing || rule.symbol() != "S" {
            Err(input.error("expected '#S' as start symbol of the grammar"))
        } else {
            input.advance_to(&fork);
            input.parse::<Token![;]>()?;
            rule.set_symbol("#S".to_string());
            Ok(rule)
        }
    }

    pub fn rule(&self, id: &NonTerminal) -> &Vec<Variant> {
        let res = self.rules.get(id);

        if res.is_none() {
            panic!("unknown rule {:?}", id.symbol);
        }

        res.unwrap()
    }

    pub fn all_rules(&self) -> &HashMap<NonTerminal, Vec<Variant>> {
        &self.rules
    }
}

impl Parse for Grammar {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut start_rule = vec![Self::parse_start_rule(input)?];

        while let Ok(another_start_rule) = Self::parse_start_rule(input) {
            start_rule.push(another_start_rule);
        }

        let mut rules = input.parse_terminated(Rule::parse, Token![;])?;

        rules.extend(start_rule);

        let rules = rules.into_iter().fold(HashMap::new(), |mut acc: HashMap<NonTerminal, Vec<Variant>>, x| {

            if let Some(entry) = acc.get_mut(&x.id) {
                entry.push(x.variant)
            } else {
                acc.insert(x.id, vec![x.variant]);
            }

            acc
        });

        Ok(Self::new(rules))
    }
}

impl ParseShortcuts for ParseStream<'_> {
    fn ident(&self) -> syn::Result<Ident> {
        self.parse::<Ident>()
    }

    fn punctuated_vec<T: Parse, P: Parse + Token>(&self) -> syn::Result<Vec<T>> {
        Ok(Punctuated::<T, P>::parse_separated_nonempty(&self)?.into_iter().collect())
    }
}