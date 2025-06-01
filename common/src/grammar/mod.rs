use crate::Id;

pub struct Variant {

}

pub trait RuleTrait {
    fn id(&self) -> Id;
}