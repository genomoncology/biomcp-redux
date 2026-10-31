//! Hosted key errors name only the declared source, never caller input.
use super::Outcome;
use serde_json::Value;
use std::io;
pub(super) fn outcome(outcome: Outcome, hosted: bool) -> io::Result<Outcome> {
    if hosted {
        let source = outcome
            .value
            .get("sources")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|row| row.get("reason").and_then(Value::as_str) == Some("no-key"))
            .and_then(|row| row.get("source"))
            .and_then(|source| serde_json::from_value(source.clone()).ok());
        if let Some(source) = source {
            return super::encode(
                biomcp_types::ErrorEnvelope::hosted_key(source),
                crate::exit::Status::Usage,
            );
        }
    }
    Ok(outcome)
}
