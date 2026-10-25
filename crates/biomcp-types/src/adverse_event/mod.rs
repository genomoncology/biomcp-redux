//! Source-specific adverse event requests and bounded report responses.
mod query;
mod response;
mod selectors;
pub use query::{AdverseEventGet, AdverseEventSearch};
pub use response::{
    AdverseEventDevice, AdverseEventDevices, AdverseEventDrug, AdverseEventDrugs,
    AdverseEventFaersCard, AdverseEventHeader, AdverseEventIdentifierRecords,
    AdverseEventIdentifiers, AdverseEventIdentityCounts, AdverseEventMaudeCard,
    AdverseEventNarrative, AdverseEventNarrativeItem, AdverseEventPage, AdverseEventReaction,
    AdverseEventReactionMatch, AdverseEventReactions, AdverseEventRecords,
};
pub use selectors::{
    AdverseEventIdentifierSection, AdverseEventSection, AdverseEventSerious, AdverseEventSource,
    ReportUnit,
};
