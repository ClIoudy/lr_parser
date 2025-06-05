use std::collections::HashSet;

use common::{NonTerminal, Variant};



use super::{get_grammar, StateItem, TableBuilder};

#[test] 
fn closure_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut grammar = get_grammar()?;
    
    let mut table_builder = TableBuilder::new(&grammar);

    {
        let symbol = NonTerminal::start_symbol();

        let closure = table_builder.closure(&symbol);

        let variants = grammar.rule(&symbol);
        let variants = variants.into_iter().map(|x| StateItem::new(x.clone())); 
        
        let expected_closure: HashSet<StateItem> = HashSet::from_iter(variants);

        assert!(closure == expected_closure, "{:?} \n!= \n{:?}", closure, expected_closure);
    }

    {
        let symbol = "F".into();

        let closure = table_builder.closure(&symbol);
        
        let variants_1 = grammar.rule(&symbol);
        let variants_2 = grammar.rule(&"E".into());
        let variants_3 = grammar.rule(&"X".into());
        
        let mut set: HashSet<&Variant> = HashSet::from_iter(variants_1.into_iter());
        set.extend(variants_2.into_iter());
        set.extend(variants_3.into_iter());

        let set: HashSet<_> = set.into_iter().map(|x| StateItem::new(x.clone())).collect();

        assert!(set == closure);
    }


    Ok(())
}