//! Checked regional name requests keep exact source-ID requests unchanged.
use crate::{DrugSection, QueryInvalid};
use biodata::Text;
use serde::{Deserialize, Serialize};
biodata::canonical_enum! {
    /// Regional medicine disclosures remain separate source records.
    DrugRegion, SourceUnknown {
        /// WHO finished pharmaceutical product prequalification only.
        Who => "who",
        /// Independent US, EU and WHO results without joining records.
        All => "all",
    }
}
/// Literal regional name lookup with bounded independent source pages.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct DrugRegionalGet(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Unchanged native product name or ingredient text; no synonym expansion.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    input: Text,
    /// Explicit WHO or independent regional outcomes; no inferred authorization.
    region: DrugRegion,
    /// Only the regulatory disclosure is supported for a regional name lookup.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 1)))]
    section: Box<[DrugSection]>,
    /// Native rows per source, default ten; never select the first candidate.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    section_limit: u32,
    /// Native rows to skip independently within each region.
    #[serde(default)]
    section_offset: u32,
}
fn limit() -> u32 { 10 }
impl TryFrom<Fields> for DrugRegionalGet {
    type Error = QueryInvalid;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        if fields.section.as_ref() != [DrugSection::Regulatory]
            || !(1..=50).contains(&fields.section_limit) {
            return Err(QueryInvalid);
        }
        Ok(Self(fields))
    }
}
impl DrugRegionalGet {
    /// Literal source name text.
    pub fn input(&self) -> &Text { &self.0.input }
    /// Selected regional collection.
    pub fn region(&self) -> DrugRegion { self.0.region }
    /// Independent per-source row bound.
    pub fn limit(&self) -> u32 { self.0.section_limit }
    /// Independent per-source native offset.
    pub fn offset(&self) -> u32 { self.0.section_offset }
    /// Continue the same literal request without changing its region.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut fields = self.0.clone();
        fields.section_offset = offset;
        Self(fields)
    }
}
