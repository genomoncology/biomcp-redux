use super::{Check, dependencies, rust_files};
use std::{collections::BTreeSet, fs, path::Path};

const INLINE_TESTS: &[&str] = &[
    "crates/biomcp-query/src/article/rank.rs",
    "crates/biomcp-fetch/src/retry.rs",
    "crates/biomcp-fetch/src/retry_tests.rs",
    "crates/biomcp-fetch/src/xml.rs",
    "crates/biomcp-fetch/src/xml_tests.rs",
];
// New exceptions require a reviewed path and reason here.
const EXCEPTIONS: &[(&str, &str)] = &[
    (
        "crates/biomcp-query/src/fan_out.rs",
        "Join scoped source jobs before returning the command result",
    ),
    (
        "crates/biomcp-fetch/src/batch.rs",
        "Own bounded scoped callback threads and join their clocks before returning",
    ),
    (
        "crates/biomcp-fetch/src/deadline.rs",
        "Advance shared replay time without real sleeping",
    ),
    (
        "crates/biomcp-fetch/src/record.rs",
        "Reserve shared replay cursors and publish complete captures",
    ),
    (
        "crates/biomcp/src/mcp.rs",
        "Own bounded stdio input and tool threads",
    ),
    (
        "crates/biomcp-fetch/src/lib.rs",
        "Retain typed leg reports across concurrent scoped requests",
    ),
    (
        "crates/biomcp-fetch/src/cache.rs",
        "Serialize SQLite operations on the shared transport connection",
    ),
    (
        "crates/biomcp/src/config.rs",
        "Read process arguments at the command boundary",
    ),
    (
        "crates/biomcp-fetch/src/quota.rs",
        "Shared transport quota state and FIFO waits",
    ),
];
const FILE_LIMIT: usize = 600;
const ADAPTER_LIMIT: usize = 300;

pub(super) fn check(root: &Path) -> Check {
    let sqlite = dependencies::sqlite_aliases(root)?;
    let mut keys = BTreeSet::new();
    let crates = root.join("crates");
    if !crates.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(crates)? {
        for path in rust_files(&entry?.path().join("src"))? {
            let relative = path
                .strip_prefix(root)?
                .to_string_lossy()
                .replace('\\', "/");
            let text = fs::read_to_string(&path)?;
            let limit = if relative.starts_with("crates/biomcp-adapters/src/")
                && matches!(
                    path.file_name().and_then(|v| v.to_str()),
                    Some("mod.rs" | "raw.rs" | "map.rs")
                ) {
                ADAPTER_LIMIT
            } else {
                FILE_LIMIT
            };
            if text.lines().count() > limit {
                return fail(&relative, "file line limit exceeded");
            }
            let tokens = tokens(&text);
            attributes(&relative, &tokens)?;
            references(root, &relative, &text, &tokens, &sqlite, &mut keys)?;
        }
    }
    Ok(())
}

fn fail(path: &str, rule: &str) -> Check {
    Err(format!("{path}: {rule}").into())
}

fn attributes(path: &str, tokens: &[String]) -> Check {
    for (i, token) in tokens.iter().enumerate() {
        if token != "#" {
            continue;
        }
        let tail = tokens.get(i + 1..).unwrap_or_default();
        let tail = tail.strip_prefix(&["!".to_owned()]).unwrap_or(tail);
        if tail.first().is_none_or(|v| v != "[") {
            continue;
        }
        let attribute: Vec<_> = tail
            .iter()
            .take_while(|v| v.as_str() != "]")
            .map(String::as_str)
            .collect();
        let suppression = attribute.iter().any(|v| matches!(*v, "allow" | "expect"));
        if suppression
            && attribute
                .iter()
                .any(|v| matches!(*v, "dead_code" | "unused_imports"))
        {
            return fail(path, "dead_code and unused_imports suppressions forbidden");
        }
        if suppression
            && (!EXCEPTIONS.iter().any(|(file, _)| *file == path) || !attribute.contains(&"reason"))
        {
            return fail(path, "lint exception requires listed location and reason");
        }
        if attribute.contains(&"test") && !INLINE_TESTS.contains(&path) {
            return fail(path, "inline test outside listed locations");
        }
        if path.starts_with("crates/biomcp-adapters/")
            && path.ends_with("/raw.rs")
            && attribute.contains(&"serde")
            && attribute.contains(&"default")
        {
            return fail(path, "raw serde default forbidden");
        }
    }
    Ok(())
}

