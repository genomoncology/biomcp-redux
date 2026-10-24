use crate::{Text, VariantInput, VariantNamespace};
use serde::{Deserialize, Serialize};
/// Page assertions from one explicitly selected evidence source.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct VariantAssertionsGet(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Checked Variation ID for ClinVar, or CAid for ERepo.
    #[cfg_attr(feature="schema",schemars(extend("x-positional"=0)))]
    input: VariantInput,
    /// One native source; no fallback or text search.
    source: AssertionSource,
    /// Native assertion accession or document ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    assertion: Option<Text>,
    /// Selected record version, never borrowed from another record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<Text>,
    /// Assertions per page.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Assertions to skip in source order.
    #[serde(default)]
    offset: u32,
}
fn limit() -> u32 {
    3
}
impl TryFrom<Fields> for VariantAssertionsGet {
    type Error = super::gene::QueryInvalid;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        let expected = match fields.source {
            AssertionSource::Clinvar => VariantNamespace::ClinVar,
            AssertionSource::Erepo => VariantNamespace::Ca,
        };
        if fields.input.namespace().ok() != Some(expected)
            || !(1..=50).contains(&fields.limit)
            || fields.version.is_some() && fields.assertion.is_none()
        {
            return Err(super::gene::QueryInvalid);
        }
        Ok(Self(fields))
    }
}
impl VariantAssertionsGet {
    /// Checked source-specific identity.
    pub fn input(&self) -> &VariantInput {
        &self.0.input
    }
    /// Explicit evidence source.
    pub fn source(&self) -> AssertionSource {
        self.0.source
    }
    /// Native record selection.
    pub fn assertion(&self) -> Option<&Text> {
        self.0.assertion.as_ref()
    }
    /// Native version selection.
    pub fn version(&self) -> Option<&Text> {
        self.0.version.as_ref()
    }
    /// Selected page size.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Source-order page offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Continue the same source result.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut q = self.clone();
        q.0.offset = offset;
        q
    }
    /// Construct a checked source query without selecting an assertion.
    pub fn new(
        input: VariantInput,
        source: AssertionSource,
    ) -> Result<Self, super::gene::QueryInvalid> {
        Fields {
            input,
            source,
            assertion: None,
            version: None,
            limit: limit(),
            offset: 0,
        }
        .try_into()
    }
    /// Retain the identity while selecting a page size and offset.
    pub fn page(mut self, limit: u32, offset: u32) -> Result<Self, super::gene::QueryInvalid> {
        self.0.limit = limit;
        self.0.offset = offset;
        self.0.try_into()
    }
}
crate::macros::wire_enum! {
    /// Native assertion provider.
    AssertionSource {
        /// NCBI ClinVar.
        Clinvar=>"clinvar",
        /// ClinGen evidence repository.
        Erepo=>"erepo",
    }
}
impl AssertionSource {
    /// Source policy identity.
    pub fn source(self) -> crate::Source {
        match self {
            Self::Clinvar => crate::Source::ClinVar,
            Self::Erepo => crate::Source::Erepo,
        }
    }
}
