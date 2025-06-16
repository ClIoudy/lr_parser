use std::error::Error;
use macros::build_parser;

// simple ac*d
#[test]
pub fn simple() -> Result<(), Box<dyn Error>> {

    build_parser! {
        S: A -> "a", B;
        B: C -> "c", B;
        B: D -> "d";
    }

    let parse = Parser::parse("accd")?;
    
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

    let parse = Parser::parse("baa")?;

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
        S: Add -> S, "\\+", Term;
        S: Sub -> S, "-", Term;
        S: T -> Term;
        Term: V -> Value;
        Term: Mul -> Term, "\\*", Value;
        Term: Div -> Term, "/", Value;
        Value: Num -> "[0-9]*";
    }

    Parser::parse("caba")?;

    Ok(())
}

#[test]
fn regex_test() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: Number -> "[0-9]*";
    }

    let parse = Parser::parse("123")?;

    assert_eq!(*parse, S::Number(Box::new("123".to_string())));

    Ok(())

}

#[test]
fn calculator_grammar() -> Result<(), Box<dyn Error>> {
    build_parser! {
        S: Add -> S, "\\+", Term;
        S: Sub -> S, "-", Term;
        S: T -> Term;
        Term: V -> Value;
        Term: Mul -> Term, "\\*", Value;
        Term: Div -> Term, "/", Value;
        Value: Num -> "[0-9]*";
    }
    
    let expr = "1+2*3";
    Parser::parse(expr)?;

    Ok(())
}