use std::{fmt::Debug, hash::Hash};


pub trait IdTrait: Hash + Clone + PartialEq + Eq + Debug + 'static {
    
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Id<RuleId: IdTrait, TokenId: IdTrait> {
    Token(TokenId),
    Rule(RuleId),
}