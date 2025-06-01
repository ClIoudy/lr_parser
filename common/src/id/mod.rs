pub struct RuleId {

}

pub enum TokenId<'a> {
    EOF,
    Value(&'a str),
}

pub enum Id<'a> {
    Rule(RuleId),
    Token(TokenId<'a>),
}