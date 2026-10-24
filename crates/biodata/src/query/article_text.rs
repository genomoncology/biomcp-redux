use super::gene::{QueryInvalid, present};
use crate::{ArticleInput, Text};
use serde::{Deserialize, Serialize};
/// Read one bounded body part or a page of matching paragraphs and table rows.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature="schema", schemars(with="Fields", extend("anyOf"=[{"required":["part"]},{"required":["find"]},{"required":["variant"]}])))]
pub struct ArticleText(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// PubMed or PMC identity. DOI selectors are not accepted by the reader.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=0)))]
    input: ArticleInput,
    /// One-based outline part number.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "u32", range(min = 1)))]
    part: Option<u32>,
    /// Case-insensitive literal text to find.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    find: Option<Text>,
    /// Explicit checked protein substitution.
    #[serde(default, deserialize_with = "present", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "crate::ProteinChange"))]
    variant: Option<crate::ProteinChange>,
    /// Number of matching rows already returned.
    #[serde(default)]
    offset: u32,
}
impl TryFrom<Fields> for ArticleText {
    type Error = QueryInvalid;
    fn try_from(v: Fields) -> Result<Self, QueryInvalid> {
        if matches!(v.input, ArticleInput::Doi(_))
            || [v.part.is_some(), v.find.is_some(), v.variant.is_some()].into_iter().filter(|v| *v).count() != 1
            || v.part == Some(0)
            || v.part.is_some() && v.offset != 0
        {
            return Err(QueryInvalid);
        }
        Ok(Self(v))
    }
}
impl ArticleText {
    /// Build a part selector used in source continuation actions.
    pub fn part(input: ArticleInput, part: u32) -> Result<Self, QueryInvalid> {
        Fields {
            input,
            part: Some(part),
            find: None,
            variant: None,
            offset: 0,
        }
        .try_into()
    }
    /// Original checked identity.
    pub fn input(&self) -> &ArticleInput {
        &self.0.input
    }
    /// Selected part.
    pub fn selected_part(&self) -> Option<u32> {
        self.0.part
    }
    /// Literal search term.
    pub fn find(&self) -> Option<&Text> {
        self.0.find.as_ref()
    }
    /// Literal needles chosen only by an explicit selector.
    pub fn needles(&self) -> Vec<String> {
        self.0.variant.as_ref().map(|v| v.forms().to_vec()).unwrap_or_else(|| self.find().map(|v| vec![v.as_str().to_owned()]).unwrap_or_default())
    }
    /// Explicit variant forms for the reply.
    pub fn forms(&self) -> Option<[String; 4]> { self.0.variant.as_ref().map(crate::ProteinChange::forms) }
    /// Checked hit offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Continue a find page without changing its selector.
    pub fn page(&self, offset: u32) -> Self {
        let mut v = self.clone();
        v.0.offset = offset;
        v
    }
}
