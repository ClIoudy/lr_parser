use std::collections::{HashMap, HashSet};

use common::{Action, NonTerminal, Terminal, Variant, VariantId};

use crate::{table_builder::table::{builder::TableBuilder, item::StateItem, state::State}, tests::TestRet};

use super::get_grammar;

#[test]
pub fn expand_test() -> TestRet {
    let grammar = get_grammar()?;
    let mut builder = TableBuilder::new(&grammar);

    let start_state = State::new(builder.closure(&NonTerminal::start_symbol()));
    builder.expand(&start_state);

    for start_rule in grammar.rule(&NonTerminal::start_symbol()) {
        let mut end_state_item = StateItem::new(start_rule.clone());

        while !end_state_item.is_finished() {
            end_state_item = end_state_item.advance();
        }

        let mut end_state = HashSet::new();
        end_state.insert(end_state_item);

        let end_state = State::new(end_state);

        let r = builder.actions().get(&end_state).unwrap().get(&common::Id::T(Terminal::EOF)).unwrap();
        assert!(matches!(r, Action::Reduce(x) if x == start_rule.id()));

    }

    Ok(())
}