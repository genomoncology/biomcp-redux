//! Fixed wire values for error statuses, codes, and schema identifiers.

// One literal defines both serde and schema, including the fixed message.
macro_rules! singleton {
    ($(#[$doc:meta])* $visibility:vis $name:ident { $( $variant:ident => $literal:literal ),+ }) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = [$($literal),+])))]
        $visibility enum $name {
            // Double backticks: one literal holds single backticks.
            $(#[doc = concat!("`` ", $literal, " ``")]
            #[serde(rename = $literal)]
            $variant,)+
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                match <String as serde::Deserialize>::deserialize(deserializer)?.as_str() {
                    $( $literal => Ok(Self::$variant), )+
                    value => Err(serde::de::Error::unknown_variant(value, &[$($literal),+])),
                }
            }
        }
    };
}

singleton! {
    /// The status of a command usage error, local failure or expired result.
    pub ErrorStatus { Usage => "usage", Local => "local", Expired => "expired" }
}

singleton! {
    /// Stable error codes for command usage, local failures and expired results.
    pub ErrorCode { SchemaUsage => "usage.schema",
 Arguments => "usage.arguments", McpUnavailable => "usage.mcp_unavailable",
 UnknownCommand => "usage.unknown_command", Retired => "usage.retired",
 Local => "local.failure", PoolExpired => "pool.expired", PoolMismatch => "usage.pool_mismatch",
 TooManyPmids => "usage.too_many_pmids", TooBroad => "usage.too_broad",
 UnsupportedFilter => "usage.unsupported_filter" }
}

singleton! {
    pub(super) ErrorSchema { V2 => "biomcp.error/4" }
}

pub(super) use singleton;
