//! Settle reply status and list the requested data that could not be read.
use biomcp_types::{Envelope, Incomplete, ResponseBody, Source, SourceState, Status};
use serde_json::Value;

/// Result nodes whose `state` marks requested data that could not be read, including source rows
/// nested in a multi-part reply. A PDF whose text was cut short (`partial`), read as gibberish,
/// damaged, or refused because its read was stopped at the memory or time limit counts (1075).
const MISSING_STATES: [&str; 15] = [
    "unavailable",
    "no-text",
    "too-large",
    "garbled",
    "partial",
    "damaged",
    "refused",
    // A label version a label history could not read (1066).
    "section-not-bounded",
    "older-format",
    "not-fetched",
    "private-use",
    "skipped",
    "failed",
    "stale",
    "syncing",
];

/// Downgrade a `complete` reply when a requested section could not be read or a source failed.
/// For a `partial` reply, list these gaps alongside any other evidence of missing data.
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
        walk(&result, "/result".into(), fallback, &mut marks);
    }
    if envelope.status == Status::Complete && (!found.is_empty() || marks.unread_section) {
        envelope.status = Status::Partial;
    }
    if envelope.status != Status::Partial {
        return;
    }
    for item in marks.missing.into_iter().chain(named) {
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

/// Keep a missing supplement letter attached to its submission after field selection.
pub(crate) fn projected_purpose(reply: &mut Value) {
    let Some(incomplete) = reply.get("incomplete").and_then(Value::as_array) else {
        return;
    };
    let moved: Vec<_> = incomplete
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            let at = item.get("at")?.as_str()?;
            let row = at.strip_suffix("/supplement_purpose")?;
            if !row.starts_with("/result/approvals/data/records/") || reply.pointer(at).is_some() {
                return None;
            }
            let submission = format!("{row}/submission_number");
            reply.pointer(&submission)?;
            Some((index, submission))
        })
        .collect();
    if let Some(incomplete) = reply.get_mut("incomplete").and_then(Value::as_array_mut) {
        for (index, at) in moved {
            if let Some(location) = incomplete
                .get_mut(index)
                .and_then(|item| item.get_mut("at"))
            {
                *location = Value::String(at);
            }
        }
    }
}

/// What the walk over a reply's result found.
#[derive(Default)]
struct Marks {
    /// Nodes marked missing.
    missing: Vec<Incomplete>,
    /// At least one requested section was skipped or unavailable.
    unread_section: bool,
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
fn walk(value: &Value, at: String, source: Option<Source>, marks: &mut Marks) {
    match value {
        Value::Object(map) => {
            let source = map
                .get("source")
                .and_then(|value| serde_json::from_value(value.clone()).ok())
                .or(source);
            if let Some(state) = map.get("state").and_then(Value::as_str)
                && MISSING_STATES.contains(&state)
                && let Some(source) = source
            {
                let reason = map
                    .get("reason")
                    .and_then(|value| serde_json::from_value(value.clone()).ok());
                // A nested source outcome carries `use` and can describe an optional read.
                marks.unread_section |=
                    matches!(state, "skipped" | "unavailable") && !map.contains_key("use");
                marks.missing.extend(entry(at, source, reason));
                return;
            }
            for (key, child) in map {
                walk(child, format!("{at}/{}", escape(key)), source, marks);
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                walk(child, format!("{at}/{index}"), source, marks);
            }
        }
        _ => {}
    }
}

/// RFC 6901 escaping for one pointer token.
fn escape(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}
