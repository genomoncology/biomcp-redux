//! The one owner of the reply's status words (1059): a failed, stale or syncing source row makes
//! an answered reply `partial`, and every `partial` reply says at its top what is missing.
use biomcp_types::{Envelope, Incomplete, ResponseBody, Source, SourceState, Status};
use serde_json::Value;

/// Result nodes whose `state` marks requested data that could not be read, including source rows
/// nested in a multi-part reply.
const MISSING_STATES: [&str; 7] = [
    "unavailable",
    "no-text",
    "too-large",
    "skipped",
    "failed",
    "stale",
    "syncing",
];

/// Downgrade a `complete` reply beside a failed source row, then list what a `partial` reply is
/// missing: failed source rows, result nodes marked missing (not skips the caller asked for), and
/// the command's own entries; refused records and source notices only when nothing else explains.
pub(crate) fn settle<B: ResponseBody>(envelope: &mut Envelope<B>, named: Vec<Incomplete>) {
    let mut found: Vec<Incomplete> = envelope
        .sources
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            matches!(
                row.state,
                SourceState::Failed | SourceState::Stale | SourceState::Syncing
            )
        })
        .filter_map(|(index, row)| entry(format!("/sources/{index}"), row.source, row.reason))
        .collect();
    if envelope.status == Status::Complete && !found.is_empty() {
        envelope.status = Status::Partial;
    }
    if envelope.status != Status::Partial {
        return;
    }
    let fallback = envelope.sources.first().map(|row| row.source);
    if let Some(result) = envelope
        .result
        .as_ref()
        .and_then(|result| serde_json::to_value(result).ok())
    {
        walk(&result, "/result".into(), fallback, &mut found);
    }
    for item in named {
        if !found.contains(&item) {
            found.push(item);
        }
    }
    if found.is_empty() {
        found = problems(&envelope.problems);
    }
    if found.is_empty() {
        found = notices(&envelope.sources);
    }
    envelope.incomplete = found;
}

/// A record or field the source sent but BioMCP refused: one entry per source and code.
fn problems(problems: &[biomcp_types::Problem]) -> Vec<Incomplete> {
    let mut seen = Vec::new();
    let mut found = Vec::new();
    for (index, problem) in problems.iter().enumerate() {
        if seen.contains(&(problem.source, problem.code)) {
            continue;
        }
        seen.push((problem.source, problem.code));
        // The problem's own code says why; a source reason would only guess.
        found.extend(
            entry(format!("/problems/{index}"), problem.source, None).map(|item| Incomplete {
                code: Some(problem.code),
                ..item
            }),
        );
    }
    found
}

/// A source's own notice is the only statement of what a short read left out.
fn notices(sources: &[biomcp_types::SourceOutcome]) -> Vec<Incomplete> {
    sources
        .iter()
        .enumerate()
        .filter_map(|(index, row)| {
            let last = row.notices.len().checked_sub(1)?;
            entry(format!("/sources/{index}/notices/{last}"), row.source, None)
        })
        .collect()
}

fn entry(at: String, source: Source, reason: Option<biomcp_types::Reason>) -> Option<Incomplete> {
    Some(Incomplete {
        at: at.parse().ok()?,
        source,
        reason,
        code: None,
    })
}

/// Collect nodes marked missing, naming the nearest source stated on the node or above it.
fn walk(value: &Value, at: String, source: Option<Source>, found: &mut Vec<Incomplete>) {
    match value {
        Value::Object(map) => {
            let source = map
                .get("source")
                .and_then(|value| serde_json::from_value(value.clone()).ok())
                .or(source);
            // A skip the caller or policy asked for (`disabled`) is not missing data.
            let asked_skip = map.get("state").and_then(Value::as_str) == Some("skipped")
                && map.get("reason").and_then(Value::as_str) == Some("disabled");
            let missing = !asked_skip
                && map
                    .get("state")
                    .and_then(Value::as_str)
                    .is_some_and(|state| MISSING_STATES.contains(&state));
            if missing && let Some(source) = source {
                let reason = map
                    .get("reason")
                    .and_then(|value| serde_json::from_value(value.clone()).ok());
                found.extend(entry(at, source, reason));
                return;
            }
            for (key, child) in map {
                walk(child, format!("{at}/{}", escape(key)), source, found);
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                walk(child, format!("{at}/{index}"), source, found);
            }
        }
        _ => {}
    }
}

/// RFC 6901 escaping for one pointer token.
fn escape(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}
