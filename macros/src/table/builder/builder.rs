use std::{collections::{HashMap, HashSet}, hash::Hash};

use common::{grammar, Action, Id, NonTerminal, StateId, Terminal, Variant, VariantId};


use super::{StateItem, State, Table};

use crate::grammar::Grammar;

pub struct TableBuilder<'a> {
    grammar: &'a Grammar,
    closures: HashMap<NonTerminal, HashSet<StateItem>>,
    follows: HashMap<NonTerminal, HashSet<Id>>,
    states: HashMap<State, usize>,
    actions: HashMap<StateId, HashMap<Id, Action>>
}

impl<'a> TableBuilder<'a> {
    pub fn new(grammar: &'a Grammar) -> Self {
        let mut f = HashSet::new();
        f.insert(Id::T(Terminal::EOF));
        let mut follows = HashMap::new();
        follows.insert(NonTerminal::start_symbol(), f);

        Self {
            grammar,
            follows,
            closures: HashMap::new(),
            states: HashMap::new(),
            actions: HashMap::new(),
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

    pub fn expand(&mut self, state: &State) {
        if self.states.contains_key(state) {
            return;
        }

        self.add_state(state.clone());
        
        let mut reductions = HashMap::new();
        let mut transitions = HashMap::new();

        for item in state.items() {
            if let Some(k) = item.get() {
                // SHIFT
                let new_state = self.make_state(k, item);
                extend_set_map(&mut transitions, k, new_state);
            } else {
                // REDUCTION
                for id in self.follow(item.symbol()) {
                    let v = item.variant().id().clone();
                    if reductions.insert(id.clone(), Action::Reduce(v)).is_some() {
                        panic!("Doulbe reduction error");
                    }
                }
            }
        }

        // collect as vec of states
        // states only created here in order to allow expanding states while going trough items
        // (necessary when state s has multiple transitions with the same key to different states. 
        //  Then those two states need to be merged. Therefore states only created here after all merges completed)
        let mut transitions: Vec<(_, _)> = transitions.into_iter().map(|(id, state_items)| (id, State::new(state_items))).collect();

        // important for tests and makes analyzing tables much easier 
        // (order lost due to hashmap and now restored by sorts)
        transitions.sort();

        let mut actions = reductions;

        for (id, new_state) in transitions {
            self.expand(&new_state);
            let state_nmbr = self.number(&new_state);
            actions.insert(id, Action::Shift(state_nmbr));
        }

        self.actions.insert(self.number(state), actions);
    }

    fn add_state(&mut self, state: State) {
        self.states.insert(state, self.states.len());
    }

    fn number(&self, state: &State) -> usize {
        *self.states.get(state).unwrap()
    }

    fn make_state(&mut self, k: &Id, item: &StateItem) -> HashSet<StateItem> {
        // state: item + closure of new item
        let advanced = item.advance();

        let mut new_state = HashSet::new();
    
        if let Some(Id::N(r)) = advanced.get() {
            new_state.extend(self.closure(&r));
        }
        
        new_state.insert(advanced);

        new_state
    }

    pub fn build(mut self) -> Table {

        let expected = todo!("construct 'expected'");
        
        Table::new(expected, self.actions)
    }    

    #[cfg(test)]
    pub fn actions(&self) -> &HashMap<StateId, HashMap<Id, Action>> {
        &self.actions
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