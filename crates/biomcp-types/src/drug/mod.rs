//! Source-selected drug contracts; adapters and command dispatch live elsewhere.
mod query;
mod response;
pub use query::{DrugGet, DrugSearch, DrugSection, DrugSource};
pub use response::{
    DrugApprovals, DrugCard, DrugCivic, DrugHeader, DrugIndications, DrugInteractions, DrugLabel,
    DrugLabelText, DrugPage, DrugRecords, DrugRegulatory, DrugTargets,
};
