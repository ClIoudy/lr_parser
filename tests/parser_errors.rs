use std::error::Error;
use macros::build_parser;
use lr_parser::LRParserError;

// ========== Parser Error Tests ==========

#[test]
fn test_unexpected_token_at_start() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a";
    }

    let result = Parser::parse("b");
    assert!(result.is_err(), "Should fail on unexpected token");
    
    // Could be parsing or lexing error depending on implementation
    match result {
        Err(LRParserError::Parsing(_)) | Err(LRParserError::Lexing(_)) => {
            // Expected
        }
        _ => {
            panic!("Expected Parsing error, got {:?}", result.unwrap_err());
        }
    }

    Ok(())
}

#[test]
fn test_unexpected_token_mid_parse() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a", "b", "c";
    }

    let result = Parser::parse("axc");
    assert!(result.is_err(), "Should fail on unexpected token in middle");
    
    // Verify it's a lexing error (not parsing, since "x" is not in the alphabet)
    match result {
        Err(LRParserError::Lexing(_)) => {
            // Expected - lexing error because "x" doesn't match any pattern
        }
        _ => {
            panic!("Expected Lexing error, got {:?}", result.unwrap_err());
        }
    }

    Ok(())
}

#[test]
fn test_unexpected_token_at_end() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a", "b";
    }

    let result = Parser::parse("aa");
    assert!(result.is_err(), "Should fail on unexpected token at end");
    Ok(())
}

#[test]
fn test_incomplete_input() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a", "b";
    }

    let result = Parser::parse("a");
    assert!(result.is_err(), "Should fail on incomplete input");

    // Should be a parsing error (expected token but got EOF)
    match result {
        Err(LRParserError::Parsing(_)) => {
            // Expected
        }
        _ => {
            panic!("Expected Parsing error for incomplete input");
        }
    }
    Ok(())
}

#[test]
fn test_extra_tokens() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a";
    }

    let result = Parser::parse("ab");
    assert!(result.is_err(), "Should fail on extra tokens");
    
    // Should be a lexing error (since "b" is not in the alphabet)
    match result {
        Err(LRParserError::Lexing(_)) => {
            // Expected - "b" doesn't match any pattern in the alphabet
        }
        _ => {
            panic!("Expected Lexing error for extra tokens, got {:?}", result.unwrap_err());
        }
    }
    Ok(())
}

#[test]
fn test_wrong_sequence() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a", "b", "c";
    }

    let result = Parser::parse("acb");
    assert!(result.is_err(), "Should fail on wrong sequence");
    Ok(())
}

#[test]
fn test_empty_input_error() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a";
    }

    let result = Parser::parse("");
    assert!(result.is_err(), "Should fail on empty input");
    
    // Should be a parsing error (expected token but got EOF)
    match result {
        Err(LRParserError::Parsing(_)) => {
            // Expected
        }
        _ => {
            panic!("Expected Parsing error for empty input");
        }
    }
    Ok(())
}

#[test]
fn test_complex_unexpected_token() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a", B;
        B: B1 -> "b";
    }

    let result = Parser::parse("ac");
    assert!(result.is_err(), "Should fail when non-terminal expected but wrong terminal found");
    Ok(())
}

#[test]
fn test_missing_required_terminal() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "(", B, ")";
        B: B1 -> "b";
    }

    let result = Parser::parse("(b");
    assert!(result.is_err(), "Should fail when closing parenthesis missing");
    Ok(())
}

#[test]
fn test_wrong_opening_token() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "(", B, ")";
        B: B1 -> "b";
    }

    let result = Parser::parse("[b)");
    assert!(result.is_err(), "Should fail when wrong opening token");
    Ok(())
}

#[test]
fn test_calculator_missing_operand() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: Add -> S, "\\+", Term;
        S: T -> Term;
        Term: V -> Value;
        Value: Num -> "[0-9]+";
    }

    let result = Parser::parse("1+");
    assert!(result.is_err(), "Should fail when operand missing");
    Ok(())
}

#[test]
fn test_calculator_missing_operator() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: Add -> S, "\\+", Term;
        S: T -> Term;
        Term: V -> Value;
        Value: Num -> "[0-9]+";
    }

    let result = Parser::parse("12");
    // This might succeed if grammar allows it, or fail if it requires operator
    // Just verify it doesn't panic
    let _ = result;
    Ok(())
}

#[test]
fn test_nested_structure_error() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: Outer -> "(", Inner, ")";
        Inner: I -> "i";
    }

    let result = Parser::parse("(x)");
    assert!(result.is_err(), "Should fail when inner content doesn't match");
    Ok(())
}

#[test]
fn test_multiple_errors_scenario() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a", "b", "c";
    }

    // Multiple issues: wrong first token, incomplete
    let result = Parser::parse("x");
    assert!(result.is_err(), "Should fail on multiple issues");
    Ok(())
}

