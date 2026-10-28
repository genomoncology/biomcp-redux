//! Reviewed exceptions bind exact schema identities and pattern transitions.
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Copy, Debug)]
pub(super) enum ContractKind {
    Query,
    Reply,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Reviews {
    #[serde(default)]
    pattern_widenings: Vec<PatternWidening>,
    #[serde(default)]
    successors: Vec<Successor>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PatternWidening {
    schema: String,
    path: String,
    from: Option<String>,
    to: Option<String>,
    review: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Successor {
    from: String,
    to: String,
    review: String,
}

impl Reviews {
    pub(super) fn pattern(
        &self,
        name: &str,
        major: u64,
        path: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> bool {
        self.pattern_widenings.iter().any(|entry| {
            entry.schema == format!("{name}/{major}")
                && entry.path == path
                && entry.from.as_deref() == from
                && entry.to.as_deref() == to
                && !entry.review.trim().is_empty()
        })
    }

    pub(super) fn successor(&self, old: &Value, new: &Value) -> bool {
        self.successors.iter().any(|entry| {
            old["$id"] == entry.from && new["$id"] == entry.to && !entry.review.trim().is_empty()
        })
    }
}
