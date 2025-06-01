use std::fmt::Debug;
use syn::{parse::Parse, Ident, LitStr};

pub enum RuleElement {
    Literal(String),
    Rule(Ident),
}

impl Parse for RuleElement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Literal(input.parse::<LitStr>()?.value()))
        } else if input.peek(Ident) {
            Ok(Self::Rule(input.parse()?))
        } else {
            Err(input.error("expected identifier or literal"))
        }
    }
}

impl Debug for RuleElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Literal(x) => format!("\"{x}\""),
            Self::Rule(x) => x.to_string()
        })
    }
}