//! Match reviewed source selectors without depending on query order or encoding.
use std::collections::BTreeMap;

pub(super) fn matches(
    prefix: &str,
    contains: &str,
    expected: &BTreeMap<String, String>,
    request: &str,
) -> Result<bool, String> {
    let line = request.lines().next().ok_or("request line missing")?;
    let mut parts = line.split_whitespace();
    let method = parts.next().ok_or("request method missing")?;
    let target = parts.next().ok_or("request target missing")?;
    let (wanted_method, wanted_target) = prefix.split_once(' ').ok_or("route method missing")?;
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    if method != wanted_method
        || path != wanted_target.trim_end_matches('?')
        || wanted_target.ends_with('?') && !target.contains('?')
        || !request.contains(contains)
    {
        return Ok(false);
    }
    let mut actual = BTreeMap::new();
    for pair in query.split('&').filter(|pair| !pair.is_empty()) {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        let key = decode(key)?;
        if actual.insert(key.clone(), decode(value)?).is_some() && expected.contains_key(&key) {
            return Err("duplicate source query selector".into());
        }
    }
    Ok(expected
        .iter()
        .all(|(key, value)| actual.get(key) == Some(value)))
}
pub(super) fn validate_command(
    selection: &super::Selection,
    step: &super::Step,
) -> Result<(), String> {
    let current = step.run.as_ref().ok_or("direct command missing")?;
    for (released, candidate) in &selection.command_selectors {
        let expected = argument(current, candidate)?;
        if expected.is_none() || argument(&selection.baseline_command, released)? != expected {
            return Err(format!(
                "released command selector {released} must preserve harvested {candidate}"
            ));
        }
    }
    if current.iter().any(|arg| arg == "--age")
        && selection.command_selectors.get("--age").map(String::as_str) != Some("--age")
    {
        return Err("released client-side age filter requires an explicit command selector".into());
    }
    Ok(())
}
fn argument<'a>(args: &'a [String], flag: &str) -> Result<Option<&'a str>, String> {
    if !flag.starts_with("--") || flag.contains('=') {
        return Err("command selector must name a long option".into());
    }
    let mut result = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        let value = if arg == flag {
            Some(
                iter.next()
                    .ok_or("command selector value missing")?
                    .as_str(),
            )
        } else {
            arg.strip_prefix(&format!("{flag}="))
        };
        if let Some(value) = value
            && result.replace(value).is_some()
        {
            return Err("duplicate command selector".into());
        }
    }
    Ok(result)
}
fn decode(encoded: &str) -> Result<String, String> {
    let mut bytes = encoded.bytes();
    let mut decoded = Vec::new();
    while let Some(byte) = bytes.next() {
        decoded.push(match byte {
            b'+' => b' ',
            b'%' => {
                let high = bytes
                    .next()
                    .and_then(hex)
                    .ok_or("invalid source query escape")?;
                let low = bytes
                    .next()
                    .and_then(hex)
                    .ok_or("invalid source query escape")?;
                high * 16 + low
            }
            other => other,
        });
    }
    String::from_utf8(decoded).map_err(|e| e.to_string())
}
fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
