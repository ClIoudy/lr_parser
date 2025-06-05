use std::error::Error;

use common::{Id, NonTerminal, Terminal};

use crate::{set, table_builder::table::{builder::TableBuilder, tests::get_grammar}, tests::utils};

#[test]
fn follow_test() -> Result<(), Box<dyn Error>> {
    let grammar = get_grammar()?;
    let mut builder = TableBuilder::new(&grammar);

    {
        let id = NonTerminal::start_symbol();
        let follow = builder.follow(&id);

        assert!(follow == set! { Id::T(Terminal::EOF) });
    }

    {
        let id = "B".into();
        let follow = builder.follow(&id);

        assert!(follow == set! { Id::T(Terminal::EOF) });
    }

    {
        let id = "E".into();
        let follow = builder.follow(&id);

        assert!(follow == set! { Id::T(Terminal::Value("f".to_string())) });
    }

    Ok(())
}