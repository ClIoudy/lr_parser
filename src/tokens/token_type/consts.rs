use super::TokenType;

impl TokenType {
    pub fn numbers() -> Self {
        Self::labeld("[0-9]+", "numbers")
    }

    pub fn lowercase() -> Self {
        Self::new("[a-z]+")
    }

    pub fn uppercase() -> Self {
        Self::new("[A-Z]+")
    }

    pub fn letter() -> Self {
        Self::new("[a-zA-Z]+")
    }
    
    /// accepts any combination of numbers and letters
    /// example: Foo2bAr
    pub fn alphanumerical() -> Self {
        Self::new("[a-zA-Z0-9]+")
    }
}