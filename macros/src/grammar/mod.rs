use std::collections::HashMap;

use syn::{parse::{discouraged::Speculative, Parse, ParseStream}, punctuated::Punctuated, token::Token, Ident, Token};
mod rule;
pub use rule::Rule;
use variant::Variant;
mod variant;
mod rule_element;


#[derive(Debug)]
pub struct Grammar {
    pub rules: HashMap<String, Vec<Variant>>,
}

impl Grammar {
    fn parse_start_rule(input: syn::parse::ParseStream) -> syn::Result<Rule> {
        let fork = input.fork();

        let pound_missing = fork.parse::<Token![#]>().is_err();
        let mut rule = fork.parse::<Rule>()?;

        if pound_missing || rule.symbol != "S".to_string() {
            Err(input.error("expected '#S' as start symbol of the grammar"))
        } else {
            input.advance_to(&fork);
            input.parse::<Token![;]>()?;
            rule.symbol = "#S".to_string();
            Ok(rule)
        }
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

        let rules = rules.into_iter().fold(HashMap::new(), |mut acc: HashMap<String, Vec<variant::Variant>>, x| {

            if let Some(entry) = acc.get_mut(&x.symbol) {
                entry.push(x.variant)
            } else {
                acc.insert(x.symbol, vec![x.variant]);
            }

            acc
        });

        Ok(Self {
            rules
        })
    }
}

trait ParseShortcuts {
    fn ident(&self) -> syn::Result<Ident>;
    fn punctuated_vec<T: Parse, P: Parse + Token>(&self) -> syn::Result<Vec<T>>;
}

impl ParseShortcuts for ParseStream<'_> {
    fn ident(&self) -> syn::Result<Ident> {
        self.parse::<Ident>()
    }

    fn punctuated_vec<T: Parse, P: Parse + Token>(&self) -> syn::Result<Vec<T>> {
        Ok(Punctuated::<T, P>::parse_separated_nonempty(&self)?.into_iter().collect())
    }
}