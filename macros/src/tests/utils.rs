use common::{Id, Terminal, Variant};

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
        let name_eq = &self.name().x == other.name;

        let values: Vec<_> = self.values().iter().map(|id| {
            match id {
                Id::NonTerminal(non_terminal) => &non_terminal.x,
                Id::Terminal(t) => match t {
                    Terminal::EOF => panic!("Cannot compare EOF"),
                    Terminal::Value(x) => x.as_str()
                }
            }
        }).collect();

        name_eq && values == other.values
    }
}