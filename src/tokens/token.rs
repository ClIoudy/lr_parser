use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Token {
    pub value: String,
    pub label: Option<String>,
    is_eof: bool,
}

impl Token {
    pub fn new(value: String, label: Option<String>) -> Self {
        Self {
            value,
            label,
            is_eof: false,
        }
    }

    pub fn eof() -> Self {
        Self {
            value: String::new(),
            label: Some("$".to_string()),
            is_eof: true,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.value.len()
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        Self::new(value.to_string(), None)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        if self.is_eof {
            return f.write_str("eof");
        }

        f.write_str(&format!("\"{}\"", self.value))?;

        if let Some(label) = &self.label {
            f.write_str(&format!(" ({})", label))?;
        }

        Ok(())
    }
}

impl PartialEq<&str> for Token {
    fn eq(&self, other: &&str) -> bool {
        self.value == other.to_string()
    }
}