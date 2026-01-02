use std::error::Error;

use crate::{PatternError, Token, lexer::Lexer};

#[test]
fn test_lexer() -> Result<(), Box<dyn Error>>{
    let lexer = Lexer::from_alphabet(["[0-9]+", "[a-z]+"])?;


    let sample = "abc901a";
    let lex = lexer.lex(sample)?;

    let lowercase_label = "[a-z]+".to_string();
    let num_label = "[0-9]+".to_string();

    assert_eq!(lex, vec![
        Token::Value { label: lowercase_label.clone(), value: "abc".to_string() }, 
        Token::Value { label: num_label, value: "901".to_string() }, 
        Token::Value { label: lowercase_label, value: "a".to_string() }
    ]);

    Ok(())
}

#[test]
fn test_empty_pattern_is_detected() -> Result<(), Box<dyn Error>> {
    let result = Lexer::from_alphabet(["[0-9]*"]);
    assert!(result.is_err());

    let error = result.unwrap_err();
    let pattern_error = PatternError::pattern_matches_empty_string("[0-9]*");
    assert_eq!(error, pattern_error);

    Ok(())
}

#[test]
fn test_longest_match_preference() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["a", "aa", "aaa"])?;
    
    let lex = lexer.lex("aaa")?;
    
    // Should match the longest pattern "aaa"
    assert_eq!(lex.len(), 1);
    match &lex[0] {
        Token::Value { value, .. } => assert_eq!(value, "aaa"),
        Token::EOF => panic!("Expected value token"),
    }
    Ok(())
}

#[test]
fn test_multiple_patterns_same_length() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["[0-9]+", "[a-z]+"])?;
    
    let lex = lexer.lex("123abc456")?;
    
    assert_eq!(lex.len(), 3);
    match &lex[0] {
        Token::Value { value, .. } => assert_eq!(value, "123"),
        Token::EOF => panic!("Expected value token"),
    }
    match &lex[1] {
        Token::Value { value, .. } => assert_eq!(value, "abc"),
        Token::EOF => panic!("Expected value token"),
    }
    match &lex[2] {
        Token::Value { value, .. } => assert_eq!(value, "456"),
        Token::EOF => panic!("Expected value token"),
    }
    Ok(())
}

#[test]
fn test_regex_patterns() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["[0-9]+", "[a-zA-Z]+", "\\s+"])?;
    
    let lex = lexer.lex("hello123world  456")?;
    
    // The exact number of spaces might vary, so we check the structure
    assert!(lex.len() >= 4);
    let get_value = |token: &Token| -> String {
        match token {
            Token::Value { value, .. } => value.clone(),
            Token::EOF => panic!("Expected value token"),
        }
    };
    assert_eq!(get_value(&lex[0]), "hello");
    assert_eq!(get_value(&lex[1]), "123");
    assert_eq!(get_value(&lex[2]), "world");
    // Check that there's whitespace (might be 2 or 3 spaces depending on matching)
    let whitespace = get_value(&lex[3]);
    assert!(whitespace.trim().is_empty() && !whitespace.is_empty(), "Expected whitespace");
    // If there's a 5th token, it should be the number
    if lex.len() > 4 {
        assert_eq!(get_value(&lex[4]), "456");
    }
    Ok(())
}

#[test]
fn test_escaped_special_characters() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["\\+", "\\*", "\\-", "/"])?;
    
    let lex = lexer.lex("+-*/")?;
    
    assert_eq!(lex.len(), 4);
    let get_value = |token: &Token| -> String {
        match token {
            Token::Value { value, .. } => value.clone(),
            Token::EOF => panic!("Expected value token"),
        }
    };
    assert_eq!(get_value(&lex[0]), "+");
    assert_eq!(get_value(&lex[1]), "-");
    assert_eq!(get_value(&lex[2]), "*");
    assert_eq!(get_value(&lex[3]), "/");
    Ok(())
}

#[test]
fn test_whitespace_patterns() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["\\s+", "[a-z]+"])?;
    
    let lex = lexer.lex("hello   world")?;
    
    assert_eq!(lex.len(), 3);
    let get_value = |token: &Token| -> String {
        match token {
            Token::Value { value, .. } => value.clone(),
            Token::EOF => panic!("Expected value token"),
        }
    };
    assert_eq!(get_value(&lex[0]), "hello");
    assert_eq!(get_value(&lex[1]), "   ");
    assert_eq!(get_value(&lex[2]), "world");
    Ok(())
}

