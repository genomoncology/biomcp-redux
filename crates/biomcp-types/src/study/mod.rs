//! Standalone study contracts; command registration belongs to the adapter slice.
mod query;
mod response;
pub use query::{StudyGet, StudySearch, StudySection};
pub use response::*;
