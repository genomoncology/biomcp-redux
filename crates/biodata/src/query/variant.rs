use crate::{Build, VariantInput, VariantNamespace};
use serde::{Deserialize, Serialize};
/// Read one published variant identifier; every source candidate is retained.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct VariantGet(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Published rsID, CAid, ClinVar Variation ID or reference-qualified HGVS.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    input: VariantInput,
    /// Named source sections; omitted by default.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 1)))]
    section: Box<[VariantSection]>,
    /// Resolver for reference-qualified notation; never repairs the input locally.
    #[serde(default = "resolver")]
    resolver: VariantResolver,
    /// Required for chromosome notation; no assembly is guessed.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    build: Option<Build>,
    /// Candidate rows per page; more remain reachable through typed next commands.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Candidate rows to skip in source order.
    #[serde(default)]
    offset: u32,
}
fn default_limit() -> u32 {
    10
}
fn resolver() -> VariantResolver {
    VariantResolver::Car
}
impl TryFrom<Fields> for VariantGet {
    type Error = super::gene::QueryInvalid;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        if !(1..=50).contains(&fields.limit) {
            return Err(super::gene::QueryInvalid);
        }
        let mut query = Self::new(fields.input, fields.resolver, fields.build)?;
        query.0.section = fields.section;
        query.0.limit = fields.limit;
        query.0.offset = fields.offset;
        Ok(query)
    }
}
impl VariantGet {
    /// Check coordinate assembly and preserve resolver choice.
    pub fn new(
        input: VariantInput,
        resolver: VariantResolver,
        build: Option<Build>,
    ) -> Result<Self, super::gene::QueryInvalid> {
        if input.namespace().ok() == Some(VariantNamespace::Unsupported)
            || resolver == VariantResolver::Variantvalidator && build.is_none()
            || build.is_some_and(|b| !matches!(b, Build::Grch37 | Build::Grch38))
            || input.namespace().ok() == Some(VariantNamespace::Chromosome) && build.is_none()
        {
            return Err(super::gene::QueryInvalid);
        }
        Ok(Self(Fields {
            input,
            section: Box::default(),
            resolver,
            build,
            limit: default_limit(),
            offset: 0,
        }))
    }
    /// Explicit section selection; an empty list means identity only.
    pub fn sections(&self) -> &[VariantSection] {
        &self.0.section
    }
    /// Keep the input and page selectors while choosing sections.
    pub fn with_sections(mut self, sections: Box<[VariantSection]>) -> Self {
        self.0.section = sections;
        self
    }
    /// Candidate rows per page.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Candidate page offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Keep all selectors while selecting another source-ordered page.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut query = self.clone();
        query.0.offset = offset;
        query
    }
    /// Unchanged checked identifier.
    pub fn input(&self) -> &VariantInput {
        &self.0.input
    }
    /// Explicit resolver service.
    pub fn resolver(&self) -> VariantResolver {
        self.0.resolver
    }
    /// Caller assembly; never a default.
    pub fn build(&self) -> Option<Build> {
        self.0.build
    }
}

crate::macros::wire_enum! {
    /// Explicit source service for reference-qualified notation.
    VariantResolver {
        /// ClinGen Allele Registry.
        Car => "car",
        /// Mutalyzer.
        Mutalyzer => "mutalyzer",
        /// VariantValidator.
        Variantvalidator => "variantvalidator",
    }
}
impl VariantResolver {
    /// Declared source policy for the selected service.
    pub fn source(self) -> crate::Source {
        match self {
            Self::Car => crate::Source::AlleleRegistry,
            Self::Mutalyzer => crate::Source::Mutalyzer,
            Self::Variantvalidator => crate::Source::VariantValidator,
        }
    }
}

crate::source::canonical_enum! {
    /// Named variant source sections, without automatic expansion of identity cards.
    VariantSection, SourceUnknown {
        /// ClinVar native assertions.
        Clinvar => "clinvar",
        /// ClinGen evidence repository assertions.
        Erepo => "erepo",
        /// gnomAD population frequencies.
        Population => "population",
        /// GWAS Catalog associations.
        Gwas => "gwas",
        /// Every implemented variant section.
        All => "all",
    }
}
