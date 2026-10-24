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
            $(#[doc = $literal]
            #[serde(rename = $literal)]
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
    pub ErrorStatus { Usage => "usage", Local => "local", Expired => "expired" }
}

singleton! {
    /// The stable error code for an invalid schema request.
    pub ErrorCode { SchemaUsage => "usage.schema", Arguments => "usage.arguments", Retired => "usage.retired", Local => "local.failure", PoolExpired => "pool.expired", PoolMismatch => "usage.pool_mismatch" }
}

singleton! {
    ErrorSchema { V1 => "biomcp.error/1" }
}

singleton! {
    /// Fixed input-free error and migration guidance.
    pub ErrorMessage { SchemaUsage => "expected schema <known artifact id>", Arguments => "expected a supported command with checked arguments", Local => "local configuration, cache or recording failed",
        Gene => "use: get gene <gene>", Pathways => "use: get gene <gene> pathways",
        Trials => "use: search trial --gene <gene>", Articles => "use: search article --gene <gene>",
        Drugs => "use: search drug --target <gene>", CellLines => "moved to the cell line entity",
        Variant => "moved to the variant entity", LocusGroup => "use: --locus-group",
        ArticleText => "use: search article <text>",
        ScoreBlends => "no score blends in 1.0; each source keeps its own order",
        ArticlePdf => "use: get article <id> pdf", Since => "use: --since", Until => "use: --until",
        CitationSort => "use: --sort citations", GeneOption => "use: --gene",
        DiseaseOption => "use: --disease", AuthorOption => "use: --author",
        Supplements => "use: supplements", Supplement => "use: article supplement <id> <file>",
        Fulltext => "use: fulltext", ArticlePart => "use: article text <id> --part <n>",
        SupplementFind => "use: article supplement <id> <file> --find <text>",
        Annotations => "use: get article <id> annotations", ArticleBatch => "use: get article <id>...",
        VariantArticles => "use: search article --variant <variant>", DiseaseArticles => "use: search article --disease <disease>",
        DrugArticles => "use: search article --drug <drug>", Pathway => "moved to the pathway entity",
        Band => "use: --band", Dropped => "dropped in 1.0", Expression => "use: expression",
        Drug => "moved to the drug entity", Diagnostic => "moved to the diagnostic entity", Deferred => "deferred", PoolExpired => "pool expired; restart the search", PoolMismatch => "pool selectors do not match this search"
    }
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
    /// Typed restart when a snapshot cannot answer this query.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next: Vec<super::Step>,
}

impl ErrorEnvelope {
    /// Construct the fixed response for every invalid schema request.
    pub fn schema_usage() -> Self {
        Self {
            next: Vec::new(),
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
    /// Report fixed migration guidance without accepting caller input.
    pub fn retired(message: ErrorMessage) -> Self {
        Self {
            next: Vec::new(),
            schema: ErrorSchema::V1,
            status: ErrorStatus::Usage,
            error: ErrorDetail {
                code: ErrorCode::Retired,
                message,
            },
        }
    }
    /// Refuse invalid command arguments without echoing input.
    pub fn arguments() -> Self {
        Self {
            next: Vec::new(),
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
            next: Vec::new(),
            schema: ErrorSchema::V1,
            status: ErrorStatus::Local,
            error: ErrorDetail {
                code: ErrorCode::Local,
                message: ErrorMessage::Local,
            },
        }
    }
}

impl ErrorEnvelope {
    /// Report an unavailable or mismatched pool with its typed restart.
    pub fn pool(mismatch: bool, next: super::Step) -> Self {
        Self {
            schema: ErrorSchema::V1,
            status: if mismatch {
                ErrorStatus::Usage
            } else {
                ErrorStatus::Expired
            },
            error: ErrorDetail {
                code: if mismatch {
                    ErrorCode::PoolMismatch
                } else {
                    ErrorCode::PoolExpired
                },
                message: if mismatch {
                    ErrorMessage::PoolMismatch
                } else {
                    ErrorMessage::PoolExpired
                },
            },
            next: vec![next],
        }
    }
}
