//! openFDA page, absence and status checks have one owner, `biomcp-adapters/src/openfda.rs`.
use super::{Check, rust_files, source::tokens};
use std::{fs, path::Path};

const ADAPTERS: &str = "crates/biomcp-adapters/src/";
const OWNER: &str = "crates/biomcp-adapters/src/openfda.rs";

pub(super) fn check(root: &Path) -> Check {
    let mut files = Vec::new();
    for path in rust_files(&root.join(ADAPTERS))? {
        let relative = path
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        files.push((relative, fs::read_to_string(&path)?));
    }
    // An openFDA module is any top-level adapter module that declares the openFDA host.
    let modules: Vec<&str> = files
        .iter()
        .filter(|(_, text)| text.contains("\"https://api.fda.gov"))
        .filter_map(|(relative, _)| module(relative))
        .collect();
    let mut failures = Vec::new();
    for (relative, text) in &files {
        if relative == OWNER {
            continue;
        }
        let openfda = module(relative).is_some_and(|name| modules.contains(&name))
            || text.contains("openfda::")
            || modules
                .iter()
                .any(|name| text.contains(&format!("{name}::SPEC")));
        let declares = tokens(text).windows(2).any(|pair| match pair {
            [a, b] => (a == "fn" && b == "check_status") || (a == "Reply" && b == "for"),
            _ => false,
        });
        if openfda && declares {
            failures.push(format!(
                "{relative}: openFDA reply checks belong in {OWNER}; use its Envelope and Item"
            ));
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n").into())
    }
}

fn module(relative: &str) -> Option<&str> {
    let rest = relative.strip_prefix(ADAPTERS)?;
    Some(rest.split('/').next()?.trim_end_matches(".rs"))
}

/// Planted copies fail inside an openFDA module, beside one, or naming the shared module.
pub(super) fn proof() -> Check {
    use super::proof_support::{put, tree};
    let rule = "openFDA reply checks belong in";
    for (path, copy) in [
        (
            "crates/biomcp-adapters/src/adverse_event_sources/received.rs",
            "use crate::faers::SPEC;\nimpl biomcp_fetch::Reply for Response {}\n",
        ),
        (
            "crates/biomcp-adapters/src/faers/counts.rs",
            "use super::SPEC;\nfn check_status() {}\n",
        ),
        (
            "crates/biomcp-adapters/src/device_counts.rs",
            "type Page = crate::openfda::Envelope<Row>;\nimpl<T> Reply for Count<T> {}\n",
        ),
    ] {
        let tree = tree()?;
        let root = tree.path();
        put(
            root,
            "crates/biomcp-adapters/src/faers/mod.rs",
            "const BASE: &str = \"https://api.fda.gov/drug/event.json\";\n",
        )?;
        put(root, path, copy)?;
        let failure = check(root).err().ok_or("planted openFDA copy passed")?;
        let failure = failure.to_string();
        assert!(
            failure.contains(path) && failure.contains(rule),
            "{failure}"
        );
        put(root, path, "type Page = crate::openfda::Envelope<Row>;\n")?;
        put(root, OWNER, "impl<T> Reply for Envelope<T> {}\n")?;
        check(root)?;
    }
    Ok(())
}
