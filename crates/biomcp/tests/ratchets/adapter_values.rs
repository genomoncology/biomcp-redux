//! Classify optional-value drops beside the existing source ratchet.
use super::super::{Check, rust_files};
use super::tokens;
use serde::Deserialize;
use std::{collections::BTreeMap, fs, path::Path};
#[derive(Deserialize)]
struct Audit {
    sites: Vec<Site>,
}
#[derive(Deserialize)]
struct Site {
    path: String,
    site: String,
    maximum: usize,
    class: String,
    owner: String,
    reason: String,
}
pub(super) fn check(root: &Path) -> Check {
    let audit: Audit = serde_json::from_str(include_str!("adapter_value_audit.json"))?;
    for path in rust_files(&root.join("crates/biomcp-adapters/src"))? {
        let relative = path
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        if path.file_name().is_none_or(|name| name != "map.rs")
            && !path.components().any(|part| part.as_os_str() == "map")
            && !relative.contains("/pubmed/")
            && !relative.contains("/orcid/")
        {
            continue;
        }
        for (site, count) in fingerprints(&tokens(&fs::read_to_string(&path)?)) {
            let allowed = audit
                .sites
                .iter()
                .find(|entry| entry.path == relative && entry.site == site);
            if !allowed.is_some_and(|entry| {
                count <= entry.maximum
                    && !entry.reason.is_empty()
                    && !entry.owner.is_empty()
                    && matches!(
                        entry.class.as_str(),
                        "migrated" | "identity_refusal" | "legitimate_absence" | "pending_audit"
                    )
            }) {
                return Err(format!("{relative}: classify new optional-value drop: {site}").into());
            }
        }
    }
    Ok(())
}
fn fingerprints(tokens: &[String]) -> BTreeMap<String, usize> {
    let mut result = BTreeMap::new();
    let mut function = "module";
    for (index, token) in tokens.iter().enumerate() {
        if token == "fn"
            && let Some(name) = tokens.get(index + 1)
        {
            function = name;
        }
        if tokens
            .get(index..index + 4)
            .is_some_and(|parts| parts == [".", "ok", "(", ")"])
        {
            let context = tokens[index.saturating_sub(12)..index + 4].join(" ");
            *result.entry(format!("{function}: {context}")).or_default() += 1;
        }
    }
    result
}
#[test]
fn new_optional_drop_requires_its_own_classification() {
    let source = "fn map() { let title = input.parse().ok(); }";
    let before = fingerprints(&tokens(source));
    let after = fingerprints(&tokens(
        "fn map() { let title = input.parse().ok(); let name = input.parse().ok(); }",
    ));
    assert!(
        after
            .iter()
            .any(|(site, count)| before.get(site).is_none_or(|old| count > old))
    );
    assert_eq!(
        before,
        fingerprints(&tokens("fn map() { let title = input.parse() . ok ( ); }"))
    );
}
