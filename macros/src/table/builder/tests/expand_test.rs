use std::collections::{HashMap, HashSet};

use common::{Action, NonTerminal, Terminal, Variant, VariantId};

use crate::tests::TestRet;

use super::{get_grammar, StateItem, TableBuilder, State};


#[test]
pub fn expand_test() -> TestRet {
    let grammar = get_grammar()?;
    let mut builder = TableBuilder::new(&grammar);

    let start_state = State::new(builder.closure(&NonTerminal::start_symbol()));
    builder.expand(&start_state);

    

    // for start_rule in grammar.rule(&NonTerminal::start_symbol()) {


    //     let r = builder.actions().get(&end_state).unwrap().get(&common::Id::T(Terminal::EOF)).unwrap();
    //     assert!(matches!(r, Action::Reduce(x) if x == start_rule.id()));

    // }

    Ok(())
}