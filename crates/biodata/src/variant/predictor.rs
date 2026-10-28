//! Source-owned measurements and component provenance, without score interpretation.
use crate::{Attribution, FiniteNumber, Text, VariantRef};
use serde::{Deserialize, Serialize};
/// One unmodified source score, never a locally blended assessment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PredictorScore {
    /// Exact source-qualified subject.
    pub subject: VariantRef,
    /// Published component field path.
    pub method: Text,
    /// Source number, including zero.
    pub score: FiniteNumber,
    /// Source label without code expansion or interpretation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Text>,
    /// Source-owned record and dataset release.
    pub attribution: Attribution,
}
