//! Source-selected drug contracts; adapters and command dispatch live elsewhere.
mod query;
mod response;
pub use query::{DrugGet, DrugSearch, DrugSection, DrugSource};
pub use response::{DrugHeader, DrugCard, DrugPage, DrugLabel, DrugLabelText, DrugTargets, DrugIndications, DrugCivic, DrugApprovals, DrugRegulatory, DrugInteractions, DrugRecords};
