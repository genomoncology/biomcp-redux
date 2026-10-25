//! Source-selected drug contracts; adapters and command dispatch live elsewhere.
mod civic;
pub use civic::{DrugCivicClaim, DrugCivicPage, DrugCivicProfile, DrugCivicStreams};
mod query;
mod response;
mod source;
pub use query::{DrugGet, DrugSearch, DrugSection};
pub use response::{
    DrugApprovals, DrugCard, DrugCivic, DrugHeader, DrugIndications, DrugInteractions, DrugLabel,
    DrugLabelText, DrugPage, DrugRecords, DrugRegulatory, DrugTargets, DrugXrefs,
};

pub use source::DrugSource;
