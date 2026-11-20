//! The one owner of the reply's status words (1059): a failed, stale or syncing source row makes
//! an answered reply `partial`, and every `partial` reply says at its top what is missing.
use biomcp_types::{Envelope, Incomplete, ResponseBody, Source, SourceState, Status};
use serde_json::Value;

/// Result nodes whose `state` marks requested data that could not be read, including source rows
/// nested in a multi-part reply. A PDF whose text was cut short (`partial`), read as gibberish,
/// damaged, or refused because its read was stopped at the memory or time limit counts (1075).
const MISSING_STATES: [&str; 11] = [
    "unavailable",
    "no-text",
    "too-large",
    "garbled",
    "partial",
    "damaged",
    "refused",
    "skipped",
    "failed",
    "stale",
    "syncing",
];

/// Downgrade a `complete` reply beside a failed source row, then list what a `partial` reply is
/// missing: failed source rows, result nodes marked missing, and the command's own entries;
/// refused records and source notices only when nothing else explains.
///
/// A section skipped because the caller disabled its source is not missing data, for every
/// section of every entity, wherever a command marked it. When no sibling section of that record
/// was read, though, nothing the caller asked for there was answered: the skip is listed with
/// reason `disabled` and the reply is `partial`, never `complete` with silently empty content.
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
    let mut marks = Marks::default();
    if let Some(result) = envelope
        .result
        .as_ref()
        .and_then(|result| serde_json::to_value(result).ok())
    {
        let fallback = envelope.sources.first().map(|row| row.source);
        walk(&result, "/result".into(), fallback, false, &mut marks);
    }
    if envelope.status == Status::Complete && !(found.is_empty() && marks.disabled.is_empty()) {
        envelope.status = Status::Partial;
    }
    if envelope.status != Status::Partial {
        return;
    }
    for item in marks.missing.into_iter().chain(marks.disabled).chain(named) {
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
    // A reply whose only gap is a skip the caller asked for is whole.
    if found.is_empty() && marks.quiet {
        envelope.status = Status::Complete;
    }
    envelope.incomplete = found;
}

/// What the walk over a reply's result found.
#[derive(Default)]
struct Marks {
    /// Nodes marked missing.
    missing: Vec<Incomplete>,
    /// Caller-disabled skips with no read sibling, listed with reason `disabled`.
    disabled: Vec<Incomplete>,
    /// A caller-disabled skip beside read data, which is not missing.
    quiet: bool,
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

fn state(value: &Value) -> Option<&str> {
    value.get("state").and_then(Value::as_str)
}

/// A node skipped because the caller disabled its source.
fn disabled(value: &Value) -> bool {
    state(value) == Some("skipped")
        && value.get("reason").and_then(Value::as_str) == Some("disabled")
}

/// Collect nodes marked missing, naming the nearest source stated on the node or above it.
/// `listed` says a caller-disabled skip here has no read sibling.
fn walk(value: &Value, at: String, source: Option<Source>, listed: bool, marks: &mut Marks) {
    match value {
        Value::Object(map) => {
            let source = map
                .get("source")
                .and_then(|value| serde_json::from_value(value.clone()).ok())
                .or(source);
            if disabled(value) {
                match source {
                    Some(source) if listed => marks.disabled.extend(entry(
                        at,
                        source,
                        Some(biomcp_types::Reason::Disabled),
                    )),
                    _ => marks.quiet = true,
                }
                return;
            }
            if state(value).is_some_and(|state| MISSING_STATES.contains(&state))
                && let Some(source) = source
            {
                let reason = map
                    .get("reason")
                    .and_then(|value| serde_json::from_value(value.clone()).ok());
                marks.missing.extend(entry(at, source, reason));
                return;
            }
            // A sibling with any other state, such as `data` or `empty`, was read.
            let read = map.values().any(|child| {
                state(child).is_some_and(|s| {
                    !disabled(child) && !MISSING_STATES.contains(&s) && s != "not-applicable"
                })
            });
            for (key, child) in map {
                walk(child, format!("{at}/{}", escape(key)), source, !read, marks);
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                walk(child, format!("{at}/{index}"), source, false, marks);
            }
        }
        _ => {}
    }
}

/// RFC 6901 escaping for one pointer token.
fn escape(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}
