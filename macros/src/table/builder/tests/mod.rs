use std::str::FromStr;
use proc_macro2::TokenStream;
use crate::grammar::Grammar;

mod closure_test;
mod follow_test;
mod expand_test;

mod build_test;

use super::*;

fn get_grammar() -> Result<Grammar, Box<dyn std::error::Error>> {
    let input = include_str!("grammar");
    let input = TokenStream::from_str(input)?;
    
    let grammar: Grammar = syn::parse2(input)?;

    Ok(grammar)
}