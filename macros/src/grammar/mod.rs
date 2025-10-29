use std::collections::HashMap;
use crate::ParseShortcuts;

use common::{NonTerminal, Variant};
use syn::{parse::{Parse, ParseStream}, punctuated::Punctuated, token::Token, Ident, Token};

mod rule;
use rule::VariantParser;

mod id_parse;
use id_parse::IdParse;

#[cfg(test)]
mod tests;


#[derive(Debug, Clone)]
pub struct Grammar {
    rules: HashMap<NonTerminal, Vec<Variant>>,
}

impl Grammar {
    pub fn new(rules: HashMap<NonTerminal, Vec<Variant>>) -> Self {
        Self {
            rules
        }
    }

    pub fn rule(&self, id: &NonTerminal) -> &Vec<Variant> {
        let res = self.rules.get(id);

        if res.is_none() {
            panic!("unknown rule {:?}", id.x);
        }

        res.unwrap()
    }

    pub fn all_rules(&self) -> &HashMap<NonTerminal, Vec<Variant>> {
        &self.rules
    }

    fn check_validity(&self) -> () {
        assert!(self.rules.get(&NonTerminal::start_symbol()).is_some(), "Grammar must contain start symbol 'S'");
    }
}

impl Parse for Grammar {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let rules = input.parse_terminated(VariantParser::parse, Token![;])?.into_iter().map(|x| x.0);

        let rules = rules.into_iter().fold(HashMap::new(), |mut acc: HashMap<NonTerminal, Vec<Variant>>, x| {
            if let Some(entry) = acc.get_mut(x.symbol()) {
                entry.push(x)
            } else {
                acc.insert(x.symbol().clone(), vec![x]);
            }

            acc
        });

        let res = Self::new(rules);
        res.check_validity();
        Ok(res)
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