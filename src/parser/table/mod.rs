use std::collections::HashMap;

use crate::{vec_into, Grammar, Token, TokenIdent};

mod builder;
pub use builder::TableBuilder;
mod state;
pub use state::StateElement;
mod action;
pub use action::Action;

pub type State = Vec<StateElement>;

#[derive(Debug)]
pub struct Table {
    grammar: Grammar,
    actions: HashMap<State, HashMap<TokenIdent, Action>>,
    start_state: State,
    states: HashMap<State, usize>,
}

impl Table {
    pub fn new(actions: HashMap<State, HashMap<TokenIdent, Action>>, start_state: State, grammar: Grammar, states: HashMap<State, usize>) -> Self {
        Self {
            grammar,
            actions,
            start_state,
            states,
        }
    }

    pub fn start_state(&self) -> &State {
        &self.start_state
    }

    fn action(&self, from: &State, key: &TokenIdent) -> Option<&Action> {
        self.actions
        .get(from)?
        .get(key)
    }

    pub fn reduction(&self, from: &State, key: &TokenIdent) -> Option<&StateElement> {
        match self.action(from, key)? {
            Action::Reduce(item) => Some(item),
            _ => None
        }
    }

    pub fn transition(&self, from: &State, key: &TokenIdent) -> Option<&State> {
        match self.action(from, key)? {
            Action::Transition(s) => Some(s),
            _ => None
        }
    }

    pub fn empty(&self, from: &State, key: &TokenIdent) -> bool {
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

    pub fn keys(&self, from: &State) -> Vec<TokenIdent> {
        // self.transitions.get(from).unwrap().keys().cloned().collect()
        self.actions.get(from).unwrap().keys().cloned().collect()
    }
    
    pub fn print_table(&self, non_terminal_names: Vec<impl Into<TokenIdent>>) {

        let mut s = self.states.iter().collect::<Vec<_>>();
        s.sort_by_key(|(x, index)| *index);
        s.iter().for_each(|(state, index)| println!("state {index}: {state:?}"));
        
        println!();
        println!();

        let mut columns = vec![];
        
        for (_, row) in self.actions.values().enumerate() {
            let column = row.iter().map(|(token, action)| {
                let a = match action {
                    Action::Reduce(_) => "r ".to_string(),
                    // Action::Goto(state) => format!("g{}", states.get(state).unwrap()),
                    // Action::Shift(state) => format!("s{}", states.get(state).unwrap()),
                    Action::Transition(state) => format!("t{:03}", self.states.get(state).unwrap()),
                };

                (token, a)
            }).collect::<HashMap<&TokenIdent, String>>();

            columns.push(column);
        }

        // let mut tokens = ["1", "2", "3", "7", "+", "-", "*", "/"].into_iter().map(|x| x.into()).collect::<Vec<_>>();
        let mut tokens = vec_into(non_terminal_names);
        tokens.push(Token::eof().into());

        print!("   | ");
        for token in tokens.clone() {
            print!("{:015} | ", format!("{:?}", token));
        }


        for i in 0..columns.len() {
            print!(" {:^3} | ", i.to_string());
            for token in &tokens {
                print!("{:010} | ", format!("{:?}", columns[i].get(token).unwrap_or(&"  ".to_string())));
            }
            println!();
        }
    }

    pub fn start_symbol(&self) -> &Token {
        self.grammar.start_symbol()
    }
}