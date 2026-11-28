use super::{StudyGet, StudySearch};
use crate::{PageInfo, ResponseBody, Section, Text};
use biodata::{Study, StudyFile, StudyHeader, StudyId, StudyProfile, StudySampleList};
use serde::{Deserialize, Serialize};
/// Metadata catalog revision and acquisition time; never a biological release date.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudySnapshot {
    /// Immutable bundle revision.
    pub revision: Text,
    /// Catalog acquisition wall time in Unix seconds.
    pub acquired: u64,
}
/// Source-ordered checked study headers from one retained catalog.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyPage {
    // The page comes before the rows (0942): a reader of only the start sees the total.
    /// Snapshot-scoped matching counts and offset.
    pub page: PageInfo,
    /// Whole source headers, without expanded descriptions.
    #[serde(default)]
    pub items: Box<[StudyHeader]>,
    /// Catalog acquisition provenance.
    pub snapshot: StudySnapshot,
}
impl ResponseBody for StudyPage {
    const SCHEMA: &'static str = "biomcp.study-page/6";
    type Query = StudySearch;
}
/// Exact card with independent live section states and catalog provenance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyCard {
    /// Exact publisher study page for this checked source identity.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub source_url: Text,
    /// Native card metadata, preserving all source count distinctions.
    pub study: Study,
    /// Catalog revision and acquisition, apart from each live section's time.
    pub snapshot: StudySnapshot,
    /// Requested molecular-profile page, absent when not requested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Section<StudyProfiles>>,
    /// Requested sample-list metadata page, without member IDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_lists: Option<Section<StudySampleLists>>,
    /// Requested file-reference page, with no downloaded data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Section<StudyFiles>>,
    /// Requested source mutation rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutations: Option<Section<StudyMutations>>,
    /// Requested sample identities and types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub samples: Option<Section<StudySamples>>,
}
impl ResponseBody for StudyCard {
    const SCHEMA: &'static str = "biomcp.study-card/8";
    type Query = StudyGet;
}

page!(
    study,
    StudyProfiles,
    StudyProfile,
    StudyGet,
    "biomcp.study-profiles/5",
    "One independently acquired, bounded native metadata collection."
);
page!(
    study,
    StudySampleLists,
    StudySampleList,
    StudyGet,
    "biomcp.study-sample-lists/5",
    "One independently acquired, bounded native metadata collection."
);
/// One native repository directory page; blob sizes do not describe data content.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyFiles {
    /// Exact study directory scope.
    pub study_id: StudyId,
    /// Rows in the whole collection, equal to `page.total`; the count to report (1022).
    #[serde(default)]
    pub total: u64,
    // The page comes before the rows (0942): a reader of only the start sees the total.
    /// Bounds within the complete received directory.
    pub page: PageInfo,
    /// Source-ordered repository file references.
    #[serde(default)]
    pub items: Box<[StudyFile]>,
    /// Directory acquisition wall time in Unix seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acquired: Option<u64>,
    /// Native LICENSE publisher link, independent of the current file page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub license_notice: Option<Text>,
}
impl ResponseBody for StudyFiles {
    const SCHEMA: &'static str = "biomcp.study-files/4";
    type Query = StudyGet;
}

/// A source sample identity and type, without patient or clinical attributes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudySample {
    /// Native sample ID within this study.
    pub sample_id: Text,
    /// Native sample type, when supplied.
    pub sample_type: Option<Text>,
}
/// A source mutation observation, without clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyMutation {
    /// Native sample ID within this study.
    pub sample_id: Text,
    /// Source protein change, without normalization.
    pub protein_change: Option<Text>,
    /// Native mutation class.
    pub mutation_type: Option<Text>,
}
/// Source sample identities and types from one complete received collection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudySamples {
    /// Exact native study scope.
    pub study_id: StudyId,
    /// Rows in the whole collection, equal to `page.total`; the count to report (1022).
    #[serde(default)]
    pub total: u64,
    // The page comes before the rows (0942): a reader of only the start sees the total.
    /// Bounds within the received collection.
    pub page: PageInfo,
    /// Source sample identities and types in native order.
    #[serde(default)]
    pub items: Box<[StudySample]>,
    /// Source collection acquisition time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acquired: Option<u64>,
}
impl ResponseBody for StudySamples {
    const SCHEMA: &'static str = "biomcp.study-samples/2";
    type Query = StudyGet;
}
/// Source mutation rows and the explicit gene/profile scope used to obtain them.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyMutations {
    /// Exact source study ID.
    pub study_id: StudyId,
    /// Selected mutation profile, including an automatically chosen default.
    pub molecular_profile_id: biodata::StudyProfileId,
    /// All-cases sample list sent to cBioPortal. Older replies may omit it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_list_id: Option<biodata::StudySampleListId>,
    /// Exact cBioPortal gene identifier.
    pub entrez_gene_id: biodata::NcbiGeneId,
    /// Rows in the whole collection, equal to `page.total`; the count to report (1022).
    #[serde(default)]
    pub total: u64,
    /// Bounds within the received collection.
    pub page: PageInfo,
    /// Source mutation observations in source order.
    pub items: Box<[StudyMutation]>,
    /// Source collection acquisition time.
    pub acquired: Option<u64>,
}
impl ResponseBody for StudyMutations {
    const SCHEMA: &'static str = "biomcp.study-mutations/2";
    type Query = StudyGet;
}
/// cBioPortal's own sample counts for one gene's amplification and deep-deletion calls in one
/// discrete copy-number (GISTIC) profile; BioMCP counts nothing itself.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyCopyNumber {
    /// Exact source study ID.
    pub study_id: StudyId,
    /// Selected discrete copy-number profile, including an automatically chosen default.
    pub molecular_profile_id: biodata::StudyProfileId,
    /// Exact cBioPortal gene identifier.
    pub entrez_gene_id: biodata::NcbiGeneId,
    /// cBioPortal's `numberOfSamples`: samples in the profile.
    pub number_of_samples: u32,
    /// cBioPortal's `numberOfSamplesWithAlterationInGene` for call 2, high-level amplification;
    /// 0 when cBioPortal omits it, as it does when no sample has the call.
    pub amplified: u32,
    /// cBioPortal's `numberOfSamplesWithAlterationInGene` for call -2, deep (homozygous) deletion;
    /// 0 when cBioPortal omits it, as it does when no sample has the call.
    pub deep_deleted: u32,
    /// Source acquisition time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acquired: Option<u64>,
}
impl ResponseBody for StudyCopyNumber {
    const SCHEMA: &'static str = "biomcp.study-copy-number/1";
    type Query = StudyGet;
}
