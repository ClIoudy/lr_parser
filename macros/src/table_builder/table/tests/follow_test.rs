use std::error::Error;

use crate::table_builder::table::{builder::TableBuilder, tests::get_grammar};

#[test]
fn follow_test() -> Result<(), Box<dyn Error>> {
    let grammar = get_grammar()?;
    let mut builder = TableBuilder::new(&grammar);

    {
        let id = common::Id::NonTerminal("#S".into());
        let follow = builder.follow(&id);

        panic!("{:?}", follow);
    }

    Ok(())
}