use proc_macro2::TokenStream;
use quote::quote;

use crate::table::Table;

pub fn expected(table: &Table) -> TokenStream {
    // let mut res = vec![];

    // for (state, answer) in &table.expected {
    //     let answer = answer.into_iter();
    //     res.push(quote! {
    //         #state => set! { #(#answer),* }
    //     });
    // }

    // quote! {
    //     match state {

    //     }
    // }

    TokenStream::new()
}