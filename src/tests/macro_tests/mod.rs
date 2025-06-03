#[cfg(test)]
use macros::build_parser;

#[cfg(test)]
build_parser! {
    #S: A -> "a", B;
    #S: B -> "b", A;
    A: A -> "a";
    B: C -> "c";
    B: B -> "b";
}