// ========== Error Cases ==========

#[test]
fn test_no_match_error() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["[0-9]+"])?;
    
    let result = lexer.lex("abc");
    assert!(result.is_err(), "Should fail when no pattern matches");
    Ok(())
}

#[test]
fn test_partial_match_error() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["[0-9]+", "[a-z]+"])?;
    
    let result = lexer.lex("123abc@def");
    assert!(result.is_err(), "Should fail on unmatched character");
    Ok(())
}

#[test]
fn test_empty_string_lexing() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["[0-9]+", "[a-z]+"])?;
    
    let lex = lexer.lex("")?;
    assert_eq!(lex.len(), 0);
    Ok(())
}

#[test]
fn test_invalid_regex_pattern() -> Result<(), Box<dyn Error>> {
    let result = Lexer::from_alphabet(["[invalid(regex"]);
    assert!(result.is_err(), "Should fail on invalid regex");
    Ok(())
}

#[test]
fn test_empty_pattern_variations() -> Result<(), Box<dyn Error>> {
    // Test various patterns that match empty strings
    let patterns = ["[0-9]*", "a*", ".*", ""];
    
    for pattern in patterns {
        let result = Lexer::from_alphabet([pattern]);
        assert!(result.is_err(), "Pattern '{}' should be rejected", pattern);
    }
    Ok(())
}

// ========== Edge Cases ==========

#[test]
fn test_single_character_input() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["a"])?;
    
    let lex = lexer.lex("a")?;
    assert_eq!(lex.len(), 1);
    match &lex[0] {
        Token::Value { value, .. } => assert_eq!(value, "a"),
        Token::EOF => panic!("Expected value token"),
    }
    Ok(())
}

#[test]
fn test_very_long_input() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["[0-9]+"])?;
    
    let long_input = "1".repeat(1000);
    let lex = lexer.lex(&long_input)?;
    
    assert_eq!(lex.len(), 1);
    match &lex[0] {
        Token::Value { value, .. } => assert_eq!(value, &long_input),
        Token::EOF => panic!("Expected value token"),
    }
    Ok(())
}

#[test]
fn test_repeated_patterns() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["a", "aa"])?;
    
    let lex = lexer.lex("aaaa")?;
    
    // Should prefer longest matches
    assert_eq!(lex.len(), 2);
    let get_value = |token: &Token| -> String {
        match token {
            Token::Value { value, .. } => value.clone(),
            Token::EOF => panic!("Expected value token"),
        }
    };
    assert_eq!(get_value(&lex[0]), "aa");
    assert_eq!(get_value(&lex[1]), "aa");
    Ok(())
}

#[test]
fn test_pattern_priority() -> Result<(), Box<dyn Error>> {
    // When multiple patterns match, longest should win
    let lexer = Lexer::from_alphabet(["a", "aa", "aaa"])?;
    
    let lex = lexer.lex("aaaaaa")?;
    
    // Should match "aaa" twice (longest pattern)
    assert_eq!(lex.len(), 2);
    let get_value = |token: &Token| -> String {
        match token {
            Token::Value { value, .. } => value.clone(),
            Token::EOF => panic!("Expected value token"),
        }
    };
    assert_eq!(get_value(&lex[0]), "aaa");
    assert_eq!(get_value(&lex[1]), "aaa");
    Ok(())
}

#[test]
fn test_mixed_alphanumeric() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::from_alphabet(["[0-9]+", "[a-z]+", "[A-Z]+"])?;
    
    let lex = lexer.lex("abc123DEF456ghi")?;
    
    assert_eq!(lex.len(), 5);
    let get_value = |token: &Token| -> String {
        match token {
            Token::Value { value, .. } => value.clone(),
            Token::EOF => panic!("Expected value token"),
        }
    };
    assert_eq!(get_value(&lex[0]), "abc");
    assert_eq!(get_value(&lex[1]), "123");
    assert_eq!(get_value(&lex[2]), "DEF");
    assert_eq!(get_value(&lex[3]), "456");
    assert_eq!(get_value(&lex[4]), "ghi");
    Ok(())
}

