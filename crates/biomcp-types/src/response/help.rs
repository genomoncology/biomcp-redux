use serde::Serialize;
/// Generated command reference from the shipped query schemas and cases.
#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Help {
    /// Commands available at this help path.
    pub commands: Vec<HelpCommand>,
}
impl super::ResponseBody for Help {
    const SCHEMA: &'static str = "biomcp.help/2";
    type Query = Vec<String>;
}
/// One command and its generated arguments.
#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HelpCommand {
    /// Command words after the executable name.
    pub command: Vec<String>,
    /// Query type documentation.
    pub description: String,
    /// Canonical reply contracts; empty for commands with no schema reply.
    #[serde(default)]
    pub reply_schemas: Vec<String>,
    /// Positional arguments and flags.
    pub arguments: Vec<HelpArgument>,
    /// Accepted section words and their documentation.
    pub sections: Vec<HelpWord>,
    /// How to disambiguate section words from inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_rule: Option<String>,
    /// Verbatim case run steps, without replay configuration.
    pub examples: Vec<Vec<String>>,
}
/// One schema argument, retaining its bounds and position.
#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HelpArgument {
    /// Positional name or long flag.
    pub name: String,
    /// Alternative accepted flags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    /// Value used when the argument is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Field documentation.
    pub description: String,
    /// Schema scalar or array type.
    pub value_type: String,
    /// Whether the query requires this argument.
    pub required: bool,
    /// Zero based positional index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    /// Lower numeric or array length bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<u64>,
    /// Upper numeric or array length bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<u64>,
    /// Allowed scalar values.
    pub choices: Vec<String>,
}
/// A documented closed vocabulary word.
#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HelpWord {
    /// Accepted spelling.
    pub word: String,
    /// Type variant documentation.
    pub description: String,
}
