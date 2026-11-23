//! Source-selected drug contracts; adapters and command dispatch live elsewhere.
mod purple_book;
pub use purple_book::{DrugPurpleBookPage, PurpleBookLicense, PurpleBookPresentation};
mod purple_book_product;
pub use purple_book_product::{
    DrugPurpleBookProducts, PurpleBookDesignationCount, PurpleBookProduct, PurpleBookReference,
};
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
mod safety_signal;
mod withdrawal;
pub use dsc::{
    DrugSafetyCommunication, DrugSafetyCommunicationCard, DrugSafetyCommunicationPage,
    DrugSafetyMatch,
};
pub use safety_signal::{DrugSafetySignal, DrugSafetySignalPage};
pub use withdrawal::{DrugWithdrawal, DrugWithdrawalPage};
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

/// Primary openFDA fields for the standard prescribing-information headings (21 CFR 201.56(d)(1)).
const NUMBERED_LABEL_HEADINGS: [&str; 17] = [
    "indications_and_usage",
    "dosage_and_administration",
    "dosage_forms_and_strengths",
    "contraindications",
    "warnings_and_cautions",
    "adverse_reactions",
    "drug_interactions",
    "use_in_specific_populations",
    "drug_abuse_and_dependence",
    "overdosage",
    "description",
    "clinical_pharmacology",
    "nonclinical_toxicology",
    "clinical_studies",
    "references",
    "how_supplied",
    "information_for_patients",
];

/// Resolve printed section numbers before splitting a request into individual label reads.
fn normalize_label_numbers(sections: &mut biodata::Text) -> Result<(), crate::QueryInvalid> {
    let selectors = sections.as_str().split(',').map(str::trim);
    let numbered = |selector: &str| {
        selector
            .trim_start_matches(['+', '-'])
            .starts_with(|c: char| c.is_ascii_digit())
    };
    if !selectors.clone().any(numbered) {
        return Ok(());
    }
    let invalid = || crate::QueryInvalid::new(&["label_section"]);
    let headings = selectors
        .map(|selector| {
            if numbered(selector) {
                selector
                    .parse::<usize>()
                    .ok()
                    .and_then(|number| number.checked_sub(1))
                    .and_then(|index| NUMBERED_LABEL_HEADINGS.get(index).copied())
                    .ok_or_else(invalid)
            } else {
                Ok(selector)
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    *sections = biodata::Text::new(&headings.join(",")).map_err(|_| invalid())?;
    Ok(())
}

/// Default submission page size for a native Drugs@FDA application.
pub const DEFAULT_DRUG_APPROVALS_LIMIT: u32 = 10;

mod indication;
pub use indication::{DrugIndicationRow, DrugIndications, DrugIndicationsPart};
