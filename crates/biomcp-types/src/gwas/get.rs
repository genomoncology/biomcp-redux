use crate::QueryInvalid;
use biodata::{GwasAssociationId, GwasStudyId};
use serde::{Deserialize, Serialize};
/// Association-only section.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum GwasAssociationSection {
    /// Native locus and reported gene context.
    Loci,
}
/// Study-only sections.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum GwasStudySection {
    /// Shared association rows.
    Associations,
    /// Independent native ancestry rows.
    Ancestries,
}
/// Explicit resource sum; impossible section and identity pairings fail during decoding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum GwasGet {
    /// Read an association.
    Association {
        /// Qualified positive association identity.
        id: GwasAssociationId,
        /// Optional native locus section.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        section: Option<GwasAssociationSection>,
        /// Checked independent section page.
        #[serde(flatten)]
        page: GwasSectionPage,
    },
    /// Read a study.
    Study {
        /// Exact Catalog accession.
        id: GwasStudyId,
        /// Optional study relation section.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        section: Option<GwasStudySection>,
        /// Checked independent section page.
        #[serde(flatten)]
        page: GwasSectionPage,
    },
}
/// Bounded relation window, independent from identity paging.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "PageFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "PageFields"))]
pub struct GwasSectionPage(PageFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct PageFields {
    /// Maximum native rows.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    section_limit: u32,
    /// Rows to skip in this relation.
    #[serde(default)]
    section_offset: u32,
}
fn limit() -> u32 {
    3
}
impl TryFrom<PageFields> for GwasSectionPage {
    type Error = QueryInvalid;
    fn try_from(f: PageFields) -> Result<Self, Self::Error> {
        if !(1..=50).contains(&f.section_limit) {
            return Err(QueryInvalid);
        }
        Ok(Self(f))
    }
}
impl GwasSectionPage {
    /// Selected relation row bound.
    pub fn limit(&self) -> u32 {
        self.0.section_limit
    }
    /// Selected relation offset.
    pub fn offset(&self) -> u32 {
        self.0.section_offset
    }
    /// Select a later relation window.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut page = self.clone();
        page.0.section_offset = offset;
        page
    }
}
