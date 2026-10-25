//! Resolve checked human gene inputs through authoritative source adapters.
pub mod article;
pub mod gene;
pub mod store;

mod context;
mod run;
mod status;
pub use context::Ctx;
pub use run::{Action, Bundle, Outcome, run};

mod fan_out;

pub mod health;

pub mod trial;
pub mod variant;

pub mod disease;
/// Single-source drug discovery and exact native cards.
pub mod drug;

/// Source-owned adverse event search.
pub mod adverse_event;
