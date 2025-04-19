pub mod parser;
pub mod lexer;
pub mod tokens;
pub(crate) use tokens::*;

pub mod grammar;
pub(crate) use grammar::*;


fn vec_into<T>(v: Vec<impl Into<T>>) -> Vec<T> {
    v.into_iter().map(|x| x.into()).collect()
}