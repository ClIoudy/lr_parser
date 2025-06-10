use common::{Id, Terminal, Variant};

pub type TestRet = Result<(), Box<dyn std::error::Error>>;

pub struct VariantCompare {
    name: &'static str,
    values: Vec<&'static str>,
}

impl VariantCompare {
    pub fn new(name: &'static str, values: &[&'static str]) -> Self {
        Self {
            name,
            values: values.to_vec()
        }
    }
}

impl PartialEq<VariantCompare> for Variant {
    fn eq(&self, other: &VariantCompare) -> bool {
        let name_eq = self.name() == other.name;

        let values: Vec<_> = self.values().iter().map(|id| {
            match id {
                Id::N(non_terminal) => &non_terminal.x,
                Id::T(t) => match t {
                    Terminal::EOF => panic!("Cannot compare EOF"),
                    Terminal::Labeld(x) => x.as_str()
                }
            }
        }).collect();

        name_eq && values == other.values
    }
}

#[macro_export]
macro_rules! set {
    {} => {
        std::collections::HashSet::new();
    };
    
    {$($x:expr),+ $(,)?} => {
        
        std::collections::HashSet::<_>::from_iter(vec![$($x),+].into_iter())
    }
}