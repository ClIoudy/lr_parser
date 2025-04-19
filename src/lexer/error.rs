use std::fmt::Display;

#[derive(Debug)]
pub struct LexingError {
    message: String
}

impl Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_str(&format!("{:?}", self))

        f.write_str(&format!("Error while lexing: {}", self.message))
    }
}

impl std::error::Error for LexingError { }

impl LexingError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

