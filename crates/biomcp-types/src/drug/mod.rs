//! Source-selected drug contracts; adapters and command dispatch live elsewhere.
mod recall;
pub use recall::{
    DrugRecallCard, DrugRecallClassification, DrugRecallCountUnit, DrugRecallHeader, DrugRecallPage,
};
mod civic;
mod fda;
pub use civic::{
    DrugCivicClaim, DrugCivicMember, DrugCivicPage, DrugCivicProfile, DrugCivicStreams,
    DrugOncokbLookup, DrugOncokbMissing,
};
pub use fda::{
    DrugFdaCandidate, DrugFdaDocument, DrugFdaIngredient, DrugFdaLatestSubmission,
    DrugFdaOriginalApproval, DrugFdaPage, DrugFdaProduct, DrugFdaProperty, DrugFdaSubmission,
    DrugFdaSupplementPurpose,
};
mod query;
mod response;
mod search;
mod source;
pub use query::DrugGet;
mod section_name;
pub use section_name::DrugSection;
mod label_find;
pub use label_find::{DrugLabelExcerpt, DrugLabelMatches};
pub use response::{
    DateRoleLabel, DrugApprovals, DrugCard, DrugCivic, DrugHeader, DrugInteractions, DrugLabel,
    DrugLabelFragment, DrugLabelText, DrugLevelCount, DrugNameResolution, DrugPage, DrugRecords,
    DrugRegulatory, DrugRegulatoryContext, DrugTargets, DrugXrefs,
};
pub use search::{DrugFdaSort, DrugSearch, LabelApplicationType};
mod sections;
pub use sections::{
    DrugApprovalsPart, DrugCivicPart, DrugInteractionsPart, DrugLabelPart, DrugRegulatoryPart,
    DrugSectionBody, DrugSectionPart, DrugSectionReply, DrugSections, DrugTargetsPart,
    DrugXrefsPart,
};

pub use source::DrugSource;

mod name_selector;
pub use name_selector::DrugNameSelector;

mod markets;
pub use markets::{DrugMarket, DrugMarketState, DrugMarkets, DrugMarketsGet};

mod history;
pub use history::{
    DrugLabelHistory, LabelHistorySection, LabelVersion, LabelVersionMatch, LabelVersionState,
};
mod dsc;
pub use dsc::{
    DrugSafetyCommunication, DrugSafetyCommunicationCard, DrugSafetyCommunicationPage,
    DrugSafetyMatch,
};
mod eml;
pub use eml::{
    DrugEssentialMedicineDecision, DrugEssentialMedicineListing, DrugEssentialMedicines,
};
mod who;
pub use who::{DrugWhoCard, DrugWhoGet, DrugWhoPage, DrugWhoRow, WhoDateStatus};
#[cfg(feature = "schema")]
mod command;
#[cfg(feature = "schema")]
pub use command::DrugGetCommand;

mod name_match;
pub use name_match::DrugNameMatch;

mod label_headings;
pub use label_headings::OPENFDA_LABEL_HEADINGS;

/// Section names the printed label uses where openFDA files the section under another field,
/// as (printed name, openFDA field).
const PRINTED_LABEL_HEADINGS: [(&str, &str); 1] =
    [("warnings_and_precautions", "warnings_and_cautions")];

/// The openFDA label field a section name reads: the field of that name, or the field the
/// printed label's name for the section is filed under.
pub fn openfda_label_field(name: &str) -> Option<&'static str> {
    OPENFDA_LABEL_HEADINGS
        .iter()
        .find(|heading| **heading == name)
        .or_else(|| {
            PRINTED_LABEL_HEADINGS
                .iter()
                .find(|(printed, _)| *printed == name)
                .map(|(_, field)| field)
        })
        .copied()
}

/// Default submission page size for a native Drugs@FDA application.
pub const DEFAULT_DRUG_APPROVALS_LIMIT: u32 = 10;

mod indication;
pub use indication::{DrugIndicationRow, DrugIndications, DrugIndicationsPart};
