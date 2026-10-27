//! Recorded source payloads reach released clients through explicit local endpoints.
use super::super::network_stub::RecordedReply;
use super::{Environment, Selection, execution, registry};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Route {
    pub(super) source: String,
    pub(super) endpoint_env: String,
    pub(super) host: String,
    pub(super) request_prefix: String,
    pub(super) request_contains: String,
    #[serde(default)]
    pub(super) request_query: BTreeMap<String, String>,
    #[serde(default)]
    exchange: Option<usize>,
    pub(super) capture: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) recording_gap: Option<String>,
    envelope: Option<String>,
    #[serde(default)]
    count_field: Option<String>,
}
pub(super) fn baseline(
    selection: &Selection,
    binary: &Path,
    replay: &Path,
    out: &Path,
    index: usize,
) -> Result<Value, String> {
    let registry = registry::load()?;
    let bodies = if selection.source_free {
        Vec::new()
    } else {
        bodies(&selection.recordings, replay, out, index)?
    };
    let env = Environment::new().map_err(|e| e.to_string())?;
    let mut args = selection.baseline_globals.clone();
    args.extend([
        "--disable-sources".into(),
        registry.disabled(&selection.recordings)?,
    ]);
    args.extend(selection.baseline_command.clone());
    let mut command = env.command(binary, &args);
    let peers = peers(&registry, selection, bodies, &mut command)?;
    let mut result =
        execution::invoke_command(command, &args, &out.join(format!("{index}-baseline")))?;
    execution::select_stream(
        &mut result,
        &selection.baseline_stream,
        &out.join(format!("{index}-baseline")),
    )?;
    let mut requests = Vec::new();
    for peer in peers {
        match peer.finish() {
            Ok(received) => requests.extend(received),
            Err(error) => execution::field(&mut result, "assessment_error", json!(error))?,
        }
    }
    fs::write(
        out.join(format!("{index}-baseline-requests.json")),
        serde_json::to_vec_pretty(&requests).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if requests.is_empty() && !selection.source_free && result.get("assessment_error").is_none() {
        execution::field(
            &mut result,
            "assessment_error",
            json!("baseline did not consume its declared source recording"),
        )?;
    }
    execution::field(&mut result, "recorded_requests", json!(requests.len()))?;
    execution::field(
        &mut result,
        "disabled_sources",
        json!(registry.disabled(&selection.recordings)?),
    )?;
    Ok(result)
}
fn peers(
    registry: &registry::Registry,
    selection: &Selection,
    bodies: Vec<RecordedReply>,
    command: &mut std::process::Command,
) -> Result<Vec<super::super::network_stub::Stub>, String> {
    let mut grouped = BTreeMap::new();
    for (route, body) in selection.recordings.iter().zip(bodies) {
        grouped
            .entry(route.endpoint_env.clone())
            .or_insert_with(Vec::new)
            .push((
                route.request_prefix.clone(),
                route.request_contains.clone(),
                route.request_query.clone(),
                body,
            ));
    }
    let mut peers = Vec::new();
    let endpoints = registry.active_endpoints(&selection.recordings)?;
    if endpoints
        .iter()
        .filter(|e| requires_fixture_origin(&e.variable))
        .count()
        > 1
    {
        return Err(
            "assessment gap: released fixture policy accepts only one S2 or ORCID peer origin"
                .into(),
        );
    }
    for endpoint in endpoints {
        let routes = grouped.remove(&endpoint.variable).unwrap_or_default();
        let expected_host = endpoint.host.clone();
        let peer = super::super::network_stub::Stub::loopback(
            expected_host.clone(),
            move |host, request| {
                if host != expected_host {
                    return Err("source peer ownership changed".into());
                }
                let mut matched = None;
                for (prefix, contains, query, body) in &routes {
                    if super::request::matches(prefix, contains, query, request)? {
                        if matched.is_some() {
                            return Err("ambiguous baseline source request".into());
                        }
                        matched = Some(body);
                    }
                }
                matched.cloned().ok_or_else(|| {
                    format!(
                        "unmapped baseline source request: {}",
                        request.lines().next().unwrap_or("<missing>")
                    )
                })
            },
        )?;
        command.env(&endpoint.variable, endpoint.local_url(peer.address));
        if requires_fixture_origin(&endpoint.variable) {
            // Released S2 and ORCID policies accept HTTP only for this exact fixture origin.
            command.env(
                "BIOMCP_TEST_UNPACED_ORIGIN",
                format!("http://{}", peer.address),
            );
        }
        peers.push(peer);
    }
    Ok(peers)
}
fn requires_fixture_origin(variable: &str) -> bool {
    matches!(variable, "BIOMCP_S2_BASE" | "BIOMCP_ORCID_BASE")
}
fn bodies(
    routes: &[Route],
    replay: &Path,
    out: &Path,
    index: usize,
) -> Result<Vec<RecordedReply>, String> {
    if routes.is_empty() {
        return Err("assessment gap: baseline source request recordings missing".into());
    }
    routes
        .iter()
        .enumerate()
        .map(|(ordinal, route)| {
            if let Some(reason) = &route.recording_gap {
                return Err(format!(
                    "assessment gap: {} recording: {reason}",
                    route.source
                ));
            }
            let capture: Value = serde_json::from_slice(
                &fs::read(replay.join(&route.capture)).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            let registry = registry::load()?;
            if capture.get("source").and_then(Value::as_str)
                != Some(registry.recording_source(route)?)
            {
                return Err("capture does not belong to routed source".into());
            }
            let exchanges = capture
                .get("exchanges")
                .and_then(Value::as_array)
                .ok_or("missing exchanges")?;
            if exchanges.len() != 1 && route.exchange.is_none() {
                return Err(
                    "assessment gap: source route needs an explicit recorded exchange".into(),
                );
            }
            let ordinal_exchange = route.exchange.unwrap_or(0);
            let exchange = exchanges
                .get(ordinal_exchange)
                .ok_or("selected exchange missing")?;
            let delay_ms = recorded_delay(exchange)?;
            let (adapted, body) = adapted_body(route, exchange)?;
            let status = exchange
                .get("status")
                .and_then(Value::as_u64)
                .and_then(|status| u16::try_from(status).ok())
                .ok_or("recorded HTTP status missing")?;
            let content_type = exchange
                .get("content_type")
                .and_then(Value::as_str)
                .ok_or("recorded content type missing")?
                .to_owned();
            fs::write(
                out.join(format!("{index}-source-{ordinal}.json")),
                serde_json::to_vec_pretty(&json!({
                    "capture":capture,"adapted_body":adapted,"host":route.host,
                    "request_prefix":route.request_prefix,"request_contains":route.request_contains,
                    "request_query":route.request_query,"exchange":ordinal_exchange,
                    "status":status,"content_type":content_type,
                    "delay_ms":delay_ms,
                }))
                .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            Ok(RecordedReply {
                status,
                content_type,
                body,
                delay_ms,
            })
        })
        .collect()
}

fn recorded_delay(exchange: &Value) -> Result<u64, String> {
    let delay = exchange
        .get("delay_ms")
        .and_then(Value::as_u64)
        .ok_or("recorded delay missing")?;
    if delay >= 10000 {
        return Err("assessment gap: recorded delay exceeds the direct peer bounds".into());
    }
    Ok(delay)
}

fn adapted_body(route: &Route, exchange: &Value) -> Result<(Value, Vec<u8>), String> {
    let body = exchange
        .pointer("/body/value")
        .ok_or("recorded source body missing")?;
    match exchange.pointer("/body/kind").and_then(Value::as_str) {
        Some("text") if route.envelope.is_none() && route.count_field.is_none() => {
            let text = body.as_str().ok_or("recorded text body must be a string")?;
            Ok((body.clone(), text.as_bytes().to_vec()))
        }
        Some("json") => {
            let mut adapted = if let Some(envelope) = &route.envelope {
                json!({envelope:body})
            } else {
                body.clone()
            };
            if let Some(field) = &route.count_field {
                if route.envelope.as_ref() == Some(field) || route.envelope.is_none() {
                    return Err("count field requires a distinct envelope field".into());
                }
                execution::field(
                    &mut adapted,
                    field,
                    json!(
                        body.as_array()
                            .ok_or("count field requires recorded array")?
                            .len()
                    ),
                )?;
            }
            let bytes = serde_json::to_vec(&adapted).map_err(|e| e.to_string())?;
            Ok((adapted, bytes))
        }
        _ => Err("assessment gap: unsupported recorded body or adaptation".into()),
    }
}
