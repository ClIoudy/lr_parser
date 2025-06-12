use std::error::Error;

use lr_parser::{lexer::Lexer};
use macros::build_parser;

// simple ac*d
#[test]
pub fn simple() -> Result<(), Box<dyn Error>> {

    build_parser! {
        S: A -> "a", B;
        B: C -> "c", B;
        B: D -> "d";
    }

    let mut lexer = Lexer::empty();
    lexer.try_add("a")?;
    lexer.try_add("c")?;
    lexer.try_add("d")?;

    let haystack = "accd";

    let lex = lexer.lex(haystack)?;

    let parse = Parser::parse(lex)?;

    let res = S::A(
        Box::new("a".to_string()),
        Box::new(B::C(
            Box::new("c".to_string()),
            Box::new(B::C(
                Box::new("c".to_string()),
                Box::new(B::D(Box::new("d".to_string()))),
            )),
        )),
    );

    assert_eq!(res, *parse);

    Ok(())
}

#[test]
pub fn recursive_on_first_value() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: A -> S, "a";
        S: B -> "b";
    }

    let lexer = Lexer::from_alphabet(Table::alphabet())?;
    let lex = lexer.lex("baa")?;
    let parse = Parser::parse(lex)?;

    let a = Box::new("a".to_string());
    let b = Box::new("b".to_string());
    
    let s_a = |s| S::A(Box::new(s), a.clone());
    let res = s_a(s_a(S::B(b)));

    assert_eq!(*parse, res);

    Ok(())
}

#[test]
pub fn recursive_first_with_multiple_choices() -> Result<(), Box<dyn Error>> {

    build_parser! {
        S: A -> S, "a";
        S: B -> S, "b";
        S: C -> "c";
    }

    let lexer = Lexer::from_alphabet(Table::alphabet())?;
    Parser::parse(lexer.lex("caba")?)?;

    Ok(())
}

#[test]
fn regex_test() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: Number -> "[0-9]*";
    }

    let lexer = Lexer::from_alphabet(Table::alphabet())?;
    let parse = Parser::parse(lexer.lex("123")?)?;

    assert_eq!(*parse, S::Number(Box::new("123".to_string())));

    Ok(())

}