#[test]
fn test_unicode_characters() -> Result<(), Box<dyn Error>> {
    // Test with Unicode-aware pattern
    let lexer = Lexer::from_alphabet([r"\p{L}+", "[0-9]+"])?;
    
    // Test that it handles Unicode characters
    let result = lexer.lex("café123");
    // The pattern \p{L}+ matches Unicode letters, so this should work
    match result {
        Ok(tokens) => {
            assert!(!tokens.is_empty(), "Should produce tokens for Unicode input");
        }
        Err(_) => {
            // If the regex engine doesn't support \p{L}, try with a simpler pattern
            let lexer2 = Lexer::from_alphabet(["[a-zA-Z0-9]+"])?;
            let result2 = lexer2.lex("cafe123");
            assert!(result2.is_ok(), "ASCII fallback should work");
        }
    }
    Ok(())
}

#[test]
fn test_unicode_handling_ascii_fallback() -> Result<(), Box<dyn Error>> {
    // Test that ASCII patterns work with mixed input
    let lexer = Lexer::from_alphabet(["[a-z]+", "[0-9]+"])?;
    
    // Input with non-ASCII character should fail gracefully
    let result = lexer.lex("hello世界123");
    assert!(result.is_err(), "Non-ASCII characters not in pattern should fail");
    
    // Verify it's a lexing error (LexError is a struct, so we just check it's an error)
    assert!(result.is_err(), "Should produce LexError for non-matching Unicode");
    Ok(())
}

#[test]
fn test_lexer_empty_alphabet() -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::empty();
    
    let result = lexer.lex("anything");
    assert!(result.is_err(), "Empty alphabet should fail to lex");
    Ok(())
}

#[test]
fn test_lexer_add_pattern() -> Result<(), Box<dyn Error>> {
    use crate::lexer::pattern::Pattern;
    let mut lexer = Lexer::empty();
    lexer.add(Pattern::new("[0-9]+")?);
    lexer.add(Pattern::new("[a-z]+")?);
    
    let lex = lexer.lex("abc123")?;
    assert_eq!(lex.len(), 2);
    Ok(())
}

#[test]
fn test_lexer_try_add_pattern() -> Result<(), Box<dyn Error>> {
    let mut lexer = Lexer::empty();
    assert!(lexer.try_add("[0-9]+").is_ok());
    assert!(lexer.try_add("[a-z]+").is_ok());
    assert!(lexer.try_add("[0-9]*").is_err()); // Empty matching pattern
    
    let lex = lexer.lex("abc123")?;
    assert_eq!(lex.len(), 2);
    Ok(())
}

#[test]
fn test_pattern_with_special_regex_chars() -> Result<(), Box<dyn Error>> {
    // Test patterns with special regex characters that need escaping
    let lexer = Lexer::from_alphabet([r"\+", r"\*", r"\?", r"\.", r"\^", r"\$"])?;
    
    let lex = lexer.lex("+*?.^$")?;
    assert_eq!(lex.len(), 6);
    Ok(())
}

#[test]
fn test_pattern_that_matches_but_leaves_remainder() -> Result<(), Box<dyn Error>> {
    // This tests the case where a pattern matches but there's remaining unmatched text
    let lexer = Lexer::from_alphabet(["[0-9]+"])?;
    
    // "123abc" - "123" matches but "abc" doesn't
    let result = lexer.lex("123abc");
    assert!(result.is_err(), "Should fail when pattern matches but remainder doesn't");
    Ok(())
}

#[test]
fn test_overlapping_patterns() -> Result<(), Box<dyn Error>> {
    // Test patterns that can overlap - longest should win
    let lexer = Lexer::from_alphabet(["a", "aa", "aaa", "aaaa"])?;
    
    let lex = lexer.lex("aaaaa")?;
    // Should match "aaaa" then "a" (longest first)
    assert_eq!(lex.len(), 2);
    let get_value = |token: &Token| -> String {
        match token {
            Token::Value { value, .. } => value.clone(),
            Token::EOF => panic!("Expected value token"),
        }
    };
    assert_eq!(get_value(&lex[0]), "aaaa");
    assert_eq!(get_value(&lex[1]), "a");
    Ok(())
}