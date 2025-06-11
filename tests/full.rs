use std::error::Error;

use lr_parser::{lexer::Lexer, Parser};
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

    let parser = Parser::<Table>::new();

    let parse = parser.parse(lex)?;

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

    let mut lexer = Lexer::empty();
    lexer.try_add("a")?;
    lexer.try_add("b")?;

    let parser = Parser::<Table>::new();
    let parse = parser.parse(lexer.lex("baa")?)?;
    let a = Box::new("a".to_string());
    let b = Box::new("b".to_string());
    
    let s_a = |s| S::A(Box::new(s), a.clone());
    let res = s_a(s_a(S::B(b)));

    assert_eq!(*parse, res);

    Ok(())
}

#[test]
pub fn calculator() -> Result<(), Box<dyn Error>> {
    
    
    let mut lexer = Lexer::empty();

    lexer.try_add("+")?;
    lexer.try_add("-")?;
    lexer.try_add("*")?;
    lexer.try_add("/")?;

    

    Ok(())
}