use super::{Check, dependencies, rust_files};
use std::{collections::BTreeSet, fs, path::Path};

// The sole dead-code exception contains only the private generated closure include.
const GENERATED_PROTO: &str = "crates/biomcp-fetch/src/grpc/proto.rs";

// New exceptions require a reviewed path and reason here.
const EXCEPTIONS: &[(&str, &str)] = &[
    (
        "crates/biomcp-fetch/src/grpc/worker.rs",
        "Serialize lazy runtime initialization and join its sole worker thread",
    ),
    (
        GENERATED_PROTO,
        "Retain the complete pinned generated provider import closure",
    ),
    (
        "crates/biomcp-adapters/src/monarch/native_contracts.rs",
        "Locate and read retained external bodies only inside the ignored private mapper contract",
    ),
    (
        "crates/biomcp-adapters/src/registry.rs",
        "Require every source outcome to use its unique publisher declaration",
    ),
    (
        "crates/biomcp-types/src/source.rs",
        "Convert closed nonempty source spellings into neutral provenance text",
    ),
    (
        "crates/biomcp/src/mcp/http.rs",
        "Own bounded HTTP connection workers and join completed handles",
    ),
    (
        "crates/biomcp-adapters/src/pmc_oa/supplement.rs",
        "Retain one parsed binary reply across its checked cache admission callback",
    ),
    (
        "crates/biomcp-fetch/src/observe.rs",
        "Serialize bounded opt-in attempt summaries across transport workers",
    ),
    (
        "crates/biomcp-query/src/store/mod.rs",
        "Own SQLite connections and wall-clock pool timestamps",
    ),
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
const LINE_WIDTH: usize = 160;
const ADAPTER_LIMIT: usize = 300;

pub(super) fn check(root: &Path) -> Check {
    super::placement::check(root)?;
    let sqlite = dependencies::sqlite_aliases(root)?;
    let mut keys = BTreeSet::new();
    let crates = root.join("crates");
    if !crates.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(crates)? {
        let crate_root = entry?.path();
        for path in rust_files(&crate_root.join("tests"))? {
            let text = fs::read_to_string(&path)?;
            width(&path.strip_prefix(root)?.to_string_lossy(), &text)?;
            if text.lines().count() > FILE_LIMIT {
                return fail(
                    &path.strip_prefix(root)?.to_string_lossy(),
                    "file line limit exceeded",
                );
            }
        }
        for path in rust_files(&crate_root.join("src"))? {
            let relative = path
                .strip_prefix(root)?
                .to_string_lossy()
                .replace('\\', "/");
            let text = fs::read_to_string(&path)?;
            width(&relative, &text)?;
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
            source_terms(root, &relative, &tokens)?;
            if relative != "crates/biomcp-types/src/response/source.rs"
                && relative != "crates/biomcp-adapters/src/registry.rs"
                && tokens
                    .windows(3)
                    .any(|part| matches!(part, [before, name, open] if before != ">" && name == "SourceOutcome" && open == "{"))
            {
                return fail(&relative, "source outcomes require registry terms");
            }
            attributes(&relative, &tokens)?;
            references(root, &relative, &text, &tokens, &sqlite, &mut keys)?;
        }
    }
    Ok(())
}

fn width(path: &str, text: &str) -> Check {
    for (index, line) in text.lines().enumerate() {
        if line.chars().count() > LINE_WIDTH {
            return fail(
                path,
                &format!("line width limit exceeded at line {}", index + 1),
            );
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
            && (attribute.contains(&"unused_imports")
                || attribute.contains(&"dead_code") && path != GENERATED_PROTO)
        {
            return fail(path, "dead_code and unused_imports suppressions forbidden");
        }
        if suppression
            && (!EXCEPTIONS.iter().any(|(file, _)| *file == path) || !attribute.contains(&"reason"))
        {
            return fail(path, "lint exception requires listed location and reason");
        }
        if path == GENERATED_PROTO && suppression {
            let after = tail
                .iter()
                .position(|v| v == "]")
                .and_then(|i| tail.get(i + 1..));
            let only_include = ["include", "!", "(", "\"generated/mod.rs\"", ")", ";"];
            let only_dead_code = matches!(
                attribute.as_slice(),
                ["[", "allow", "(", "dead_code", ",", "reason", "=", _, ")"]
                    | [
                        "[",
                        "allow",
                        "(",
                        "dead_code",
                        ",",
                        "reason",
                        "=",
                        _,
                        ",",
                        ")"
                    ]
            );
            if !only_dead_code
                || i != 0
                || tokens.get(i + 1).is_none_or(|v| v != "!")
                || after.is_none_or(|v| v.iter().map(String::as_str).ne(only_include))
            {
                return fail(
                    path,
                    "generated exception requires only the private closure include",
                );
            }
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
pub(super) fn tokens(text: &str) -> Vec<String> {
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

fn source_terms(root: &Path, path: &str, tokens: &[String]) -> Check {
    if !path.starts_with("crates/biomcp-adapters/src/") {
        return Ok(());
    }
    for (i, word) in tokens.iter().enumerate() {
        if word != "SourceSpec" || tokens.get(i + 1).is_none_or(|s| s != "{") {
            continue;
        }
        let rest = tokens.get(i + 2..).unwrap_or_default();
        let mut depth = 1;
        let end = rest
            .iter()
            .position(|s| {
                if s == "{" {
                    depth += 1;
                } else if s == "}" {
                    depth -= 1;
                }
                depth == 0
            })
            .unwrap_or(rest.len());
        let declaration = rest.get(..end).unwrap_or_default();
        let Some(pos) = declaration.iter().position(|s| s == "terms") else {
            return fail(path, "source declaration requires publisher terms");
        };
        let terms = declaration.get(pos + 1..).unwrap_or_default();
        let terms: Vec<_> = terms.iter().take_while(|s| s.as_str() != "}").collect();
        for name in ["name", "url", "usage"] {
            let Some(pos) = terms.iter().position(|s| s.as_str() == name) else {
                return fail(path, "source declaration requires publisher terms");
            };
            if name != "usage" && terms.get(pos + 2).is_none_or(|s| s.as_str() == "\"\"") {
                return fail(path, "source declaration requires publisher terms");
            }
        }
        let relative = path
            .strip_prefix("crates/biomcp-adapters/src/")
            .unwrap_or("");
        let module = relative.strip_suffix(".rs").unwrap_or(relative);
        let module = module
            .strip_suffix("/mod")
            .unwrap_or(module)
            .replace('/', "::");
        let prefix = tokens.get(..i).ok_or("source declaration offset is invalid")?;
        let Some(position) = prefix.iter().rposition(|token| token == "static") else {
            return fail(path, "source declarations require named statics");
        };
        if prefix
            .get(position..)
            .ok_or("source static offset is invalid")?
            .iter()
            .any(|token| matches!(token.as_str(), ";" | "{" | "}"))
        {
            return fail(path, "source declarations require named statics");
        }
        let name = tokens
            .get(position + 1)
            .ok_or("source static lacks a name")?;
        let expected = format!("&crate::{module}::{name}");
        let registry = fs::read_to_string(root.join("crates/biomcp-adapters/src/registry.rs"))?;
        if !registry.contains(&expected) {
            return fail(
                path,
                &format!("implemented source must appear in source registry: expected {expected}"),
            );
        }
    }
    Ok(())
}