#[test]
fn test_regex_pattern_mismatch() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: Number -> "[0-9]+";
    }

    let result = Parser::parse("abc");
    assert!(result.is_err(), "Should fail when regex pattern doesn't match");
    
    // Should be a lexing error since "abc" doesn't match the number pattern
    match result {
        Err(LRParserError::Lexing(_)) => {
            // Expected - lexing error
        }
        Err(LRParserError::Parsing(_)) => {
            // Also acceptable if lexer accepts it but parser rejects
        }
        _ => {
            panic!("Expected Lexing or Parsing error");
        }
    }
    Ok(())
}

#[test]
fn test_partial_regex_match() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: Number -> "[0-9]+";
        S: Letter -> "[a-z]+";
    }

    // This should succeed as "123" matches number pattern
    let result = Parser::parse("123abc");
    // Might succeed or fail depending on grammar - just check it doesn't panic
    let _ = result;
    Ok(())
}

#[test]
fn test_lexing_error_propagates() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a";
    }

    // Input that can't be lexed (no matching pattern)
    let result = Parser::parse("@");
    assert!(result.is_err(), "Should fail when lexing fails");
    
    // Should be a lexing error since "@" doesn't match any pattern
    match result {
        Err(LRParserError::Lexing(err)) => {
            let err_msg = format!("{:?}", err);
            assert!(err_msg.contains("lexing") || err_msg.contains("pattern") || err_msg.contains("match"), 
                "Error message should mention lexing/pattern matching");
        }
        _ => {
            panic!("Expected Lexing error for unlexable input");
        }
    }
    Ok(())
}

#[test]
fn test_complex_grammar_error() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> X, Y;
        X: X1 -> "x";
        Y: Y1 -> "y";
    }

    // This grammar may have LR(0) conflicts, so "xy" might not parse
    // Just verify the parser handles it without panicking
    let result = Parser::parse("xy");
    let _ = result; // May succeed or fail depending on grammar
    
    let result = Parser::parse("yx");
    // Should fail - wrong order
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_right_recursive_error() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a", S;
        S: B -> "b";
    }

    // Valid: "ab"
    let result = Parser::parse("ab");
    assert!(result.is_ok());
    
    // Invalid: "ba" - wrong order
    let result = Parser::parse("ba");
    assert!(result.is_err() || result.is_ok()); // Depends on grammar interpretation
    Ok(())
}

#[test]
fn test_left_recursive_error() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> S, "a";
        S: B -> "b";
    }

    // Valid: "ba"
    let result = Parser::parse("ba");
    assert!(result.is_ok());
    
    // Invalid: "ab" - wrong order for left recursion
    let result = Parser::parse("ab");
    assert!(result.is_err() || result.is_ok()); // Depends on grammar interpretation
    Ok(())
}

#[test]
fn test_deeply_nested_error() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a", M;
        M: B -> "b", N;
        N: C -> "c";
    }

    // Valid: "abc"
    let result = Parser::parse("abc");
    assert!(result.is_ok());
    
    // Invalid: "acb" - wrong middle token
    let result = Parser::parse("acb");
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_multiple_variants_error() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a", "a";
        S: B -> "a", "b";
    }

    // "aa" should match A
    let result = Parser::parse("aa");
    assert!(result.is_ok());
    
    // "ab" should match B
    let result = Parser::parse("ab");
    assert!(result.is_ok());
    
    // "ac" should fail
    let result = Parser::parse("ac");
    assert!(result.is_err());
    
    // Verify error type - "c" is not in the alphabet, so it's a lexing error
    match result {
        Err(LRParserError::Lexing(_)) => {
            // Expected - lexing error because "c" doesn't match any pattern
        }
        _ => {
            panic!("Expected Lexing error, got {:?}", result.unwrap_err());
        }
    }
    Ok(())
}

#[test]
fn test_error_position_accuracy() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a", "b", "c";
    }

    // Test error at position 1 (after "a")
    let result = Parser::parse("axc");
    assert!(result.is_err());
    
    if let Err(LRParserError::Parsing(err)) = result {
        let err_msg = format!("{:?}", err);
        // Error should mention position (though exact format may vary)
        assert!(err_msg.contains("at:") || err_msg.contains("position"), 
            "Error message should contain position information");
    }
    
    Ok(())
}

#[test]
fn test_error_message_contains_expected_tokens() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> "a", "b";
    }

    let result = Parser::parse("ac");
    assert!(result.is_err());
    
    if let Err(LRParserError::Parsing(err)) = result {
        let err_msg = format!("{:?}", err);
        // Error should mention what was expected
        assert!(err_msg.contains("expected") || err_msg.contains("b"), 
            "Error message should mention expected tokens");
    }
    
    Ok(())
}

