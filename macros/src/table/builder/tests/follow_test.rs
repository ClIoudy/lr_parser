use std::str::FromStr;

use common::{Id, NonTerminal, Terminal};
use proc_macro2::TokenStream;

use super::{get_grammar, TableBuilder};
use crate::{set, tests::TestRet};

#[test]
fn follow_test_1() -> TestRet {
    let grammar = get_grammar()?;
    let mut builder = TableBuilder::new(&grammar);

    {
        let id = NonTerminal::start_symbol();
        let follow = builder.follow(&id);

        assert_eq!(follow, set! { Id::T(Terminal::EOF) });
    }

    {
        let id = "B".into();
        let follow = builder.follow(&id);

        assert_eq!(follow, set! { Id::T(Terminal::EOF) });
    }

    {
        let id = "E".into();
        let follow = builder.follow(&id);

        assert_eq!(follow, set! { Id::T(Terminal::Labeld("f".to_string())) });
    }

    Ok(())
}

#[test]
fn follow_test_2() -> TestRet {
    // let grammar = get_grammar();
    let input = "S: A -> S, \"a\"; S: B -> \"b\"";
    let input = TokenStream::from_str(input)?;
    let grammar = syn::parse2(input)?;

    let mut builder = TableBuilder::new(&grammar);

    {
        let id = "S".into();
        let follow = builder.follow(&id);
        
        assert_eq!(follow, set!{ Id::T(Terminal::EOF), Id::T("a".into()) })
    }

    Ok(())
}