fn references(
    root: &Path,
    path: &str,
    text: &str,
    tokens: &[String],
    sqlite: &[String],
    keys: &mut BTreeSet<String>,
) -> Check {
    if path.starts_with("crates/biomcp-query/src/")
        && !path.starts_with("crates/biomcp-query/src/store/")
        && tokens.iter().any(|v| sqlite.contains(v))
    {
        return fail(path, "SQLite references belong in src/store/");
    }
    for line in text.lines() {
        if let Some((_, reference)) = line.split_once("// Quirk:") {
            let case = reference
                .split_whitespace()
                .find(|part| part.ends_with(".yaml"));
            if case.is_none_or(|case| {
                !Path::new(case)
                    .components()
                    .all(|part| matches!(part, std::path::Component::Normal(_)))
                    || !root.join("tests/cases").join(case).is_file()
            }) {
                return fail(path, "Quirk requires an existing case path");
            }
        }
    }
    let mut source_keys = vec!["SourceKey".to_owned()];
    let mut quotas = vec!["Quota".to_owned()];
    for alias in tokens.windows(3) {
        if alias.get(1).is_some_and(|v| v == "as") {
            if alias.first().is_some_and(|v| source_keys.contains(v)) {
                source_keys.push(alias.get(2).cloned().unwrap_or_default());
            }
            if alias.first().is_some_and(|v| quotas.contains(v)) {
                quotas.push(alias.get(2).cloned().unwrap_or_default());
            }
        }
    }
    for (i, token) in tokens.iter().enumerate() {
        let tail = tokens.get(i + 1..).unwrap_or_default();
        let declaration = source_keys.contains(token)
            && tail.starts_with(&[":".into(), ":".into(), "declare".into(), "(".into()]);
        let quota = token == "static"
            && i.checked_sub(1)
                .and_then(|i| tokens.get(i))
                .is_none_or(|v| v != "\'")
            && tail
                .iter()
                .take_while(|v| v.as_str() != "=")
                .any(|v| quotas.contains(v));
        if (declaration || quota) && path != "crates/biomcp-adapters/src/quotas.rs" {
            return fail(path, "quota declarations belong in adapter quotas.rs");
        }
        if declaration {
            let Some(key) = tail.get(4).filter(|v| v.starts_with('"')) else {
                return fail(path, "source key declaration requires a literal key");
            };
            if !keys.insert(key.clone()) {
                return fail(path, "duplicate source key declaration");
            }
        }
    }
    Ok(())
}

// Only identifiers, literals, comments and punctuation are needed; no Rust AST.
fn tokens(text: &str) -> Vec<String> {
    let mut input = text.chars().peekable();
    let mut result = Vec::new();
    while let Some(ch) = input.next() {
        if ch.is_whitespace() {
            continue;
        }
        if ch == '/' && input.peek() == Some(&'/') {
            for ch in input.by_ref() {
                if ch == '\n' {
                    break;
                }
            }
        } else if ch == '/' && input.peek() == Some(&'*') {
            input.next();
            let mut depth = 1;
            while let Some(ch) = input.next() {
                if ch == '/' && input.peek() == Some(&'*') {
                    input.next();
                    depth += 1;
                }
                if ch == '*' && input.peek() == Some(&'/') {
                    input.next();
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
        } else if ch == '"' {
            let mut value = String::from(ch);
            while let Some(ch) = input.next() {
                value.push(ch);
                if ch == '\\' {
                    if let Some(ch) = input.next() {
                        value.push(ch);
                    }
                } else if ch == '"' {
                    break;
                }
            }
            result.push(value);
        } else if ch.is_alphanumeric() || ch == '_' {
            let mut value = String::from(ch);
            while input
                .peek()
                .is_some_and(|ch| ch.is_alphanumeric() || *ch == '_')
            {
                if let Some(ch) = input.next() {
                    value.push(ch);
                }
            }
            result.push(value);
        } else {
            result.push(ch.to_string());
        }
    }
    result
}
