use crate::{Build, VariantInput, VariantNamespace, VariantResolver};
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
    /// Resolver for reference-qualified notation; never repairs the input locally.
    #[serde(default = "resolver")]
    resolver: VariantResolver,
    /// Required for chromosome notation; no assembly is guessed.
    #[serde(default, deserialize_with = "super::gene::present", skip_serializing_if = "Option::is_none")]
    build: Option<Build>,
}
fn resolver() -> VariantResolver { VariantResolver::Car }
impl TryFrom<Fields> for VariantGet {
    type Error = super::gene::QueryInvalid;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        Self::new(fields.input, fields.resolver, fields.build)
    }
}
impl VariantGet {
    /// Check coordinate assembly and preserve resolver choice.
    pub fn new(input: VariantInput, resolver: VariantResolver, build: Option<Build>) -> Result<Self, super::gene::QueryInvalid> {
        if resolver == VariantResolver::Variantvalidator && build.is_none() || build.is_some_and(|b| !crate::variant_build(b)) || input.namespace().ok() == Some(VariantNamespace::Chromosome) && build.is_none() {
            return Err(super::gene::QueryInvalid);
        }
        Ok(Self(Fields { input, resolver, build }))
    }
    /// Unchanged checked identifier.
    pub fn input(&self) -> &VariantInput { &self.0.input }
    /// Explicit resolver service.
    pub fn resolver(&self) -> VariantResolver { self.0.resolver }
    /// Caller assembly; never a default.
    pub fn build(&self) -> Option<Build> { self.0.build }
}
