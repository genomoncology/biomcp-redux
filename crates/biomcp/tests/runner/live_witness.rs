//! Keep bounded diagnostic context only when a case drifts or fails.
use super as live;
use super::super::launch::Environment;
use biomcp_fetch::{Observation, Shape};
use biomcp_types::Use;
use serde_json::{Value, json};
use std::path::Path;

/// Total serialized witness payload per report; summaries remain independent.
const MAX_WITNESS_BYTES: usize = 1024 * 1024;

pub(super) fn retain(results: &mut [Value]) {
    let cut =
        json!({"cut":true,"reason":"report witness byte limit", "limit_bytes":MAX_WITNESS_BYTES});
    let overhead = serde_json::to_vec(&cut).map_or(MAX_WITNESS_BYTES, |bytes| bytes.len());
    let mut remaining = MAX_WITNESS_BYTES.saturating_sub(results.len().saturating_mul(overhead));
    for result in results {
        let wanted = live::has_drift(result) || result.get("error").is_some();
        let Some(map) = result.as_object_mut() else {
            continue;
        };
        let Some(witness) = map.remove("witness") else {
            continue;
        };
        if !wanted {
            continue;
        }
        let size = serde_json::to_vec(&witness).map_or(usize::MAX, |bytes| bytes.len());
        if size <= remaining {
            remaining -= size;
            map.insert("witness".into(), witness);
        } else {
            map.insert("witness".into(), cut.clone());
        }
    }
}

type Reply = (Value, Vec<Observation>);
pub(super) fn command(
    baseline: &Environment,
    live: &Environment,
    args: (&[String], &[String]),
    replay: &Path,
    ordinal: usize,
    captures: &mut Vec<Value>,
) -> Result<(Reply, Result<Reply, String>), String> {
    captures.push(json!({"ordinal":ordinal}));
    let capture = captures.last_mut().ok_or("missing command capture")?;
    let expected = invoke(baseline, args.0, Some(replay), ordinal, capture, "baseline")?;
    if ordinal == 1 && expected.1.is_empty() {
        return Err("recorded command has no provider observations".into());
    }
    let actual = invoke(live, args.1, None, ordinal, capture, "live");
    Ok((expected, actual))
}

pub(in crate::runner) fn invoke(
    environment: &Environment,
    args: &[String],
    replay: Option<&Path>,
    ordinal: usize,
    capture: &mut Value,
    side: &str,
) -> Result<(Value, Vec<Observation>), String> {
    let result = live::invoke(environment, args, replay, ordinal);
    let context = match &result {
        Ok((output, observations)) => context(output, observations),
        Err(error) => {
            let observations = live::observations(environment, ordinal).unwrap_or_default();
            json!({"error":error,"requests":observations})
        }
    };
    if let Some(map) = capture.as_object_mut() {
        map.insert(side.into(), context);
    }
    result
}

pub(in crate::runner) fn context(output: &Value, observations: &[Observation]) -> Value {
    // Open-only retention is conservative for mixed, restricted, unknown and
    // noncommercial terms. Never retain raw provider bodies or any headers.
    let permitted = observations.iter().all(|observation| {
        biomcp_adapters::registry::terms(observation.source)
            .is_some_and(|terms| terms.usage == Use::Open)
    }) && output
        .get("sources")
        .and_then(Value::as_array)
        .is_none_or(|sources| {
            sources
                .iter()
                .all(|source| source.get("use").and_then(Value::as_str) == Some("open"))
        });
    let mut ids = Vec::new();
    identities(output, &mut ids, 0);
    json!({"requests":observations,"result_ids":ids,
        "output_shape":Shape::inspect(output.to_string().as_bytes(),"application/json"),
        "output":if permitted {output.clone()} else {json!({"omitted":"source terms require shape and identities only"})}})
}

fn identities(value: &Value, ids: &mut Vec<Value>, depth: usize) {
    if depth > 12 || ids.len() >= 64 {
        return;
    }
    match value {
        Value::Object(map) => {
            for (key, item) in map {
                if (key == "id" || key.ends_with("_id") || key == "accession")
                    && (item.is_string() || item.is_number())
                    && item.to_string().len() <= 256
                    && ids.len() < 64
                {
                    ids.push(json!({"field":key,"value":item}));
                }
                identities(item, ids, depth + 1);
            }
        }
        Value::Array(items) => {
            for item in items.iter().take(64) {
                identities(item, ids, depth + 1);
            }
        }
        _ => {}
    }
}
