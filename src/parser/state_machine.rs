use common::{StateId, TableTrait};

pub struct StateMachine {
    state_history: Vec<StateId>,
}

impl StateMachine {
    pub fn new<T: TableTrait>() -> Self {
        Self {
            state_history: vec![T::start_state()],
        }
    }

    pub fn state(&self) -> &StateId {
        self.state_history.last().unwrap()
    }

    pub fn advance(&mut self, state: StateId) {
        self.state_history.push(state);
    }

    pub fn revert(&mut self, by: usize) {
        let n = self.state_history.len() - by;
        self.state_history.truncate(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{Action, Id, NonTerminal, Terminal};
    use std::collections::{HashMap, HashSet};
    use std::any::Any;

    // Mock TableTrait implementation for testing
    struct TestTable;

    impl TableTrait for TestTable {
        type StartSymbol = String;

        fn start_state() -> StateId {
            0
        }

        fn action(_state: &StateId, _token: &Id) -> Option<Action> {
            None
        }

        fn build_rule(_variant: common::VariantId, _children: Vec<Box<dyn Any>>) -> Option<Box<dyn Any>> {
            None
        }

        fn expected(_state: &StateId) -> Option<HashSet<Terminal>> {
            None
        }

        fn alphabet() -> HashSet<&'static str> {
            HashSet::new()
        }
    }

    #[test]
    fn test_new_initializes_with_start_state() {
        let sm = StateMachine::new::<TestTable>();
        assert_eq!(*sm.state(), 0);
    }

    #[test]
    fn test_advance_adds_state() {
        let mut sm = StateMachine::new::<TestTable>();
        sm.advance(1);
        assert_eq!(*sm.state(), 1);
        
        sm.advance(2);
        assert_eq!(*sm.state(), 2);
    }

    #[test]
    fn test_revert_removes_states() {
        let mut sm = StateMachine::new::<TestTable>();
        sm.advance(1);
        sm.advance(2);
        sm.advance(3);
        
        assert_eq!(*sm.state(), 3);
        
        sm.revert(2);
        assert_eq!(*sm.state(), 1);
    }

    #[test]
    fn test_revert_to_start_state() {
        let mut sm = StateMachine::new::<TestTable>();
        sm.advance(1);
        sm.advance(2);
        
        sm.revert(2);
        assert_eq!(*sm.state(), 0);
    }

    #[test]
    fn test_multiple_advances_and_reverts() {
        let mut sm = StateMachine::new::<TestTable>();
        
        // Build up state stack
        sm.advance(1);
        sm.advance(2);
        sm.advance(3);
        assert_eq!(*sm.state(), 3);
        
        // Revert partially
        sm.revert(1);
        assert_eq!(*sm.state(), 2);
        
        // Advance again
        sm.advance(4);
        assert_eq!(*sm.state(), 4);
        
        // Revert multiple
        sm.revert(2);
        assert_eq!(*sm.state(), 1);
    }
}