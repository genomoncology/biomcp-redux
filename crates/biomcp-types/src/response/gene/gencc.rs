use crate::Text;
use serde::{Deserialize, Serialize};
/// GenCC records in published order without merged classifications.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GenccGene {
    /// Individual submissions in source order.
    pub submissions: Box<[GenccSubmission]>,
}
/// One source submission; no clinical validity is inferred.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GenccSubmission {
    /// Source submission identifier.
    pub id: Text,
    /// Source submission version.
    pub version: Text,
    /// Source disease identifier.
    pub disease_id: Text,
    /// Source disease title.
    pub disease: Text,
    /// Source classification identifier.
    pub classification_id: Text,
    /// Source classification title without inference.
    pub classification: Text,
    /// Source inheritance identifier.
    pub mode_of_inheritance_id: Text,
    /// Source inheritance title.
    pub mode_of_inheritance: Text,
    /// Source submitter identifier.
    pub submitter_id: Text,
    /// Source submitter title.
    pub submitter: Text,
    /// Source submitted_as_date, preserving its spelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_date: Option<Text>,
    /// Source submitted_run_date, preserving its spelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_date: Option<Text>,
}
