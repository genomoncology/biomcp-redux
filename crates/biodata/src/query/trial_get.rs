use super::gene::{QueryInvalid, present};
use crate::{NctId, Text};
use serde::{Deserialize, Serialize};
/// Retrieve exact trial cards and bounded source sections in input order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct TrialGet(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// One to twenty exact NCT identifiers, in caller order.
    #[cfg_attr(feature="schema", schemars(length(min=1,max=20), extend("x-positional"=0)))]
    input: Box<[NctId]>,
    /// Optional intact source sections. The default returns cards only.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    #[cfg_attr(feature="schema", schemars(extend("x-trailing-section"=true)))]
    section: Box<[TrialSection]>,
    /// Record count per outcomes or arms page; eligibility pages follow the text budget.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Character offset for eligibility or whole-record offset for outcomes and arms.
    #[serde(default)]
    offset: u32,
    /// Checked stored source record, supplied by a continuation.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pool: Option<Text>,
}
fn default_limit() -> u32 {
    10
}
impl TryFrom<Fields> for TrialGet {
    type Error = QueryInvalid;
    fn try_from(v: Fields) -> Result<Self, Self::Error> {
        if !(1..=20).contains(&v.input.len())
            || !(1..=50).contains(&v.limit)
            || (v.pool.is_some() || v.offset != 0)
                && (v.input.len() != 1
                    || v.section.len() != 1
                    || v.section.first() == Some(&TrialSection::All))
            || v.offset != 0 && v.pool.is_none()
        {
            return Err(QueryInvalid);
        }
        Ok(Self(v))
    }
}
impl TrialGet {
    /// Exact checked selectors.
    pub fn input(&self) -> &[NctId] {
        &self.0.input
    }
    /// Requested section words.
    pub fn sections(&self) -> &[TrialSection] {
        &self.0.section
    }
    /// Whole-record page limit.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Source character or record position.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Stored source record capability.
    pub fn pool(&self) -> Option<&Text> {
        self.0.pool.as_ref()
    }
    /// One card or section request, used by typed next steps.
    pub fn single(input: NctId, section: Option<TrialSection>, limit: u32) -> Self {
        Self(Fields {
            input: Box::new([input]),
            section: section.into_iter().collect(),
            limit,
            offset: 0,
            pool: None,
        })
    }
    /// Continue one immutable source record at a checked position.
    pub fn page(&self, input: NctId, section: TrialSection, pool: Text, offset: u32) -> Self {
        let mut next = Self::single(input, Some(section), self.limit());
        next.0.pool = Some(pool);
        next.0.offset = offset;
        next
    }
    /// Initial request without continuation state.
    pub fn identity(&self) -> Self {
        let mut q = self.clone();
        q.0.pool = None;
        q.0.offset = 0;
        q
    }
}
crate::macros::wire_enum! {
    /// Available source protocol sections.
    TrialSection {
        /// Intact criteria and structured age/sex fields.
        Eligibility => "eligibility",
        /// Primary, secondary and other outcome records.
        Outcomes => "outcomes",
        /// Arms and interventions with original relationships.
        Arms => "arms",
        /// Every implemented trial section, each with independent continuations.
        All => "all",
    }
}
