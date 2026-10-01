//! sts2core — deterministic simulation kernel.
//!
//! This crate deliberately contains only the game-state transition layer.
//! Real-game observation, replay validation, search/planning, evaluation,
//! MCP integration, and data extraction belong in external crates/modules.

pub mod asc;
pub mod content;
pub mod damage;
pub mod ops;
pub mod state;
pub mod step;

pub use state::{CardInst, Entity, Pending, Rng, St, State, F_CORRUPT, F_UPGRADED};
pub use step::{
    begin_combat, end_turn_with_incoming, end_turn_with_live_incoming, legal_actions, step, Action,
    Incoming,
};
