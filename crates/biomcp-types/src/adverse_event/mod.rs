//! Standalone adverse event contracts; command registration belongs to its runtime slice.
mod query;
mod response;
mod selectors;
pub use query::{AdverseEventGet, AdverseEventSearch};
pub use response::{
    AdverseEventDevice, AdverseEventDevices, AdverseEventDrug, AdverseEventDrugs,
    AdverseEventFaersCard, AdverseEventHeader, AdverseEventIdentifierRecords,
    AdverseEventIdentifiers, AdverseEventIdentityCounts, AdverseEventMaudeCard,
    AdverseEventNarrative, AdverseEventNarrativeItem, AdverseEventPage, AdverseEventReaction,
    AdverseEventReactions, AdverseEventRecords,
};
pub use selectors::{
    AdverseEventIdentifierSection, AdverseEventSection, AdverseEventSerious, AdverseEventSource,
    ReportUnit,
};
