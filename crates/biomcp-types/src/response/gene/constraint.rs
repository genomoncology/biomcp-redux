use crate::{AccessionBase, FiniteNumber, Text};
use serde::{Deserialize, Serialize};
/// Source constraint metrics for the source's preferred transcript.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Constraint {
    /// Probability of loss-of-function intolerance.
    pub pli: FiniteNumber,
    /// Upper confidence bound of the observed-to-expected loss-of-function ratio.
    pub loeuf: FiniteNumber,
    /// Observed-to-expected loss-of-function ratio.
    pub oe_lof: FiniteNumber,
    /// Missense Z score.
    pub mis_z: FiniteNumber,
    /// Synonymous Z score.
    pub syn_z: FiniteNumber,
    /// Source flags, omitted when empty.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub flags: Box<[Text]>,
    /// Source MANE Select transcript, otherwise the source canonical transcript.
    pub transcript: AccessionBase,
    /// Source dataset family, not a minor release identifier.
    pub version: Text,
}
