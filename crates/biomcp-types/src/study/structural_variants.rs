use super::StudyGet;
use crate::{PageInfo, ResponseBody, Text};
use biodata::{NcbiGeneId, StudyId, StudyProfileId};
use serde::{Deserialize, Serialize};

/// One source fusion or rearrangement observation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyStructuralVariant {
    /// Native sample ID within this study.
    pub sample_id: Text,
    /// Source symbol at the first site.
    pub site1_hugo_symbol: Option<Text>,
    /// Source symbol at the second site.
    pub site2_hugo_symbol: Option<Text>,
    /// Native variant class.
    pub variant_class: Option<Text>,
    /// Native event label.
    pub event_info: Option<Text>,
}

/// Gene-filtered structural variant observations from one cBioPortal profile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyStructuralVariants {
    /// Exact source study ID.
    pub study_id: StudyId,
    /// Selected structural variant profile.
    pub molecular_profile_id: StudyProfileId,
    /// The gene queried at either site.
    pub entrez_gene_id: NcbiGeneId,
    /// Rows in the whole collection, equal to `page.total`.
    pub total: u64,
    /// Bounds within the received collection.
    pub page: PageInfo,
    /// Source observations in native order.
    pub items: Box<[StudyStructuralVariant]>,
    /// Source collection acquisition time.
    pub acquired: Option<u64>,
}
impl ResponseBody for StudyStructuralVariants {
    const SCHEMA: &'static str = "biomcp.study-structural-variants/1";
    type Query = StudyGet;
}
