use std::any::Any;
use std::error::Error;

use common::VariantId;
use macros::build_parser;

build_parser! {
    S: A -> "a", A;
    A: X -> "x";
}

#[test]
fn build_parser_working() -> Result<(), Box<dyn Error>> {
    let id = VariantId::new("S".to_string(), "A".to_string(), 2);
    
    let children: Vec<Box<dyn Any>> = vec![
        Box::new(A::X(Box::new("x".to_string()))),
        Box::new("a".to_string())
    ];

    let r = Table::build_rule(id, children);
    assert!(r.is_some(), "build_rule should return Some");

    let downcast = r.unwrap().downcast::<S>();
    assert!(downcast.is_ok(), "downcast to S should succeed");

    let res = Box::new(S::A(
        Box::new("a".to_string()),
        Box::new(A::X(Box::new("x".to_string())))
    ));
    assert_eq!(*downcast.unwrap(), *res);

    Ok(())
}