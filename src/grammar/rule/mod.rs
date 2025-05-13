use std::hash::Hash;

// use std::{any::Any, fmt::Debug, hash::Hash};
use dyn_clone::DynClone;

// use crate::{Id, IdTrait};

use crate::{Id, IdTrait};

pub type Rule<R, T, V> = Box<dyn RuleVariant<R, T, V>>;

pub trait VariantId: Eq + Hash + PartialEq {

}

pub trait RuleVariant<R: IdTrait, T: IdTrait, V: VariantId>: DynClone {
    fn start_symbol(&self) -> R;
    fn id(&self) -> V;
    fn values(&self) -> Vec<Id<R, T>>;
}


// pub trait RuleTrait<R: IdTrait, T: IdTrait>: Any + DynClone + Debug {
//     fn values(&self) -> Vec<Vec<Id<R, T>>>;
//     fn id(&self) -> R;
//     fn start_symbol() -> R;
// }

dyn_clone::clone_trait_object!(<R, T, V> RuleVariant<R, T, V> where R: IdTrait, T: IdTrait, V: VariantId);

impl<R: IdTrait, T: IdTrait, V: VariantId> PartialEq for dyn RuleVariant<R, T, V> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<R: IdTrait, T: IdTrait, V: VariantId> Eq for dyn RuleVariant<R, T, V> {

}

impl<R: IdTrait, T: IdTrait, V: VariantId> Hash for dyn RuleVariant<R, T, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}