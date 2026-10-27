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
    /// VCV archive or ERepo document version; RCV/SCV selectors match a returned native version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<Text>,
    /// Assertions per page; omitted uses the native source default.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "u32", range(min = 1, max = 50)))]
    limit: Option<u32>,
    /// Assertions to skip in source order.
    #[serde(default)]
    offset: u32,
}
impl TryFrom<Fields> for VariantAssertionsGet {
    type Error = super::gene::QueryInvalid;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        let expected = match fields.source {
            AssertionSource::Clinvar => VariantNamespace::ClinVar,
            AssertionSource::Erepo => VariantNamespace::Ca,
        };
        if fields.input.namespace().ok() != Some(expected)
            || fields.limit.is_some_and(|n| !(1..=50).contains(&n))
            || fields.version.is_some() && fields.assertion.is_none()
        {
            return Err(super::gene::QueryInvalid);
        }
        if let Some(id) = &fields.assertion {
            let valid = match fields.source {
                AssertionSource::Clinvar => {
                    (id.as_str().parse::<crate::VcvId>().is_ok()
                        || id.as_str().parse::<crate::RcvId>().is_ok()
                        || id.as_str().parse::<crate::ScvId>().is_ok())
                        && !id.as_str().contains('.')
                }
                AssertionSource::Erepo => id.as_str().strip_prefix("PCER").is_some_and(|n| {
                    n.len() == 9
                        && n.bytes().all(|b| b.is_ascii_digit())
                        && n.parse::<u64>().is_ok_and(|n| n > 0)
                }),
            };
            if !valid {
                return Err(super::gene::QueryInvalid);
            }
        }
        if let Some(version) = &fields.version {
            let parts = version.as_str().split('.').collect::<Vec<_>>();
            let count = match fields.source {
                AssertionSource::Clinvar => 1,
                AssertionSource::Erepo => 3,
            };
            if parts.len() != count
                || parts.iter().any(|s| {
                    s.is_empty()
                        || !s.bytes().all(|b| b.is_ascii_digit())
                        || s.parse::<u32>().is_err()
                })
                || parts
                    .first()
                    .is_some_and(|s| s.parse::<u32>().is_ok_and(|n| n == 0))
            {
                return Err(super::gene::QueryInvalid);
            }
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
        self.0
            .limit
            .unwrap_or_else(|| self.0.source.default_limit())
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
            limit: None,
            offset: 0,
        }
        .try_into()
    }
    /// Select an existing native assertion and its exact source version.
    pub fn select(&self, record: Text, version: Option<Text>) -> Self {
        let mut q = self.clone();
        q.0.assertion = Some(record);
        q.0.version = version;
        q.0.offset = 0;
        q
    }
    /// Retain the identity while selecting a page size and offset.
    pub fn page(mut self, limit: u32, offset: u32) -> Result<Self, super::gene::QueryInvalid> {
        self.0.limit = Some(limit);
        self.0.offset = offset;
        self.0.try_into()
    }
}
biodata::wire_enum! {
    /// Native assertion provider.
    AssertionSource {
        /// NCBI ClinVar.
        Clinvar=>"clinvar",
        /// ClinGen evidence repository.
        Erepo=>"erepo",
    }
}
impl AssertionSource {
    /// Default native assertion page size.
    pub fn default_limit(self) -> u32 {
        match self {
            Self::Clinvar => 10,
            Self::Erepo => 3,
        }
    }

    /// Source policy identity.
    pub fn source(self) -> crate::Source {
        match self {
            Self::Clinvar => crate::Source::ClinVar,
            Self::Erepo => crate::Source::Erepo,
        }
    }
}
