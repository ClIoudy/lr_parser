use common::Terminal;

pub enum Token {
    EOF,
    Value(&'static str),
}

impl Token {
    pub fn id(&self) -> Terminal {
        match self {
            Self::EOF => Terminal::EOF,
            Self::Value(x) => Terminal::Value(x.to_string()),
        };

        todo!()
    }

    
}

