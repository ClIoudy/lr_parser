use std::{
    collections::{HashMap, HashSet},
    hash::Hash, process::exit,
};

use crate::{Rule, Token, TokenIdent};

use super::{state::StateElement, Action, State};

use super::Table;
use crate::Grammar;

pub struct TableBuilder {
    grammar: Grammar,
    start_symbol: Token,
    closures: HashMap<Token, Vec<StateElement>>,
    actions: HashMap<State, HashMap<TokenIdent, Action>>,
    follows: HashMap<Token, HashSet<TokenIdent>>,
    states: HashMap<State, usize>,
}

impl TableBuilder {
    pub fn new(mut grammar: Grammar) -> Self {
        let start_symbol = grammar.start_symbol().clone();
        if !grammar.rules().contains_key(&start_symbol) {
            panic!("Grammar has to contain start symbol");
        }

        let mut iter = grammar.rules_mut().remove(&start_symbol).unwrap().into_iter();

        let mut set = HashSet::with_capacity(iter.len());

        for mut v in &mut iter {
            v.values_mut().push(Token::eof().into());
            set.insert(v);
        }

        grammar.rules_mut().insert(start_symbol.clone(), set);

        Self {
            grammar,
            start_symbol,
            closures: HashMap::new(),
            actions: HashMap::new(),
            follows: HashMap::new(),
            states: HashMap::new(),
        }
    }

    /// determines the follow set of a given token in the grammar.
    fn follow(&mut self, token: &Token) -> HashSet<TokenIdent> {
        if let Some(x) = self.follows.get(token) {
            return x.clone();
        }

        insert_extended(&mut self.follows, token, HashSet::new());

        let mut to_follow = vec![];

        for (start, rule) in self.grammar.rules() {
            for r in rule {
                // if let Some(pos) =  {
                let pos = r.values().iter().position(|x| x == token);
                
                if pos.is_none() {
                    continue;
                }

                let pos = pos.unwrap();

                if pos + 1 < r.values().len() {
                    insert_extended(
                        &mut self.follows,
                        token,
                        vec![r.values()[pos + 1].clone()].into_iter().collect(),
                    );
                } else {
                    to_follow.push(start.clone());
                }
            }
        }

        for f in to_follow {
            let v = self.follow(&f);
            insert_extended(&mut self.follows, token, v);
        }

        self.follows.get(token).unwrap().clone()
    }

    fn closure(&mut self, symbol: &Token) -> Vec<StateElement> {
        // check if already computed once
        if let Some(x) = self.closures.get(symbol) {
            return x.clone();
        }


        // get rules from symbol
        let rules: Option<&HashSet<_>> = self.grammar.rules().get(symbol);

        // if symbol is a terminal, rules will be none -> just return empty
        if rules.is_none() {
            return Vec::new();
        }

        // create state elements from rules
        let rules: Vec<StateElement> = rules
            .unwrap()
            .clone()
            .into_iter()
            .map(|x| StateElement::new(symbol.clone(), x))
            .collect();

        self.closures.insert(symbol.clone(), vec![]);

        let mut res = HashSet::new();

        for item in &rules { 
            if item.get().is_none() || res.contains(item) {
                continue;
            }

            res.insert(item.clone());

            let token = item.get().unwrap().try_into();

            if token.is_err() || self.is_terminal(token.as_ref().unwrap()) {
                continue;
            }

            res.extend(self.closure(token.as_ref().unwrap()));
        }
        res.extend(rules);
        let res = res.into_iter().collect::<Vec<_>>();
        self.closures.insert(symbol.clone(), res.clone());
        res
    }

    fn advance(&mut self, state: &State) {
        // for each item
        // get current token
        // create new state transition from token to
        // advanced item
        // + closure, if advanced item has a token it exists
        // keep track of new transitions

        // insert transitions
        // advance the transition targets

        if self.actions.contains_key(state) {
            return;
        }

        self.states.insert(state.clone(), self.states.len());

        let mut reductions = HashMap::new();
        let mut transitions = HashMap::new();

        for item in state {
            let token = item.get();
            let action = Action::Reduce(item.clone());

            if token.is_none() {
                let follow = self.follow(&item.start_symbol());

                for t in follow {
                    if reductions.contains_key(&t) {
                        let prev = reductions.get(&t).unwrap();

                        if *prev == action {
                            continue;
                        }

                        panic!("double action error");
                        // Self::double_action_error(state, &t, prev, Action::Reduce(item.clone()));
                    }

                    reductions.insert(t, Action::Reduce(item.clone()));
                }

                continue;
            }

            let token = token.unwrap();
            let next_item = item.advance();
            let next_token = next_item.get();

            let mut next_state = 
                if let Some(t) = next_token {
                    let token = t.try_into();
                    if token.is_err() {
                        vec![]
                    } else {
                        self.closure(&token.unwrap())
                    }
                } else {
                    vec![]
                };

            next_state.push(next_item);

            if reductions.contains_key(&token) {
                panic!("shift/goto and reduce for same inputs");
                // Self::double_action_error(state, &token, reductions.get(&token).unwrap(), Action::Transition(next_state))
            }

            insert_extended(&mut transitions, &token, next_state);
        }

        let mut actions = reductions;

        let added_states = transitions.clone().into_values().collect::<Vec<State>>();

        let transitions = transitions
            .into_iter()
            .map(|(token, state)| {
                (token, Action::Transition(state))
            })
            .collect::<HashMap<_, _>>();

        actions.extend(transitions);

        self.actions.insert(state.clone(), actions);

        for state in added_states {
            self.advance(&state);
        }
    }

    fn is_terminal(&self, token: &Token) -> bool {
        self.grammar.rules().get(token).is_none()
    }

    pub fn build(mut self) -> Table {
        let start_symbol = self.start_symbol.clone();
        let mut f = self.follow(&start_symbol);
        f.insert(Token::eof().into());

        self.follows.insert(
            self.start_symbol.clone(),
            f
            // vec![Token::eof().into()].into_iter().collect(),
        );
        
        let state_0 = self.closure(&start_symbol);

        self.advance(&state_0);
        Table::new(self.actions, state_0, self.grammar, self.states)
    }

    fn double_action_error(state: &State, token: &Token, prev: &Action, new: Action) -> ! {
        eprintln!("double action:");
        eprintln!("    previous: {:?}", prev);
        eprintln!("    new: {:?}", new);
        eprintln!("    state: {:?}", state);
        eprintln!("    token: {:?}", token);
        panic!()
    }
}

fn insert_extended<Item, K: Hash + Eq + Clone, V: IntoIterator<Item = Item> + Extend<Item>>(
    map: &mut HashMap<K, V>,
    k: &K,
    v: V,
) {
    if let Some(x) = map.get_mut(&k) {
        x.extend(v.into_iter());
    } else {
        map.insert(k.clone(), v);
    }
}
