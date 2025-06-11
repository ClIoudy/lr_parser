use std::error::Error;

use common::{NonTerminal};
use quote::quote;

use crate::{grammar::Grammar, tests::utils::VariantCompare};

#[test]
pub fn test() -> Result<(), Box<dyn Error>> {
    let input = quote! {
        S: A -> "a", B;
        S: B -> "b", A;
        A: A -> "a";
        B: B -> "b";
        B: C -> "c";
    };

    let grammar = syn::parse2::<Grammar>(input)?;

    assert!(grammar.all_rules().len() == 3);

    {
        let s = grammar.rule(&NonTerminal::start_symbol());
        assert!(s.len() == 2);
        
        let s_a = &s[0];
        assert!(*s_a.symbol() == NonTerminal::start_symbol());
        let cmp = VariantCompare::new("A", &["a", "B"]);
        assert!(*s_a == cmp);

        let s_b = &s[1];
        assert!(*s_b.symbol() == NonTerminal::start_symbol());
        let cmp = VariantCompare::new("B", &["b", "A"]);
        assert!(*s_b == cmp);
    }

    {
        let a = grammar.rule(&"A".into());
        assert!(a.len() == 1);
        let a = &a[0];
        let cmp = VariantCompare::new("A", &["a"]);
        assert!(*a == cmp);
    }
    
    {
        let b = grammar.rule(&"B".into());
        assert!(b.len() == 2);

        let b_b = &b[0];
        let cmp = VariantCompare::new("B", &["b"]);
        assert!(*b_b == cmp);  

        let b_c = &b[1];
        let cmp = VariantCompare::new("C", &["c"]);
        assert!(*b_c == cmp);  
    }

    Ok(())
}