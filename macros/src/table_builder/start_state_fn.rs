use proc_macro::TokenStream;

use crate::grammar::Grammar;

pub fn start_state(grammar: &Grammar) -> TokenStream {
    TokenStream::new()
}