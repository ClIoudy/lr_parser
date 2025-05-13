use crate::{IdTrait, Token};

#[derive(Debug, Clone)]
pub struct Ast<'a, T: IdTrait> {
    pub value: Token<'a, T>,
    pub children: Vec<Ast<'a, T>>,
}

impl<'a, T: IdTrait> Ast<'a, T> {
    pub fn new(value: Token<'a, T>) -> Self {
        Self {
            value,
            children: Vec::new(),
        }
    }

    pub fn with_children(value: Token<'a, T>, children: Vec<Ast<'a, T>>) -> Self {
        Self {
            value,
            children,
        }
    }

    pub fn add_child(&mut self, child: Ast<'a, T>) {
        self.children.push(child);
    }

    pub fn pop_checked(&mut self) -> Option<Ast<T>> {
        self.children.pop()
    }

    pub fn pop(&mut self) -> Ast<T> {
        self.children.pop().unwrap()
    }
}