use std::collections::HashMap;

use crate::Token;

mod builder;
pub use builder::TableBuilder;
mod state;
pub use state::StateElement;
mod action;
pub use action::Action;

pub type State = Vec<StateElement>;

#[derive(Debug)]
pub struct Table {
    // transitions: HashMap<State, HashMap<Token, State>>,
    // reductions: HashMap<State, HashMap<Token, StateElement>>,
    actions: HashMap<State, HashMap<Token, Action>>,
    start_state: State,
}

impl Table {
    pub fn new(actions: HashMap<State, HashMap<Token, Action>>, start_state: State) -> Self {
        Self {
            actions,
            start_state
        }
    }

    pub fn start_state(&self) -> &State {
        &self.start_state
    }

    fn action(&self, from: &State, key: &Token) -> Option<&Action> {
        self.actions
        .get(from)?
        .get(key)
    }

    pub fn reduction(&self, from: &State, key: &Token) -> Option<&StateElement> {
        match self.action(from, key)? {
            Action::Reduce(item) => Some(item),
            _ => None
        }
    }

    pub fn transition(&self, from: &State, key: &Token) -> Option<&State> {
        match self.action(from, key)? {
            Action::Transition(s) => Some(s),
            _ => None
        }
    }

    pub fn empty(&self, from: &State, key: &Token) -> bool {
        self.action(from, key).is_none()
    }

    // pub fn transition(&self, from: &State, key: &Token) -> Option<&State> {
    //     self.transitions
    //     .get(from)?
    //     .get(key)
    // }

    // pub fn reduction(&self, from: &State, key: &Token) -> Option<&StateElement> {
    //     self.reductions
    //     .get(from)?
    //     .get(key)
    // }

    pub fn keys(&self, from: &State) -> Vec<Token> {
        // self.transitions.get(from).unwrap().keys().cloned().collect()
        self.actions.get(from).unwrap().keys().cloned().collect()
    }

    
    // pub fn print_table(actions: &HashMap<State, HashMap<Token, Action>>, states: HashMap<State, usize>) {
        // {
        //     let mut s = states.iter().collect::<Vec<_>>();
        //     s.sort_by_key(|(x, index)| *index);
        //     s.iter().for_each(|(state, index)| println!("state {index}: {state:?}"));
        // }

        // println!();
        // println!();

        // let mut columns = vec![];
        
        // for (_, row) in actions.values().enumerate() {
        //     let column = row.iter().map(|(token, action)| {
        //         let a = match action {
        //             Action::Reduce(_) => "r ".to_string(),
        //             Action::Goto(state) => format!("g{}", states.get(state).unwrap()),
        //             Action::Shift(state) => format!("s{}", states.get(state).unwrap()),
        //         };

        //         (token, a)
        //     }).collect::<HashMap<&Token, String>>();

        //     columns.push(column);
        // }

        // let mut tokens = ["1", "2", "3", "7", "+", "-", "*", "/"].into_iter().map(|x| x.into()).collect::<Vec<Token>>();
        // tokens.push(Token::eof());

        // print!("   | ");
        // for token in tokens.clone() {
        //     print!("{:?}  | ", token);
        // }

        // println!();
        // println!("{}", "   |   ".repeat(tokens.len() + 1));
        // for i in 0..columns.len() {
        //     print!(" {i} | ");
        //     for token in &tokens {
        //         print!("{:?} | ", columns[i].get(token).unwrap_or(&"  ".to_string()));
        //     }
        //     println!();
        // }
    // }
}
