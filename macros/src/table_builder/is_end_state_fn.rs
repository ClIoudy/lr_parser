use proc_macro::TokenStream;

use crate::grammar::Grammar;

pub fn is_end_state(grammar: &Grammar) -> TokenStream {
    TokenStream::new()
}