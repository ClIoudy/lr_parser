use common::TokenId;

pub enum Token {
    EOF,
    Value(&'static str),
}

impl Token {
    pub fn id(&self) -> TokenId {
        match self {
            Self::EOF => TokenId::EOF,
            Self::Value(x) => TokenId::Value(x),
        };

        todo!()
    }

    
}

