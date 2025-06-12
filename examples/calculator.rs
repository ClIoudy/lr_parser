use std::error::Error;

use macros::build_parser;

build_parser! {
    S: Add -> S, "+", Term;
    S: Sub -> S, "-", Term;
    Term: Mul -> Term, "*", Value;
    Term: Div -> Term, "/", Value;
    Value: Num -> "[0-9]*";
}

fn main() -> Result<(), Box<dyn Error>> {

    

    Ok(())

}