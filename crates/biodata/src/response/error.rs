//! The fixed schema usage response, including its checked wire literals.
use serde::{Deserialize, Serialize};

// One literal defines both serde and schema, including the fixed message.
macro_rules! singleton {
    ($(#[$doc:meta])* $visibility:vis $name:ident { $( $variant:ident => $literal:literal ),+ }) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = [$($literal),+])))]
        $visibility enum $name {
            $(#[serde(rename = $literal)]
            $variant,)+
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                match String::deserialize(deserializer)?.as_str() {
                    $( $literal => Ok(Self::$variant), )+
                    value => Err(serde::de::Error::unknown_variant(value, &[$($literal),+])),
                }
            }
        }
    };
}

singleton! {
    /// The status of an invalid schema request.
    pub ErrorStatus { Usage => "usage", Local => "local" }
}

singleton! {
    /// The stable error code for an invalid schema request.
    pub ErrorCode { SchemaUsage => "usage.schema", Arguments => "usage.arguments", Local => "local.failure" }
}

singleton! {
    ErrorSchema { V1 => "biomcp.error/1" }
}

singleton! {
    ErrorMessage { SchemaUsage => "expected schema <known artifact id>", Arguments => "expected a supported command with checked arguments", Local => "local configuration, cache or recording failed" }
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

impl ErrorEnvelope {
    /// Refuse invalid command arguments without echoing input.
    pub fn arguments() -> Self {
        Self {
            schema: ErrorSchema::V1,
            status: ErrorStatus::Usage,
            error: ErrorDetail {
                code: ErrorCode::Arguments,
                message: ErrorMessage::Arguments,
            },
        }
    }
    /// Report a local failure without paths or provider diagnostics.
    pub fn local() -> Self {
        Self {
            schema: ErrorSchema::V1,
            status: ErrorStatus::Local,
            error: ErrorDetail {
                code: ErrorCode::Local,
                message: ErrorMessage::Local,
            },
        }
    }
}
