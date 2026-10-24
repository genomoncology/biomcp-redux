use serde::{Deserialize, Serialize};
/// Configuration names for one keyed source; never includes credential values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct KeyHelp {
    /// Optional source-specific key guidance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance: Option<crate::Text>,
    /// Source name.
    pub source: crate::Source,
    /// Configuration file key path.
    pub config: crate::Text,
    /// Environment variable name.
    pub environment: crate::Text,
}
impl super::ResponseBody for KeyHelp {
    const SCHEMA: &'static str = "biomcp.key/1";
    type Query = ();
}
