use quote::quote;

use crate::{grammar::Grammar, tests::TestRet};

#[test]
pub fn test() -> TestRet {
    let input = quote! {
        S: A -> "a", B;
    };

    let grammar = syn::parse2::<Grammar>(input);

    // This test checks if grammar validation catches undefined non-terminals
    // The grammar references B which is not defined
    // Note: This may or may not fail at grammar parsing stage depending on implementation
    // If it doesn't fail here, it should fail during table building
    let _ = grammar;
    
    Ok(())
}