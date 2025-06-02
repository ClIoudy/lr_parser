use crate::Id;

mod abc;
pub use abc::ABC;

#[derive(Debug, Clone, Hash)]
pub struct Variant {
    name: String,
    values: Vec<Id>
}

impl Variant {
    pub fn new(name: String, values: Vec<Id>) -> Self {
        Self {
            name,
            values
        }
    }

    pub fn values(&self) -> &Vec<Id> {
        &self.values
    }
}

pub trait RuleTrait {
    fn id(&self) -> Id;
}