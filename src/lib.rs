pub mod parser;
pub mod lexer;
pub mod ids;
pub(crate) use ids::*;
mod tokens;
pub use tokens::*;
pub mod grammar;
pub(crate) use grammar::*;


pub fn vec_into<T>(v: Vec<impl Into<T>>) -> Vec<T> {
    v.into_iter().map(|x| x.into()).collect()
}