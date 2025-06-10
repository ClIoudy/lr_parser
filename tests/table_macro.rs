use std::{any::Any, fmt::Debug};

use common::{TableTrait, VariantId};
use macros::build_parser;

build_parser! {
    S: A -> "a", A;
    A: X -> "x";
}

#[test]
fn build_parser_working() {
    let id = VariantId::new("S".to_string(), "A".to_string(), 2);
    
    let mut children: Vec<Box<dyn Any>> = vec![Box::new("a".to_string()), Box::new(A::X(Box::new("x".to_string())))];
    children.reverse();

    let r = Table::build_rule(id, children);
    assert!(r.is_some());

    let downcast = r.unwrap().downcast::<S>();
    assert!(downcast.is_ok());

    let res = Box::new(S::A(Box::new("a".to_string()), Box::new(A::X(Box::new("x".to_string())))));
    assert_eq!(downcast.unwrap(), res);
}