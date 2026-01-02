use quote::quote;

use crate::{grammar::Grammar, tests::TestRet};

#[test]
pub fn test() -> TestRet {
    let input = quote! {
        A: A -> "a";
    };

    let grammar = syn::parse2::<Grammar>(input);

    assert!(grammar.is_err(), "Grammar without start symbol S should fail");

    Ok(())
}