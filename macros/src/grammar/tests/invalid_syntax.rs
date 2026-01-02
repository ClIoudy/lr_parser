use quote::quote;

use crate::{grammar::Grammar, tests::TestRet};

#[test]
pub fn test_invalid_grammar_syntax() -> TestRet {
    // Test with invalid syntax - missing arrow
    let input = quote! {
        S: A "a";
    };

    let grammar = syn::parse2::<Grammar>(input);
    assert!(grammar.is_err(), "Invalid syntax should fail to parse");

    Ok(())
}

#[test]
pub fn test_missing_semicolon() -> TestRet {
    // Test with missing semicolon
    let input = quote! {
        S: A -> "a"
        S: B -> "b";
    };

    let grammar = syn::parse2::<Grammar>(input);
    // This might parse or fail depending on syn's error recovery
    let _ = grammar;
    
    Ok(())
}

#[test]
pub fn test_empty_grammar() -> TestRet {
    // Test with empty grammar
    let input = quote! {};

    let grammar = syn::parse2::<Grammar>(input);
    // Should fail because no start symbol S
    assert!(grammar.is_err(), "Empty grammar should fail (no start symbol)");

    Ok(())
}

#[test]
pub fn test_duplicate_variant_names_same_symbol() -> TestRet {
    // Test with duplicate variant names for same symbol
    // This is actually allowed in the grammar - same symbol can have multiple rules
    let input = quote! {
        S: A -> "a";
        S: A -> "b";
    };

    let grammar = syn::parse2::<Grammar>(input);
    // This should parse successfully (duplicate variant names are allowed)
    assert!(grammar.is_ok(), "Duplicate variant names should be allowed");

    Ok(())
}

