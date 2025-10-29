use std::{assert_matches::assert_matches, collections::{HashMap, HashSet}, fmt::Debug};

use common::{Action, Id, NonTerminal, StateId, Terminal};

use crate::{table::builder::{builder::TableBuilder, state::State, tests::get_grammar}, tests::TestRet};

#[test]
pub fn test() -> TestRet {

    let grammar = get_grammar()?;
    let mut builder = TableBuilder::new(&grammar);

    let start_state = State::new(builder.closure(&NonTerminal::start_symbol()));
    builder.expand(&start_state);

    let table = builder.build();
    
    let expected = table.expected;
    let actions = table.actions;

    assert_eq!(actions.len(), expected.len());
    assert_eq!(actions.len(), 6);

    let a = Id::T("a".into());
    let c = Id::T("c".into());
    let d = Id::T("d".into());

    assert_shift(&actions, 0, &a, 1);
    
    assert_shift(&actions, 1, &c, 3);
    assert_shift(&actions, 1, &d, 5);
    
    assert_shift(&actions, 3, &c, 3);
    
    // assert_actions(&actions, 3, &c, Action::Reduce());
    assert_reduce(&actions, 4, &Id::T(Terminal::EOF), "B", "C");

    assert_expected(&expected, 0, vec!["a"]);
    assert_expected(&expected, 3, vec!["c", "d"]);

    assert_expected(&expected, 3, vec!["c", "d"]);

    Ok(())
}

fn assert_shift(actions: &HashMap<StateId, HashMap<Id, Action>>, state_id: usize, token: &Id, shift_to: usize) {
    // assert!(actions.get(&state_id).cloned().unwrap().into_keys().collect::<Vec<_>>() == with);
    assert_eq!(
        *actions.get(&state_id).unwrap().get(token).unwrap(),
        Action::Shift(shift_to)
    )
}

fn assert_reduce(actions: &HashMap<StateId, HashMap<Id, Action>>, state_id: usize, token: &Id, expected_symbol: &str, expected_name: &str) {

    let reduction = actions.get(&state_id).unwrap().get(token).unwrap();

    assert_matches!(reduction, Action::Reduce(s) if s.symbol() == expected_symbol && s.name() == expected_name);
}

fn assert_expected<I: Into<Terminal> + Debug>(expected: &HashMap<usize, HashSet<Terminal>>, state_id: usize, with: Vec<I>) {
    let with: HashSet<Terminal> = with.into_iter().map(|x| x.into()).collect();

    assert_eq!(*expected.get(&state_id).unwrap(), with)
}