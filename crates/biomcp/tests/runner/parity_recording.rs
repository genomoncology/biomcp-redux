//! Recorded source payloads reach released clients through explicit local endpoints.
use super::{Environment, Selection, execution, registry};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Route {
    pub(super) source: String,
    pub(super) endpoint_env: String,
    pub(super) host: String,
    request_prefix: String,
    request_contains: String,
    capture: String,
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
    let bodies = bodies(&selection.recordings, replay, out, index)?;
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
    if requests.is_empty() {
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
    bodies: Vec<String>,
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
                body,
            ));
    }
    let mut peers = Vec::new();
    for (variable, routes) in grouped {
        let route = selection
            .recordings
            .iter()
            .find(|route| route.endpoint_env == variable)
            .ok_or("unreviewed endpoint")?;
        let endpoint = registry.endpoint(route)?;
        let expected_host = endpoint.host.clone();
        let peer = super::super::network_stub::Stub::loopback(
            expected_host.clone(),
            move |host, request| {
                if host != expected_host {
                    return Err("source peer ownership changed".into());
                }
                let matches = routes
                    .iter()
                    .filter(|(prefix, contains, _)| {
                        request.starts_with(prefix) && request.contains(contains)
                    })
                    .collect::<Vec<_>>();
                let [(_, _, body)] = matches.as_slice() else {
                    return Err("unmapped or ambiguous baseline source request".into());
                };
                Ok(body.clone())
            },
        )?;
        command.env(
            &endpoint.variable,
            format!("http://{}{}", peer.address, endpoint.base_path),
        );
        peers.push(peer);
    }
    Ok(peers)
}
fn bodies(
    routes: &[Route],
    replay: &Path,
    out: &Path,
    index: usize,
) -> Result<Vec<String>, String> {
    if routes.is_empty() {
        return Err("assessment gap: baseline source request recordings missing".into());
    }
    routes
        .iter()
        .enumerate()
        .map(|(ordinal, route)| {
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
            let [exchange] = exchanges.as_slice() else {
                return Err("assessment gap: source route needs one recorded exchange".into());
            };
            if exchange.get("status") != Some(&json!(200)) {
                return Err(
                    "assessment gap: source route needs one successful recorded exchange".into(),
                );
            }
            let body = exchange
                .pointer("/body/value")
                .ok_or("missing JSON source body")?;
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
            fs::write(
                out.join(format!("{index}-source-{ordinal}.json")),
                serde_json::to_vec_pretty(&json!({
                    "capture":capture,"adapted_body":adapted,"host":route.host,
                    "request_prefix":route.request_prefix,"request_contains":route.request_contains,
                }))
                .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_string(&adapted).map_err(|e| e.to_string())
        })
        .collect()
}
