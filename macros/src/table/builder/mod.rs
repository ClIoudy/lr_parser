use item::StateItem;

mod builder;
pub use builder::TableBuilder;
mod item;
mod state;
use state::State;
use super::TableMacroInfo;

#[cfg(test)]
mod tests;

