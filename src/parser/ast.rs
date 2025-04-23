use crate::tokens::Token;

#[derive(Debug, Clone)]
pub struct Ast {
    pub value: Token,
    pub children: Vec<Ast>,
}

impl Ast {
    pub fn new(value: Token) -> Self {
        Self {
            value,
            children: Vec::new(),
        }
    }

    pub fn with_children(value: Token, children: Vec<Ast>) -> Self {
        Self {
            value,
            children,
        }
    }

    pub fn add_child(&mut self, child: Ast) {
        self.children.push(child);
    }

    pub fn pop_checked(&mut self) -> Option<Ast> {
        self.children.pop()
    }

    pub fn pop(&mut self) -> Ast {
        self.children.pop().unwrap()
    }
}