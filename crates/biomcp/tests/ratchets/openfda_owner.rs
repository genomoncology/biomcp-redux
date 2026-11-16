//! openFDA page, absence and status checks have one owner, `biomcp-adapters/src/openfda.rs`.
use super::{Check, rust_files, source::tokens};
use std::{fs, path::Path};

const ADAPTERS: &str = "crates/biomcp-adapters/src/";
const OWNER: &str = "crates/biomcp-adapters/src/openfda.rs";

pub(super) fn check(root: &Path) -> Check {
    let mut files = Vec::new();
    for path in rust_files(&root.join(ADAPTERS))? {
        let relative = path.strip_prefix(root)?.to_string_lossy().replace('\\', "/");
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
            || modules.iter().any(|name| text.contains(&format!("{name}::SPEC")));
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

#[test]
fn openfda_reply_checks_have_one_owner() -> Check {
    let rule = "openFDA reply checks belong in";
    super::proof_support::rejects(
        "crates/biomcp-adapters/src/adverse_event_sources/received.rs",
        "use crate::faers::SPEC;\nimpl biomcp_fetch::Reply for Response {}\n",
        rule,
    )?;
    super::proof_support::rejects(
        "crates/biomcp-adapters/src/faers/counts.rs",
        "use crate::openfda::Absence;\nfn check_status() {}\n",
        rule,
    )?;
    let tree = super::proof_support::tree()?;
    let root = tree.path();
    super::proof_support::put(
        root,
        "crates/biomcp-adapters/src/fda_new/mod.rs",
        "const BASE: &str = \"https://api.fda.gov/new.json\";\n",
    )?;
    let copy = "crates/biomcp-adapters/src/fda_new/raw.rs";
    super::proof_support::put(root, copy, "impl<T> Reply for Page<T> {}\n")?;
    let failure = super::check(root).err().ok_or("openFDA module copy passed")?;
    assert!(failure.to_string().contains(copy), "{failure}");
    super::proof_support::put(root, copy, "type Page = crate::openfda::Envelope<Row>;\n")?;
    super::proof_support::put(
        root,
        "crates/biomcp-adapters/src/openfda.rs",
        "impl<T> Reply for Envelope<T> {}\n",
    )?;
    if let Err(error) = super::check(root) {
        assert!(!error.to_string().contains(rule), "{error}");
    }
    Ok(())
}
