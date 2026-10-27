//! Bind reviewed positional slots to this case's question and native request captures.
use super::{Case, Selection, Step, recording::Route};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Shape {
    #[serde(default)]
    name: String,
    command: Vec<String>,
    config: Option<Value>,
    exit: i32,
    declaration: Selection,
}
pub(super) fn resolve(
    shapes: &[Shape],
    name: &str,
    index: usize,
    step: &Step,
    case: &Case,
    root: &Path,
) -> Result<Option<Selection>, String> {
    let command = step
        .run
        .as_ref()
        .or(step.mcp.as_ref())
        .ok_or("shape command missing")?;
    let mut incomplete = None;
    for shape in shapes {
        if shape.config != case.config || shape.exit != step.exit {
            continue;
        }
        let Some(bindings) = bind(&shape.command, command)? else {
            continue;
        };
        if !step.expect.keys().all(|p| {
            shape.declaration.facts.contains_key(p)
                || shape.declaration.unprojectable_facts.contains_key(p)
        }) {
            incomplete = Some(format!(
                "{}: harvested assertions need reviewed projections",
                shape.name
            ));
            continue;
        }
        let mut selected = shape.declaration.clone();
        if !selected.pages.is_empty()
            || selected.candidate_command.is_some()
            || selected.baseline_gap.is_some()
            || selected.unsupported_command.is_some()
            || selected.stored_only
            || selected.baseline_items.is_some()
        {
            return Err("shape cannot bind pages, overrides, gaps or stored operations".into());
        }
        selected.case = name.into();
        selected.step = index;
        selected.assertion_pointers = None;
        selected.facts.retain(|p, _| step.expect.contains_key(p));
        selected
            .unprojectable_facts
            .retain(|p, _| step.expect.contains_key(p));
        for arg in &mut selected.baseline_command {
            *arg = expand(arg, &bindings)?;
        }
        super::operation::validate(&selected.baseline_command)?;
        bind_routes(&mut selected.recordings, &bindings, case, root)
            .map_err(|reason| format!("{}: {reason}", shape.name))?;
        super::registry::validate(&selected)?;
        return Ok(Some(selected));
    }
    incomplete.map_or(Ok(None), Err)
}
fn bind(
    pattern: &[String],
    command: &[String],
) -> Result<Option<BTreeMap<String, String>>, String> {
    if pattern.len() != command.len() {
        return Ok(None);
    }
    let mut bindings = BTreeMap::new();
    for (index, (token, actual)) in pattern.iter().zip(command).enumerate() {
        if let Some(name) = token.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
            if index < 2
                || pattern
                    .get(index.saturating_sub(1))
                    .is_some_and(|previous| previous.starts_with('-'))
                || name.is_empty()
                || !name.bytes().all(|b| b.is_ascii_lowercase() || b == b'_')
            {
                return Err(
                    "shape slots must name whole positional values after the command".into(),
                );
            }
            if actual.is_empty() || actual.starts_with('-') {
                return Ok(None);
            }
            if bindings.get(name).is_some_and(|value| value != actual) {
                return Ok(None);
            }
            bindings.insert(name.into(), actual.clone());
        } else if token != actual {
            return Ok(None);
        }
    }
    if bindings.is_empty() {
        return Err("shape needs a positional value slot".into());
    }
    Ok(Some(bindings))
}
fn expand(template: &str, bindings: &BTreeMap<String, String>) -> Result<String, String> {
    let mut remaining = template;
    let mut expanded = String::new();
    while let Some((prefix, rest)) = remaining.split_once('{') {
        expanded.push_str(prefix);
        let (name, tail) = rest.split_once('}').ok_or("unclosed shape slot")?;
        expanded.push_str(bindings.get(name).ok_or("unknown shape slot")?);
        remaining = tail;
    }
    if remaining.contains('}') {
        return Err("unopened shape slot".into());
    }
    expanded.push_str(remaining);
    Ok(expanded)
}
fn capture(route: &Route, replay: &Path) -> Result<String, String> {
    if !route.capture.is_empty()
        || route.recording_gap.is_some()
        || !route.request_prefix.starts_with("GET ")
    {
        return Err("shape requires native GET capture discovery in the current replay".into());
    }
    let registry = super::registry::load()?;
    let source = registry.recording_source(route)?;
    let directory = replay.join(source);
    let mut matched = None;
    for entry in fs::read_dir(&directory)
        .map_err(|_| format!("shape native recording missing for {}", route.source))?
    {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let value: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
            .map_err(|e| format!("malformed current-case capture: {e}"))?;
        if value.get("source").and_then(Value::as_str) != Some(source)
            || value.get("method").and_then(Value::as_str) != Some("GET")
        {
            continue;
        }
        let origin = format!("https://{}/", route.host);
        let Some(target) = value
            .get("url")
            .and_then(Value::as_str)
            .and_then(|url| url.strip_prefix(&origin))
        else {
            continue;
        };
        let query_len = target.split_once('?').map_or(0, |(_, query)| {
            query.split('&').filter(|pair| !pair.is_empty()).count()
        });
        if query_len != route.request_query.len() {
            continue;
        }
        if super::request::matches(
            &route.request_prefix,
            &route.request_contains,
            &route.request_query,
            &format!("GET /{target} HTTP/1.1\r\n"),
        )? {
            if matched.is_some() {
                return Err("ambiguous current-case native recording".into());
            }
            matched = Some(
                path.strip_prefix(replay)
                    .map_err(|e| e.to_string())?
                    .to_str()
                    .ok_or("capture path must be UTF-8")?
                    .to_owned(),
            );
        }
    }
    matched.map_or_else(
        || {
            Err(format!(
                "shape native recording missing for {}: {} {:?}",
                route.source, route.request_prefix, route.request_query
            ))
        },
        Ok,
    )
}

fn bind_routes(
    routes: &mut [Route],
    bindings: &BTreeMap<String, String>,
    case: &Case,
    root: &Path,
) -> Result<(), String> {
    for route in routes {
        if let Some(reason) = &route.recording_gap {
            return Err(format!(
                "held native recording: {}",
                expand(reason, bindings)?
            ));
        }
        if route.request_prefix.contains('{')
            && bindings.values().any(|value| {
                !value
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"-_.~".contains(&b))
            })
        {
            return Err("shape path value needs a reviewed native encoding".into());
        }
        route.request_prefix = expand(&route.request_prefix, &bindings)?;
        route.request_contains = expand(&route.request_contains, &bindings)?;
        for value in route.request_query.values_mut() {
            *value = expand(value, &bindings)?;
        }
        let replay = case
            .replay
            .as_ref()
            .ok_or("shape requires current-case recordings")?;
        let replay = root
            .parent()
            .ok_or("fixture parent missing")?
            .join("replies")
            .join(replay);
        route.capture = capture(route, &replay)?;
    }
    Ok(())
}
