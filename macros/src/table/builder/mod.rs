use std::collections::HashMap;

use common::{Action, Id, Terminal};
use item::StateItem;

mod builder;
use builder::TableBuilder;
mod item;
mod state;
use state::State;
use super::Table;

#[cfg(test)]
mod tests;

