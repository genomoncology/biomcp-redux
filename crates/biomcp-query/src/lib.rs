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

/// Source-owned diagnostic laboratory tests and relations.
pub mod diagnostic;
/// Explicit CPIC gene and native context retrieval.
pub mod pgx;

/// Explicit native device report cards and sections.
pub mod adverse_event_maude;

pub mod biomarker;
pub mod organization;
/// Native human pathway discovery and exact cards.
pub mod pathway;
/// Literal phenotype discovery and exact independent source cards.
pub mod phenotype;
mod source_snapshot;
/// Stored study catalog discovery.
pub mod study;

/// Exact reviewed-human protein discovery and cards.
pub mod protein;

/// Checked native GWAS association search.
pub mod gwas;

/// Provider-local author discovery and exact cards.
pub mod author;
/// Literal Cellosaurus discovery and exact cell identity.
pub mod cell_line;
