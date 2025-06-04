use common::NonTerminal;

use crate::{table_builder::table::{builder::TableBuilder, state::State}, tests::TestRet};

use super::get_grammar;

#[test]
pub fn expand_test() -> TestRet {
    let grammar = get_grammar()?;
    let mut builder = TableBuilder::new(&grammar);

    let start_state = State::new(builder.closure(&NonTerminal::start_symbol()));
    builder.expand(&start_state);

    panic!("{:#?}", builder.shifts());


    Ok(())
}