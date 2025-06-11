use common::Terminal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    EOF,
    Value { label: String, value: String }
}

impl Token {
    pub fn labeld(label: String, value: String) -> Self {
        Self::Value { label, value }
    }

    pub fn eof() -> Self {
        Self::EOF
    }

    pub fn id(&self) -> Terminal {
        match self {
            Self::EOF => Terminal::EOF,
            Self::Value { label, value } => Terminal::Labeld(label.clone())
        }
    }
}

