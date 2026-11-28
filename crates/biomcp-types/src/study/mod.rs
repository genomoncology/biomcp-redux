//! Standalone study contracts; command registration belongs to the adapter slice.
mod query;
mod response;
mod structural_variants;
pub use query::{StudyGet, StudySearch, StudySection};
pub use response::*;
pub use structural_variants::*;
