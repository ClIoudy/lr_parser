use std::{collections::{HashMap, HashSet}, hash::Hash};

use common::{Action, Id, NonTerminal, Terminal, Variant, VariantId};


use super::item::StateItem;

use crate::{grammar::Grammar, table_builder::table::{state::State, Table}};

pub struct TableBuilder<'a> {
    grammar: &'a Grammar,
    closures: HashMap<NonTerminal, HashSet<StateItem>>,
    follows: HashMap<NonTerminal, HashSet<Id>>,
    // actions: HashMap<State, HashMap<Id, Action>>,
    shifts: HashMap<State, HashMap<Id, State>>,
    reductions: HashMap<State, HashMap<Id, VariantId>>
}

impl<'a> TableBuilder<'a> {
    pub fn new(mut grammar: &'a Grammar) -> Self {

        // let start_rules = grammar.rule_mut(&NonTerminal::start_symbol());

        // for s in start_rules {
        //     s.values_mut().push(Id::Terminal(Terminal::EOF));
        // }

        //-----

        // let new_start_symbol = "#E".to_string();
        // let name = "_".to_string();

        // let values = vec![Id::NonTerminal(NonTerminal::start_symbol()), Id::Terminal(Terminal::EOF)];

        // let id = VariantId::new(new_start_symbol.clone(), name, 2);

        // let v = Variant::new(values, id);

        // grammar.add_rule(new_start_symbol.into(), vec![v]);

        let mut f = HashSet::new();
        f.insert(Id::T(Terminal::EOF));
        let mut follows = HashMap::new();
        follows.insert(NonTerminal::start_symbol(), f);

        Self {
            grammar,
            closures: HashMap::new(),
            follows,
            shifts: HashMap::new(),
            reductions: HashMap::new(),
        }
    }

    pub fn closure(&mut self, id: &NonTerminal) -> HashSet<StateItem> {
        if let Some(res) = self.closures.get(id) {
            return res.clone();
        }

        let variants = self.grammar.rule(id);
        
        self.closures.insert(id.clone(), HashSet::new());
        
        let mut res = HashSet::new();
        
        let mut others = HashSet::new();

        // for r in rules add itself and its closure to the result
        for r in variants {
            if r.values().first().is_none() {
                continue;
            }

            if let Id::N(x) = r.values().first().unwrap() {
                others.insert(x.clone());
            }

            let item = StateItem::new(r.clone());

            res.insert(item);
        }
        
        for o in others {
            res.extend(self.closure(&o));
        }

        self.closures.insert(id.clone(), res.clone());
        
        res
    }

    pub fn follow(&mut self, id: &NonTerminal) -> HashSet<Id> {        
        if let Some(res) = self.follows.get(id) {
            return res.clone();
        }

        self.follows.insert(id.clone(), HashSet::new());
        let mut res = HashSet::new();

        let mut to_follow = HashSet::new();

        for (rule_id, variants) in self.grammar.all_rules() {
            for v in variants {
                let values = v.values();

                let occurences = find_all(v.values().iter(), |x| {
                    if let Id::N(y) = x {
                        y == id
                    } else {
                        false
                    }
                });

                for i in occurences {
                    if i + 1 < values.len() {
                        res.insert(values[i+1].clone());
                    } else {
                        to_follow.insert(rule_id.clone());
                    }
                }
            }
        }

        for f in to_follow {
            res.extend(self.follow(&f));
        }

        self.follows.insert(id.clone(), res.clone());

        res
    }

    fn expand(&mut self, state: &State) {
        if self.shifts.contains_key(state) {
            return;
        }

        self.shifts.insert(state.clone(), HashMap::new());
        let mut reductions = HashMap::new();
        let mut new_states = HashMap::new();

        for item in state.items() {
            if let Some(k) = item.get() {
                // SHIFT
                let new_state = self.make_state(k, item);
                extend_set_map(&mut new_states, k, new_state);
            } else {
                // REDUCTION
                for id in self.follow(item.symbol()) {
                    let v = item.variant().id().clone();

                    if reductions.insert(id.clone(), v).is_some() {
                        panic!("Doulbe reduction error");
                    }
                }
            }
        }

        let mut transitions = new_states.into_iter().map(|(id, state_items)| (id, State::new(state_items))).collect();

        for (id, new_state) in &transitions {
            self.expand(new_state);
        }

        self.reductions.insert(state.clone(), reductions);
        self.shifts.insert(state.clone(), transitions);
    }

    fn make_state(&mut self, k: &Id, item: &StateItem) -> HashSet<StateItem> {
        // state: item + closure of new item
        let advanced = item.advance();
        
        if advanced.is_none() {
            return HashSet::new();
        }
        
        let mut new_state = HashSet::new();
        new_state.insert(item.advance().unwrap());
    
        if let Id::N(r) = k {
            new_state.extend(self.closure(&r));
        }
        
        new_state
    }

    pub fn build(mut self) -> Table {
        todo!("number states");
        todo!("construct actions");
        todo!("construct expected");
        
        let start_state = State::new(self.closure(&NonTerminal::start_symbol()));
        self.expand(&start_state);

        // number states

        // create actions
        // create expected


        // Table::new(expected, actions);
        todo!()
        // Table::new()
    }
}

fn find_all<T>(iter: impl Iterator<Item=T>, predicate: impl Fn(T) -> bool) -> Vec<usize> {
    let mut res = vec![];

    for (i, x) in iter.enumerate() {
        if predicate(x) {
            res.push(i);
        }
    }

    res
}

fn extend_set_map<K: Hash + Eq + Clone, V: Hash + Eq>(map: &mut HashMap<K, HashSet<V>>, key: &K, value: impl IntoIterator<Item=V>) {
    if let Some(x) = map.get_mut(key) {
        x.extend(value);
    } else {
        let mut x = HashSet::new();
        // x.insert(value);
        x.extend(value);
        map.insert(key.clone(), x);
    }
}

// id is user-specified and lexer uses ids too instead of labels