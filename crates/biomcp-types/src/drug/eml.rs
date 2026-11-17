//! WHO Model List of Essential Medicines listings as the WHO eEML publishes them.
use super::{DrugGet, DrugHeader, DrugRecords};
use crate::{ResponseBody, Section};
use biodata::{PartialDate, Text};
use serde::{Deserialize, Serialize};

/// One medicine's listings on the WHO Model List of Essential Medicines, read from the WHO eEML.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugEssentialMedicines {
    /// The eEML medicine: its eEML number, name and page.
    pub identity: DrugHeader,
    /// One listing per eEML recommendation (one indication each), in eEML number order; empty
    /// when the eEML names the medicine but lists it for nothing.
    pub listings: Section<DrugRecords<DrugEssentialMedicineListing>>,
    /// What the eEML is and is not: WHO says the PDF lists are the binding versions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hint: Option<Text>,
}

/// One eEML recommendation: the medicine's listing for one indication.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugEssentialMedicineListing {
    /// eEML recommendation number.
    pub id: Text,
    /// The recommendation's eEML page.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Text,
    /// The listed medicine, when the drug asked for is only its therapeutic alternative here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternative_to: Option<Text>,
    /// `core` or `complementary`: which part of the Model List holds this listing; any other
    /// eEML value as published.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list: Option<Text>,
    /// EML section, numbered as the Model List numbers it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<Text>,
    /// Indication, as the eEML names it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indication: Option<Text>,
    /// The indication's ICD-11 code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icd11: Option<Text>,
    /// The eEML's status for this indication, as published, such as `AddedToEml`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Text>,
    /// The earliest Expert Committee decision to add this listing: the edition it was added in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added: Option<PartialDate>,
    /// Every Expert Committee decision on this listing, latest first, as the eEML gives them.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub decisions: Box<[DrugEssentialMedicineDecision]>,
    /// Whether the listing is also on the EML for children (EMLc).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<bool>,
    /// Therapeutic alternatives the listing names (the Model List's square box).
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub alternatives: Box<[Text]>,
    /// ATC codes of the listed medicine.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub atc: Box<[Text]>,
    /// Listed formulations and strengths.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub formulations: Box<[Text]>,
    /// The Expert Committee's recommendation text; omitted when it repeats the listing before
    /// it. The background and evidence summaries are on the listing's page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committee: Option<Text>,
}

/// One Expert Committee decision on a listing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugEssentialMedicineDecision {
    /// The decision as the eEML types it, such as `DecisionToAdd`.
    pub decision: Text,
    /// The decision's date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<PartialDate>,
    /// `core` or `complementary`, as for the listing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list: Option<Text>,
}

impl ResponseBody for DrugEssentialMedicines {
    const SCHEMA: &'static str = "biomcp.drug-essential-medicines/1";
    type Query = DrugGet;
}
