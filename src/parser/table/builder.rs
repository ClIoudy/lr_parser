use std::{collections::{HashMap, HashSet}, hash::Hash, marker::PhantomData};

use crate::{ids::*, GrammarTrait, Rule, RuleVariant, VariantId };
use super::{state::Action, State, StateItem, Table};

pub struct TableBuilder<R: IdTrait, T: IdTrait, V: VariantId, G: GrammarTrait<R, T, V>> {
    _phantom: PhantomData<V>,
    grammar: G,
    closures: HashMap<R, HashSet<StateItem<R, T>>>,
    follows: HashMap<Id<R, T>, HashSet<Id<R, T>>>,
    actions: HashMap<State<R, T>, HashMap<Id<R, T>, Action<R, T>>>,
    
}

impl<R: IdTrait, T: IdTrait, V: VariantId, G: GrammarTrait<R, T, V>> TableBuilder<R, T, V, G> {
    pub fn new(grammar: G) -> Self {
        Self {
            _phantom: PhantomData,
            grammar,
            closures: HashMap::new(),
            follows: HashMap::new(),
            actions: HashMap::new(),
        }
    }

    pub fn closure(&mut self, id: &R) -> HashSet<StateItem<R, T>> {
        if let Some(res) = self.closures.get(id) {
            return res.clone();
        }

        let rules = self.grammar.rule(id);
        self.closures.insert(id.clone(), HashSet::new());
        
        let mut res = HashSet::new();
        
        // for r in rules add itself and its closure to the result
        for r in rules {
            if r.values().first().is_none() {
                continue;
            }

            if let Id::Rule(x) = r.values().first().unwrap() {
                res.extend(self.closure(x));
            }
            
            let item = StateItem::new(r);

            res.insert(item);
        }
        
        self.closures.insert(id.clone(), res.clone());
        
        res
    }

    fn follow(&mut self, id: &Id<R, T>) -> HashSet<Id<R, T>> {
        self.follows.insert(id.clone(), HashSet::new());
        let mut res = HashSet::new();

        for rule in self.grammar.all_rules() {
            let values = rule.values();

            let occurences = find_all(rule.values().iter(), |x| x == id);

            for i in occurences {
                if i + 1 < values.len() {
                    res.insert(values[i+1].clone());
                } else {
                    res.extend(self.follow(
                        &Id::Rule(rule.start_symbol())
                    ));
                }
            }
        }

        res
    }

    fn expand(&mut self, state: &State<R, T>) {
        let mut actions = HashMap::new();
        let mut transitions = HashMap::new();

        if self.actions.contains_key(state) {
            return;
        }

        self.actions.insert(state.clone(), HashMap::new());

        for item in state.items() {
            if let Some(k) = item.get() {
                let new_state = self.make_state(k, item);
                extend_set_map(&mut transitions, k, new_state);
                // Self::add_action(&mut actions, k.clone(), Action::SHIFT(new_state), state);
            } else {
                for (k, v) in self.reduction(item) {
                    // reductions.insert(k, v);
                    Self::add_action(&mut actions, k, Action::REDUCE(v), state);
                }
            }
        }

        for (k, new_state) in transitions {
            let new_state = State::new(new_state);
            self.expand(&new_state);
            Self::add_action(&mut actions, k, Action::SHIFT(new_state), state);
        } 

        self.actions.insert(state.clone(), actions);

    }

    fn add_action(map: &mut HashMap<Id<R, T>, Action<R, T>>, k: Id<R, T>, v: Action<R, T>, from: &State<R, T>) {
        if map.contains_key(&k) {
            panic!("double action error: key: {k:?}, \nprev value: {v:#?} \nother value: {:#?} \nfrom: {from:#?}", map.get(&k).unwrap())
        } else {
            map.insert(k, v);
        }
    }

    fn make_state(&mut self, k: &Id<R, T>, item: &StateItem<R, T>) -> HashSet<StateItem<R, T>> {
            // state: item + closure of new item
            let mut new_state = HashSet::new();
            println!("{:?}", item);
            println!("{:?}", item.advance());
            new_state.insert(item.advance().unwrap());
        
            if let Id::Rule(r) = k {
                new_state.extend(self.closure(&r));
            }
            
            new_state
    }

    fn reduction(&mut self, item: &StateItem<R, T>) -> HashSet<(Id<R, T>, StateItem<R, T>)> {
        
        let mut res = HashSet::new();

        for f in self.follow(&Id::Rule(item.start_symbol().clone())) {
            res.insert((f, item.clone()));
        }

        res

    }

    pub fn build(mut self) -> Table<R, T, V, G> {
        let start_symbol = self.grammar.start_symbol().clone();
        let start_state = State::new(self.closure(&start_symbol));
        self.expand(&start_state.clone());
        Table::new(start_state, self.actions, self.grammar)
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