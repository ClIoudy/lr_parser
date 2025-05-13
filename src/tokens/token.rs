use crate::IdTrait;

#[derive(Debug, Clone)]
pub struct Token<'a, T: IdTrait> {
    id: Option<&'a T>,
    value: &'a str,
    is_eof: bool,
}

impl<'a, T: IdTrait> Token<'a, T> {
    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn new(id: &'a T, value: &'a str) -> Self {
        Self {
            id: Some(id),
            value,
            is_eof: false,
        }
    }

    pub fn eof() -> Self {
        Self {
            id: None,
            value: "",
            is_eof: true,

        }
    }

    pub fn is_eof(&self) -> bool {
        self.is_eof
    }
}