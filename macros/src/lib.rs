#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;
mod grammar;
use grammar::Grammar;

mod table_builder;

/// rule syntax:
/// `rule` = \<symbol>: \<name> -> \<Elements> 
/// <br>where elements are comma seperated terminals/non-terminals
/// <br>(`"..."` for terminal symbols and `identifiers` for non-terminals.)
/// <br>e.g.: S: A -> "a", S 
#[proc_macro]
pub fn build_parser(input: TokenStream) -> TokenStream {
    
    
    let grammar = syn::parse_macro_input!(input as Grammar);
    println!("{:#?}", grammar);
    let mut rules = grammar.rules;

    
    let expanded = quote! {

    };

    TokenStream::new()
}