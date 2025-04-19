use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::Token;

use super::{state::StateElement, Action, State};

use super::Table;
use crate::Grammar;

pub struct TableBuilder<'a> {
    grammar: Grammar,
    start_symbol: &'a Token,
    closures: HashMap<Token, Vec<StateElement>>,
    actions: HashMap<State, HashMap<Token, Action>>,
    follows: HashMap<Token, HashSet<Token>>,
}

impl<'a> TableBuilder<'a> {
    pub fn new(grammar: &'a Grammar, start_symbol: &'a Token) -> Self {
        let mut grammar = grammar.clone();

        if !grammar.rules().contains_key(start_symbol) {
            panic!("Grammar has to contain start symbol");
        }

        let mut iter = grammar.rules_mut().remove(&start_symbol).unwrap().into_iter();

        let mut set = HashSet::with_capacity(iter.len());

        for mut v in &mut iter {
            v.values_mut().push(Token::eof());
            set.insert(v);
        }

        grammar.rules_mut().insert(start_symbol.clone(), set);

        Self {
            grammar,
            start_symbol,
            closures: HashMap::new(),
            actions: HashMap::new(),
            follows: HashMap::new(),
        }
    }

    fn follow(&mut self, token: &Token) -> HashSet<Token> {
        if let Some(x) = self.follows.get(token) {
            return x.clone();
        }

        insert_extended(&mut self.follows, token, HashSet::new());

        let mut to_follow = vec![];

        for (start, rule) in self.grammar.rules() {
            for r in rule {
                if let Some(pos) = r.values().iter().position(|x| x == token) {
                    if pos + 1 >= r.values().len() {
                        to_follow.push(start.clone());
                    } else {
                        insert_extended(
                            &mut self.follows,
                            token,
                            vec![r.values()[pos + 1].clone()].into_iter().collect(),
                        );
                    }
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

        let mut res = HashSet::new();

        // get rules from symbol
        let rules: Option<&HashSet<_>> = self.grammar.rules().get(symbol);

        // if symbol is a terminal, rules will be none -> just return empty
        if rules.is_none() {
            return Vec::new();
        }

        // println!("RULES: {symbol:?} -> {:?}", rules);

        // create state elements from rules
        let rules: Vec<StateElement> = rules
            .unwrap()
            .clone()
            .into_iter()
            .map(|x| StateElement::new(symbol.clone(), x))
            .collect();

        // println!("ITEMS: {:?}", rules);

        self.closures.insert(symbol.clone(), vec![]);

        for item in &rules { 
            // println!("#{:?}", item);
            if item.get().is_none() || res.contains(item) {
                // println!("  skipping: {:?}", item);
                // println!("  get: {} | {}", item.get().is_none(), res.contains(item));
                // println!("  :  RES: {:?}; \n     ITEM: {:?}", res, item);
                continue;
            }

            // println!("          adding {:?}", item);
            res.insert(item.clone());

            res.extend(self.closure(&item.get().unwrap()));
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

                        Self::double_action_error(state, &t, prev, Action::Reduce(item.clone()));
                    }

                    reductions.insert(t, Action::Reduce(item.clone()));
                }

                continue;
            }

            let token = token.unwrap();
            let next_item = item.advance();
            let next_token = next_item.get();

            let mut next_state = if let Some(t) = next_token {
                self.closure(&t)
            } else {
                vec![]
            };

            next_state.push(next_item);

            if reductions.contains_key(&token) {
                // panic!("shift/goto and reduce for same inputs");
                Self::double_action_error(state, &token, reductions.get(&token).unwrap(), Action::Transition(next_state))
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
            .collect::<HashMap<Token, Action>>();

        actions.extend(transitions);

        self.actions.insert(state.clone(), actions);

        for state in added_states {
            self.advance(&state);
        }
    }

    pub fn build(mut self) -> Table {
        self.follows.insert(
            self.start_symbol.clone(),
            vec![Token::eof()].into_iter().collect(),
        );

        let state_0 = self.closure(self.start_symbol);

        self.advance(&state_0);
        Table::new(self.actions, state_0)
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
