use crate::{Build, HgvsText};
use serde::{Deserialize, Serialize};
/// Send unchanged reference-qualified notation to explicitly named services.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct VariantNormalize(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Explicit service; all preserves independent results without consensus.
    source: NormalizationSource,
    /// Unchanged reference-qualified HGVS; no local molecular parse.
    hgvs: HgvsText,
    /// Caller assembly; required when VariantValidator is selected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    build: Option<Build>,
    /// Source mapping rows per service, with typed continuation for remaining rows.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Source mapping rows to skip per service.
    #[serde(default)]
    offset: u32,
}
fn limit() -> u32 {
    10
}
crate::query::validation::checked_query!(VariantNormalize, Fields);
impl TryFrom<Fields> for VariantNormalize {
    type Error = super::gene::QueryInvalid;
    fn try_from(f: Fields) -> Result<Self, Self::Error> {
        if !(1..=50).contains(&f.limit) {
            return Err(super::gene::QueryInvalid::new(&["limit"]));
        }
        if f.hgvs.as_str().len() > 2048 {
            return Err(super::gene::QueryInvalid::new(&["hgvs"]));
        }
        if matches!(
            f.source,
            NormalizationSource::All | NormalizationSource::Variantvalidator
        ) && f.build.is_none()
        {
            return Err(super::gene::QueryInvalid::new(&["source", "build"]));
        }
        if f.build
            .is_some_and(|b| !matches!(b, Build::GRCh37 | Build::GRCh38))
        {
            return Err(super::gene::QueryInvalid::new(&["build"]));
        }
        Ok(Self(f))
    }
}
impl VariantNormalize {
    /// Selected service set.
    pub fn source(&self) -> NormalizationSource {
        self.0.source
    }
    /// Unchanged caller description.
    pub fn hgvs(&self) -> &HgvsText {
        &self.0.hgvs
    }
    /// Explicit caller assembly.
    pub fn build(&self) -> Option<Build> {
        self.0.build
    }
    /// Mapping page size.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Mapping page offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Preserve selectors for another native mapping page.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut q = self.clone();
        q.0.offset = offset;
        q
    }
}
biodata::wire_enum! {
    /// Explicit normalization service selection.
    NormalizationSource {
        /// ClinGen Allele Registry.
        Car => "car",
        /// Mutalyzer.
        Mutalyzer => "mutalyzer",
        /// VariantValidator.
        Variantvalidator => "variantvalidator",
        /// Every service independently, without a consensus.
        All => "all",
    }
}
