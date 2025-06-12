#![feature(assert_matches)]

use proc_macro::TokenStream;
mod grammar;
use grammar::Grammar;
use syn::{parse::Parse, token::Token, Ident};

use crate::enums::enums;
mod enums;

mod table;

#[cfg(test)]
mod tests;

mod parser;

// #[cfg(test)]
// pub use tests::test_quoting;


/// rule syntax:
/// `rule` = \<symbol>: \<name> -> \<Elements> 
/// <br>where elements are comma seperated terminals/non-terminals
/// <br> (`"..."` for terminal symbols and `identifiers` for non-terminals.)
/// <br> example:
/// <br> S: A -> "a", S;
/// <br> S: B -> "b";
#[proc_macro]
pub fn build_parser(input: TokenStream) -> TokenStream {
    let grammar = syn::parse_macro_input!(input as Grammar);

    let mut res = enums(&grammar);

    res.extend(table::table(&grammar));
    
    res.extend(parser::parser_struct_tokens());

    res.into()
}

pub (crate) trait ParseShortcuts {
    fn ident(&self) -> syn::Result<Ident>;
    fn punctuated_vec<T: Parse, P: Parse + Token>(&self) -> syn::Result<Vec<T>>;
}