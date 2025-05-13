mod state;
use std::{collections::HashMap, marker::PhantomData};

mod table_print;
pub use table_print::*;
use crate::{Grammar, GrammarTrait, Id, IdTrait, VariantId};
mod builder;
pub use builder::TableBuilder;
pub use state::*;

#[derive(Debug)]
pub struct Table<R: IdTrait, T: IdTrait, V: VariantId, G: GrammarTrait<R, T, V>> {
    _phantom: PhantomData<V>,
    start_state: State<R, T>,
    actions: HashMap<State<R, T>, HashMap<Id<R, T>, Action<R, T>>>,
    grammar: G,
}

impl<R: IdTrait, T: IdTrait, V: VariantId, G: GrammarTrait<R, T, V>> Table<R, T, V, G> {
    pub fn new(start_state: State<R, T>, actions: HashMap<State<R, T>, HashMap<Id<R, T>, Action<R, T>>>, grammar: G) -> Self {
        Self {
            _phantom: PhantomData,
            start_state,
            actions,
            grammar,
        }
    }
    
    pub fn start_state(&self) -> &State<R, T> {
        &self.start_state
    }

    // pub fn 
}