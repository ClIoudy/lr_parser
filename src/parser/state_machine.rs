use common::{State, TableTrait};

pub struct StateMachine {
    state_history: Vec<State>,
}

impl StateMachine {
    pub fn new<T: TableTrait>(table: &T) -> Self {
        Self {
            state_history: vec![*table.start_state()],
        }
    }

    pub fn state(&self) -> &State {
        self.state_history.last().unwrap()
    }

    pub fn advance(&mut self, state: State) {
        self.state_history.push(state);
    }

    pub fn revert(&mut self, by: usize) {
        let n = self.state_history.len() - by;
        self.state_history.truncate(n);
    }
}