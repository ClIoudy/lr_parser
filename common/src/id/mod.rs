use std::fmt::Debug;

mod non_terminal;
pub use non_terminal::NonTerminal;

mod terminal;
pub use terminal::Terminal;




#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Id {
    N(NonTerminal),
    T(Terminal),
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Id::T(x) => x.fmt(f),
            Id::N(x) => x.fmt(f),
        }
    }
}
