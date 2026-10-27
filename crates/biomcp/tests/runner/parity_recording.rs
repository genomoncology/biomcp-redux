//! Reviewed request routes adapt only envelopes around unchanged source values.
use super::{Environment, Selection, execution};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{fs, path::Path};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Route {
    host: String,
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
    let bodies = bodies(&selection.recordings, replay, out, index)?;
    let hosts = selection
        .recordings
        .iter()
        .map(|r| r.host.as_str())
        .collect::<Vec<_>>();
    let routes = selection
        .recordings
        .iter()
        .map(|r| {
            (
                r.host.clone(),
                r.request_prefix.clone(),
                r.request_contains.clone(),
            )
        })
        .collect::<Vec<_>>();
    // This peer never opens a provider connection. Unknown hosts or targets fail closed.
    let server =
        super::super::network_stub::Stub::for_sources_with_host(&hosts, move |host, request| {
            let matching = routes
                .iter()
                .enumerate()
                .filter(|(_, (source_host, prefix, contains))| {
                    host == source_host && request.starts_with(prefix) && request.contains(contains)
                })
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            let [index] = matching.as_slice() else {
                return Err("unmapped baseline request".into());
            };
            bodies
                .get(*index)
                .cloned()
                .ok_or("missing recorded body".into())
        })?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let certificate = env.root().join("source.pem");
    fs::write(&certificate, &server.certificate).map_err(|e| e.to_string())?;
    let mut args = selection.baseline_globals.clone();
    args.extend(selection.baseline_command.clone());
    let mut command = env.command(binary, &args);
    let proxy = format!("http://{}", server.address);
    command
        .env("HTTPS_PROXY", &proxy)
        .env("HTTP_PROXY", &proxy)
        .env("ALL_PROXY", &proxy)
        .env("BIOMCP_CA_BUNDLE", &certificate);
    let mut result =
        execution::invoke_command(command, &args, &out.join(format!("{index}-baseline")))?;
    let requests = match server.finish() {
        Ok(requests) => requests,
        Err(error) => {
            execution::field(&mut result, "assessment_error", json!(error))?;
            return Ok(result);
        }
    };
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
    Ok(result)
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
