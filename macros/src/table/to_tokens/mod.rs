mod expected;
mod quotable;

// use common::{NonTerminal, RuleTrait, TableTrait};

// enum S {
//     // "a", B
//     A(Box<String>, Box<B>),
// }

// impl RuleTrait for S {
//     fn id(&self) -> common::NonTerminal {
//         NonTerminal::new("S".to_string())
//     }
// }

// enum B {
//     C(String),
//     D(String),
// }

// impl RuleTrait for B {
//     fn id(&self) -> common::NonTerminal {
//         NonTerminal::new("B".to_string())
//     }
// }

// struct Table {

// }

// impl TableTrait for Table {
//     fn action(&self, state: &common::StateId, token: &common::Id) -> Option<common::Action> {
//         match state {
//             0 => match token {
                
//             }
//         }
//     }
// }