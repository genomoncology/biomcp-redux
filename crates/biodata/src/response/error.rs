//! The fixed schema usage response, including its checked wire literals.
use serde::{Deserialize, Serialize};

/// The status of an invalid schema request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ErrorStatus {
    /// The request cannot be used.
    #[serde(rename = "usage")]
    Usage,
}

/// The stable error code for an invalid schema request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ErrorCode {
    /// The command requires a known schema artifact ID.
    #[serde(rename = "usage.schema")]
    SchemaUsage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
enum ErrorSchema {
    #[serde(rename = "biomcp.error/1")]
    V1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
enum ErrorMessage {
    #[serde(rename = "expected schema <known artifact id>")]
    SchemaUsage,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct ErrorDetail {
    code: ErrorCode,
    message: ErrorMessage,
}

/// A versioned error response that never contains caller input.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ErrorEnvelope {
    schema: ErrorSchema,
    status: ErrorStatus,
    error: ErrorDetail,
}

impl ErrorEnvelope {
    /// Construct the fixed response for every invalid schema request.
    pub fn schema_usage() -> Self {
        Self {
            schema: ErrorSchema::V1,
            status: ErrorStatus::Usage,
            error: ErrorDetail {
                code: ErrorCode::SchemaUsage,
                message: ErrorMessage::SchemaUsage,
            },
        }
    }
}
