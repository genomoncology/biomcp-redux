use super::{DrugIdentity, DrugNativeReference, DrugNativeTarget};
use crate::{Attribution, Error, GeneRef, Text};
use serde::{Deserialize, Serialize};

/// A native target or mechanism relation, without an indication inference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields", extend("anyOf" = [{"required":["target"]},{"required":["native_target"]}])))]
pub struct DrugTargetRelation(Fields);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Source drug identity.
    drug: DrugIdentity,
    /// Source-stated target gene, without using mechanism text as a gene label.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "GeneRef"))]
    target: Option<GeneRef>,
    /// Native target identity independent of a gene reference.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "DrugNativeTarget"))]
    native_target: Option<DrugNativeTarget>,
    /// Native relationship label.
    relation: Text,
    /// Native mechanism wording, only when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mechanism: Option<Text>,
    /// Source-native supporting references.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    references: Box<[DrugNativeReference]>,
    /// Native relation provenance.
    attribution: Attribution,
}
impl DrugTargetRelation {
    /// Keep the source gene, native target, or both; reject an unidentified target.
    pub fn new(
        drug: DrugIdentity,
        target: Option<GeneRef>,
        native_target: Option<DrugNativeTarget>,
        relation: Text,
        mechanism: Option<Text>,
        references: Box<[DrugNativeReference]>,
        attribution: Attribution,
    ) -> Result<Self, Error> {
        Self::try_from(Fields {
            drug,
            target,
            native_target,
            relation,
            mechanism,
            references,
            attribution,
        })
    }
    /// Source drug identity.
    pub fn drug(&self) -> &DrugIdentity {
        &self.0.drug
    }
    /// Source-stated gene, only when supplied.
    pub fn target(&self) -> Option<&GeneRef> {
        self.0.target.as_ref()
    }
    /// Original source relationship label.
    pub fn relation(&self) -> &Text {
        &self.0.relation
    }
    /// Native mechanism wording, only when supplied.
    pub fn mechanism(&self) -> Option<&Text> {
        self.0.mechanism.as_ref()
    }
    /// Native supporting references.
    pub fn references(&self) -> &[DrugNativeReference] {
        &self.0.references
    }
    /// Source record provenance.
    pub fn attribution(&self) -> &Attribution {
        &self.0.attribution
    }
}
impl TryFrom<Fields> for DrugTargetRelation {
    type Error = Error;
    fn try_from(value: Fields) -> Result<Self, Error> {
        if value.target.is_none() && value.native_target.is_none() {
            return Err(Error::DrugTargetAbsent);
        }
        Ok(Self(value))
    }
}
fn present<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}
