use super::QueryInvalid;
use biodata::{DiagnosticDeviceId, FdaDrugApplication, GeneSymbol, NcbiGeneId, Text};
use serde::{Deserialize, Serialize};
mod get;
pub use get::DiagnosticGet;

biodata::wire_enum! {
    /// One explicit diagnostic discovery source.
    DiagnosticSearchSource { #[doc = "Native Gtr spelling."] Gtr => "gtr",
 #[doc = "Native FdaCdx spelling."] FdaCdx => "fda-cdx",
 #[doc = "Stored WHO product headers."] WhoIvd => "who-ivd" }
}
biodata::wire_enum! {
    /// One explicit exact diagnostic record source.
    DiagnosticGetSource { #[doc = "Native Gtr spelling."] Gtr => "gtr",
 #[doc = "Native WhoIvd spelling."] WhoIvd => "who-ivd",
 #[doc = "Native FdaPma spelling."] FdaPma => "fda-pma",
 #[doc = "Native 510(k) clearance spelling."] Fda510k => "fda-510k" }
}
biodata::wire_enum! {
    described
    /// Source-supported diagnostic sections; no all section.
    DiagnosticSection {
        #[doc = "GTR native gene rows; five per page."] Genes => "genes",
        #[doc = "GTR native condition rows; five per page."] Conditions => "conditions",
        #[doc = "GTR native method entries; five per page."] Methods => "methods",
        #[doc = "Selected PMA submission statement; one fragment per page."] Regulatory => "regulatory",
        #[doc = "PMA base application supplement headers; five per page."] Supplements => "supplements"
    }
}
fn limit() -> u32 {
    5
}
/// Search one diagnostic source. A listing is not authorization; a companion row is not a treatment recommendation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct DiagnosticSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Explicit native GTR, WHO IVD or FDA companion discovery.
    source: DiagnosticSearchSource,
    /// Exact source GeneID, exclusive with gene symbol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gene_id: Option<NcbiGeneId>,
    /// Exact source gene_symbol field, without alias expansion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gene: Option<GeneSymbol>,
    /// Exact MedGen:CUI identifier only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(regex(pattern = "^MedGen:C[0-9]+$")))]
    condition_id: Option<Text>,
    /// Exact native test_type value.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    test_type: Option<biodata::GtrTestType>,
    /// Exact PMA application, application/supplement or native K identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    device_id: Option<DiagnosticDeviceId>,
    /// Cell-owned typed NDA/BLA application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    drug_application: Option<FdaDrugApplication>,
    /// Exact table-0 biomarker cell; unsupported in the group table.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    biomarker: Option<Text>,
    /// Literal whole-word phrase in WHO's Pathogen/Disease/Marker cell.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    marker: Option<Text>,
    /// Literal whole-word phrase in WHO's Manufacturer name cell.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    manufacturer: Option<Text>,
    /// Five results by default; at most fifty source rows or product headers.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Zero-based source result offset.
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
        let who = v.marker.is_some() || v.manufacturer.is_some();
        if !(1..=50).contains(&v.limit)
            || [
                v.gene.is_some(),
                v.gene_id.is_some(),
                v.condition_id.is_some(),
            ]
            .into_iter()
            .filter(|v| *v)
            .count()
                > 1
            || (v.source == DiagnosticSearchSource::Gtr && (cdx || who))
            || (v.source == DiagnosticSearchSource::FdaCdx && (gtr || who))
            || (v.source == DiagnosticSearchSource::WhoIvd && (gtr || cdx))
            || [v.marker.as_ref(), v.manufacturer.as_ref()]
                .into_iter()
                .flatten()
                .any(|v| !v.as_str().chars().any(char::is_alphanumeric))
            || v.device_id
                .as_ref()
                .is_some_and(|id| !(id.as_str().starts_with('P') || id.as_str().starts_with('K')))
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
    /// Selected PMA base, supplement or native K identity.
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
    /// Literal WHO marker words.
    pub fn marker(&self) -> Option<&Text> {
        self.0.marker.as_ref()
    }
    /// Literal WHO manufacturer words.
    pub fn manufacturer(&self) -> Option<&Text> {
        self.0.manufacturer.as_ref()
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
