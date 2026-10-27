use crate::{Build, HgvsText, VariantInput, VariantNamespace};
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
    /// Reference-qualified HGVS, or chromosome genomic HGVS with an explicit build for CAR.
    #[cfg_attr(feature = "schema", schemars(schema_with = "hgvs_schema"))]
    hgvs: String,
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
impl TryFrom<Fields> for VariantNormalize {
    type Error = super::gene::QueryInvalid;
    fn try_from(f: Fields) -> Result<Self, Self::Error> {
        if !(1..=50).contains(&f.limit)
            || f.hgvs.as_str().len() > 2048
            || !valid_hgvs(&f)
            || f.build.is_some_and(|build| !Self::supports_build(&f.hgvs, build))
            || matches!(
                f.source,
                NormalizationSource::All | NormalizationSource::Variantvalidator
            ) && f.build.is_none()
            || f.build
                .is_some_and(|b| !matches!(b, Build::GRCh37 | Build::GRCh38))
        {
            return Err(super::gene::QueryInvalid);
        }
        Ok(Self(f))
    }
}
fn valid_hgvs(fields: &Fields) -> bool {
    fields.hgvs.as_str().parse::<HgvsText>().is_ok()
        || fields.source == NormalizationSource::Car
            && fields.build.is_some()
            && VariantInput::new(fields.hgvs.as_str())
                .and_then(|value| value.namespace())
                .is_ok_and(|namespace| namespace == VariantNamespace::Chromosome)
}
#[cfg(feature = "schema")]
fn hgvs_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "pattern": format!("({}|^chr([1-9][0-9]*|X|Y|M|MT):g\\.[\\x21-\\x7e]+$)", HgvsText::PATTERN)
    })
}
impl VariantNormalize {
    /// Refuse only a provable primary-reference conflict; never assign an unknown accession.
    pub fn supports_build(input: &str, build: Build) -> bool {
        let Ok(accession) = input.parse::<HgvsText>().and_then(|value| value.accession()) else {
            return true;
        };
        let other = if build == Build::GRCh37 { Build::GRCh38 } else { Build::GRCh37 };
        biodata::Chromosome::from_accession(accession, build).is_ok()
            || biodata::Chromosome::from_accession(accession, other).is_err()
    }

    /// Selected service set.
    pub fn source(&self) -> NormalizationSource {
        self.0.source
    }
    /// Unchanged caller description.
    pub fn hgvs(&self) -> &str {
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
