use std::fmt::Debug;
use common::{Id, NonTerminal};
use syn::{parse::Parse, Ident, LitStr};

pub(super) struct IdParse(pub Id);

impl Parse for IdParse {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let id = if input.peek(LitStr) {
        
            Id::Terminal(
                common::Terminal::Value(
                    input.parse::<LitStr>()?.value()                        
                )
            )
        
        } else if input.peek(Ident) {
            Id::NonTerminal(
                NonTerminal::new(
                    input.parse::<Ident>()?.to_string()
                )
            )
        } else {
            return Err(
                input.error("expected identifier or literal")
            )
        };

        Ok(Self(id))
    }
}

impl Debug for IdParse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match &self.0 {
            Id::NonTerminal(x) => format!("\"{x:?}\""),
            Id::Terminal(x) => format!("{x:?}")
        })
    }
}