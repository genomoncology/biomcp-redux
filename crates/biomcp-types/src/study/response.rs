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
    /// Whole source headers, without expanded descriptions.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[StudyHeader]>,
    /// Snapshot-scoped matching counts and offset.
    pub page: PageInfo,
    /// Catalog acquisition provenance.
    pub snapshot: StudySnapshot,
}
impl ResponseBody for StudyPage {
    const SCHEMA: &'static str = "biomcp.study-page/1";
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
}
impl ResponseBody for StudyCard {
    const SCHEMA: &'static str = "biomcp.study-card/2";
    type Query = StudyGet;
}
macro_rules! page {
    ($name:ident,$row:ty,$schema:literal) => {
        /// One independently acquired, bounded native metadata collection.
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
                #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        pub struct $name {
            /// Exact study scope of this collection.
            pub study_id: StudyId,
            /// Complete ordered metadata records, never member sample IDs.
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            pub items: Box<[$row]>,
            /// Bounds and source-stated total for this collection.
            pub page: PageInfo,
            /// Live collection acquisition wall time in Unix seconds, distinct from catalog acquisition.
            pub acquired: u64,
        }
        impl ResponseBody for $name {
            const SCHEMA: &'static str = $schema;
            type Query = StudyGet;
        }
    };
}
page!(StudyProfiles, StudyProfile, "biomcp.study-profiles/1");
page!(
    StudySampleLists,
    StudySampleList,
    "biomcp.study-sample-lists/1"
);
/// One native repository directory page; blob sizes do not describe data content.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyFiles {
    /// Exact study directory scope.
    pub study_id: StudyId,
    /// Source-ordered repository file references.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[StudyFile]>,
    /// Bounds within the complete received directory.
    pub page: PageInfo,
    /// Directory acquisition wall time in Unix seconds.
    pub acquired: u64,
    /// Native LICENSE publisher link, independent of the current file page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub license_notice: Option<Text>,
}
impl ResponseBody for StudyFiles {
    const SCHEMA: &'static str = "biomcp.study-files/1";
    type Query = StudyGet;
}
