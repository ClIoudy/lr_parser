use std::error::Error;

use macros::build_parser;

build_parser! {
    // '\\' needed in order to escape the '+' regex definition and just use '+' as a literal
    S: Add -> S, "\\+", Term;
    S: Sub -> S, "-", Term;
    S: T -> Term;
    Term: V -> Value;
    Term: Mul -> Term, "\\*", Value;
    Term: Div -> Term, "/", Value;
    Value: Num -> "[0-9]*";
}

fn main() -> Result<(), Box<dyn Error>> {

    let mut expr = String::new();

    println!("Please enter a mathmatical expression.");

    std::io::stdin().read_line(&mut expr)?;

    let expr = expr.replace(" ", "");
    let expr = expr.trim();

    println!("Parsing expression: '{}'", expr);
    
    let parse = Parser::parse(&expr)?;

    println!("Parsing finished: {:?}", parse);

    

    Ok(())
}