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

pub mod variant;
