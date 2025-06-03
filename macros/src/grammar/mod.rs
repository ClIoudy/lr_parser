use std::collections::HashMap;
use crate::ParseShortcuts;

use common::{Id, NonTerminal, Variant};
use syn::{parse::{discouraged::Speculative, Parse, ParseStream}, punctuated::Punctuated, token::Token, Ident, Token};

mod rule;
use rule::{VariantParser, StartRule};

mod id_parse;
use id_parse::IdParse;

#[cfg(test)]
mod tests;


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
}

impl Parse for Grammar {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_rule = input.parse::<StartRule>()?.0;
        let mut rules = input.parse_terminated(VariantParser::parse, Token![;])?.into_iter().map(|x| x.0);

        let mut rules = rules.into_iter().fold(HashMap::new(), |mut acc: HashMap<NonTerminal, Vec<Variant>>, x| {
            if let Some(entry) = acc.get_mut(x.symbol()) {
                entry.push(x)
            } else {
                acc.insert(x.symbol().clone(), vec![x]);
            }

            acc
        });

        rules.insert(NonTerminal::start_symbol(), start_rule);

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

    fn expect(&self, expected: &str) -> syn::Result<()> {
        let x = self.ident()?;

        if x != expected {
            Err(self.error(format!("expected ident {expected}, found: {x}")))
        } else {
            Ok(())
        }
    }
}