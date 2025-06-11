#![allow(unused)]

mod parser;
pub use parser::Parser;

mod tokens;

pub(crate) use tokens::Token;
pub(crate) use tokens::*;

pub use common::*;
pub use macros::build_parser;

pub mod lexer;

#[cfg(test)]
mod tests;

struct ABC<T> {
    x: T,
}

enum S {
    A(Box<A>, String),
    S(Box<S>, String)
}

enum A {
    A(String),
}



// return: Result<S>
// id: S_A, S_S, A_A
// tokens: ids (included in grammar syntax?)
//         eof
// token ids/labels as strings?
// in grammar syntax is "x" for just "x" and labled "x" for labeld ones? 
// or just always use regex?
// table
// - indexed by id (token AND rule ids possible)
// 






