use crate::{Text, macros::text_value};
use serde::{Deserialize, Serialize};
/// A checked human Reactome stable pathway identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReactomeId(Box<str>);
impl ReactomeId {
    /// Check the human namespace and canonical positive decimal identifier.
    pub fn new(value: &str) -> Result<Self, ReactomeIdInvalid> {
        let suffix = value.strip_prefix("R-HSA-").ok_or(ReactomeIdInvalid)?;
        if suffix.is_empty() || suffix.starts_with('0') || !suffix.bytes().all(|ch| ch.is_ascii_digit()) {
            return Err(ReactomeIdInvalid);
        }
        Ok(Self(value.into()))
    }
    /// Borrow the stable identifier.
    pub fn as_str(&self) -> &str { &self.0 }
}
impl std::str::FromStr for ReactomeId {
    type Err = ReactomeIdInvalid;
    fn from_str(value: &str) -> Result<Self, Self::Err> { Self::new(value) }
}
impl std::fmt::Display for ReactomeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.as_str()) }
}
text_value!(ReactomeId, "ReactomeId", "^R-HSA-[1-9][0-9]*$");
/// An identifier outside the canonical human Reactome namespace.
#[derive(Clone, Copy, Debug)]
pub struct ReactomeIdInvalid;
impl std::fmt::Display for ReactomeIdInvalid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("id.reactome.invalid") }
}
impl std::error::Error for ReactomeIdInvalid {}
/// A pathway reference without membership or hierarchy data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PathwayRef {
    /// Human stable identifier.
    #[serde(rename = "stId")]
    pub st_id: ReactomeId,
    /// Whole source pathway name.
    #[serde(rename = "displayName")]
    pub display_name: Text,
}
/// A bounded list with the total before its cap was applied.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Capped<T> {
    /// References in stable source order.
    pub items: Box<[T]>,
    /// Distinct references before capping.
    pub total: usize,
}
