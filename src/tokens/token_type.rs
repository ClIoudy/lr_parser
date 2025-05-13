use crate::IdTrait;

pub struct TokenType<T: IdTrait> {
    id: T,
}

impl<T: IdTrait> TokenType<T> {
    pub fn match_start(&self, haystack: &str) -> Option<&str> {
        None
    }

    pub fn id(&self) -> &T {
        &self.id
    }
}