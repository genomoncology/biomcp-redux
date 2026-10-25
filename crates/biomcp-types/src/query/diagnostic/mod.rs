use super::QueryInvalid;
use biodata::{DiagnosticDeviceId, FdaDrugApplication, GeneSymbol, NcbiGeneId, Text};
use serde::{Deserialize, Serialize};
mod get;
pub use get::DiagnosticGet;

biodata::wire_enum! {
    /// One explicit diagnostic discovery source.
    DiagnosticSearchSource { #[doc = "Native Gtr spelling."] Gtr => "gtr", #[doc = "Native FdaCdx spelling."] FdaCdx => "fda-cdx" }
}
biodata::wire_enum! {
    /// One explicit exact diagnostic record source.
    DiagnosticGetSource { #[doc = "Native Gtr spelling."] Gtr => "gtr", #[doc = "Native WhoIvd spelling."] WhoIvd => "who-ivd", #[doc = "Native FdaPma spelling."] FdaPma => "fda-pma" }
}
biodata::wire_enum! {
    /// Source-supported diagnostic sections; no all section.
    DiagnosticSection { #[doc = "Native Genes spelling."] Genes => "genes", #[doc = "Native Conditions spelling."] Conditions => "conditions", #[doc = "Native Methods spelling."] Methods => "methods", #[doc = "Native Regulatory spelling."] Regulatory => "regulatory", #[doc = "Native Supplements spelling."] Supplements => "supplements" }
}
fn limit() -> u32 {
    5
}
/// Checked selected-source diagnostic discovery; no identity resolution or I/O.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct DiagnosticSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Required explicit GTR or FDA companion-table source.
    source: DiagnosticSearchSource,
    /// Exact source GeneID, exclusive with gene symbol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gene_id: Option<NcbiGeneId>,
    /// Exact source gene_symbol field, without alias expansion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gene: Option<GeneSymbol>,
    /// Exact MedGen:CUI identifier only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    condition_id: Option<Text>,
    /// Exact native test_type value.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    test_type: Option<biodata::GtrTestType>,
    /// Exact PMA application or application/supplement identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    device_id: Option<DiagnosticDeviceId>,
    /// Cell-owned typed NDA/BLA application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    drug_application: Option<FdaDrugApplication>,
    /// Exact table-0 biomarker cell; unsupported in the group table.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    biomarker: Option<Text>,
    /// Maximum native rows on this page.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Zero-based native row offset.
    #[serde(default)]
    offset: u32,
    /// Immutable stored bundle revision retained by a continuation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bundle_revision: Option<Text>,
}
impl TryFrom<SearchFields> for DiagnosticSearch {
    type Error = QueryInvalid;
    fn try_from(v: SearchFields) -> Result<Self, Self::Error> {
        let gtr = v.gene_id.is_some()
            || v.gene.is_some()
            || v.condition_id.is_some()
            || v.test_type.is_some();
        let cdx = v.device_id.is_some() || v.drug_application.is_some() || v.biomarker.is_some();
        if !(1..=50).contains(&v.limit)
            || (v.gene.is_some() && v.gene_id.is_some())
            || (v.source == DiagnosticSearchSource::Gtr && cdx)
            || (v.source == DiagnosticSearchSource::FdaCdx && gtr)
            || v.device_id
                .as_ref()
                .is_some_and(|id| !id.as_str().starts_with('P'))
            || v.condition_id.as_ref().is_some_and(|id| {
                id.as_str()
                    .strip_prefix("MedGen:C")
                    .is_none_or(|id| id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()))
            })
        {
            return Err(QueryInvalid);
        }
        Ok(Self(v))
    }
}
impl DiagnosticSearch {
    /// Explicit source, without fallback.
    pub fn source(&self) -> DiagnosticSearchSource {
        self.0.source
    }
    /// Exact source GeneID.
    pub fn gene_id(&self) -> Option<NcbiGeneId> {
        self.0.gene_id
    }
    /// Exact source symbol.
    pub fn gene(&self) -> Option<&GeneSymbol> {
        self.0.gene.as_ref()
    }
    /// Exact source MedGen CUI.
    pub fn condition_id(&self) -> Option<&Text> {
        self.0.condition_id.as_ref()
    }
    /// Exact native test type.
    pub fn test_type(&self) -> Option<biodata::GtrTestType> {
        self.0.test_type
    }
    /// Selected PMA base or supplement.
    pub fn device_id(&self) -> Option<&DiagnosticDeviceId> {
        self.0.device_id.as_ref()
    }
    /// Owned NDA/BLA application.
    pub fn drug_application(&self) -> Option<&FdaDrugApplication> {
        self.0.drug_application.as_ref()
    }
    /// Whole exact biomarker cell.
    pub fn biomarker(&self) -> Option<&Text> {
        self.0.biomarker.as_ref()
    }
    /// Native rows per page.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Native row offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Selected immutable bundle revision.
    pub fn bundle_revision(&self) -> Option<&Text> {
        self.0.bundle_revision.as_ref()
    }
    /// Continue without changing source, filters or revision.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut v = self.clone();
        v.0.offset = offset;
        v
    }
    /// Bind a stored query to its immutable source snapshot.
    pub fn with_bundle_revision(&self, revision: Text) -> Self {
        let mut v = self.clone();
        v.0.bundle_revision = Some(revision);
        v
    }
}
