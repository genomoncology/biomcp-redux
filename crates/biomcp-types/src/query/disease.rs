//! Shared disease request types.
mod get;
mod get_validation;
mod id;
mod search;
mod section;
mod source;
pub use get::DiseaseGet;
pub use id::DiseaseId;
pub use search::DiseaseSearch;
#[cfg(feature = "schema")]
pub use search::DiseaseSearchCommand;
pub use section::DiseaseSection;
pub use source::{DiseaseOntology, DiseaseSource};
fn default_limit() -> u32 {
    5
}
fn get_source() -> DiseaseSource {
    DiseaseSource::MyDisease
}
