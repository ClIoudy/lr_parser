use std::error::Error;

use crate::{lexer::Lexer, Token};

#[test]
fn test_lexer() -> Result<(), Box<dyn Error>>{
    let lexer = Lexer::from_alphabet(["[0-9]*", "[a-z]*"])?;


    let sample = "abc901a";
    let lex = lexer.lex(sample)?;

    let lowercase_label = "[a-z]*".to_string();
    let num_label = "[0-9]*".to_string();

    assert_eq!(lex, vec![
        Token::Value { label: lowercase_label.clone(), value: "abc".to_string() }, 
        Token::Value { label: num_label, value: "901".to_string() }, 
        Token::Value { label: lowercase_label, value: "a".to_string() }
    ]);

    Ok(())
}