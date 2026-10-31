//! Keep configuration reads and command tree construction at process setup.
use super::{Check, rust_files, source::tokens};
use std::{fs, path::Path};

pub(super) fn check(root: &Path) -> Check {
    let source = root.join("crates/biomcp/src");
    for path in rust_files(&source)? {
        let relative = path
            .strip_prefix(&source)?
            .to_string_lossy()
            .replace('\\', "/");
        // These modules contain only private tests, not product entry points.
        if matches!(
            relative.as_str(),
            "grammar/reference.rs" | "config/credential_reads_tests.rs"
        ) {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        // The HTTP worker ownership test has its own explicit server fixture.
        let product = text
            .split("#[cfg(test)]\nmod tests")
            .next()
            .unwrap_or(&text);
        placement(&relative, product)?;
    }
    Ok(())
}

fn placement(path: &str, text: &str) -> Check {
    let input = tokens(text);
    for (index, name) in input.iter().enumerate() {
        if !matches!(name.as_str(), "read_file" | "root") {
            continue;
        }
        if input
            .get(index.wrapping_sub(1))
            .is_some_and(|word| word == "fn")
        {
            continue;
        }
        let call = input.get(index + 1).is_some_and(|word| word == "(");
        let import = input
            .get(index + 1)
            .is_some_and(|word| matches!(word.as_str(), ";" | "," | "}" | "as"))
            && input[..index]
                .iter()
                .rev()
                .take_while(|word| word.as_str() != ";")
                .any(|word| word == "use");
        if !(call || import) {
            continue;
        }
        let setup = matches!(path, "main.rs" | "mcp.rs" | "mcp/http.rs");
        if !setup {
            return Err(format!("{path}: {name} belongs in main or MCP setup").into());
        }
    }
    Ok(())
}

#[test]
fn reject_config_reads_and_tree_builds_in_commands() -> Check {
    for text in [
        "crate::config::file::read_file(None)?;",
        "use crate::config::file::read_file as load; load(None)?;",
        "crate::grammar::root()?;",
        "use crate::grammar::{root as build}; build()?;",
        "use file::{ConfigFile, read_file}; read_file(None)?;",
        "let mut root = root()?;",
    ] {
        assert!(placement("command/planted.rs", text).is_err(), "{text}");
    }
    placement(
        "main.rs",
        "config::file::read_file(path)?; grammar::root()?;",
    )?;
    Ok(())
}
