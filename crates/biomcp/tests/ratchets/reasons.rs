//! Request error reasons have one owner even when an adapter adds a new section.
use super::{Check, rust_files};
use std::{fs, path::Path};

pub(super) fn check(root: &Path) -> Check {
    let owner = root.join("crates/biomcp-query/src/status.rs");
    for path in rust_files(&root.join("crates/biomcp-query/src"))? {
        if path == owner {
            continue;
        }
        let text: String = fs::read_to_string(&path)?
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        for tail in text.split("RequestError::Source(").skip(1) {
            let Some((binding, after)) = tail.split_once(')') else {
                continue;
            };
            let arm = after
                .strip_prefix("=>")
                .or_else(|| after.strip_prefix(")=>"));
            if arm.is_some_and(|arm| {
                arm.split("=>")
                    .next()
                    .is_some_and(|body| body.contains(&format!("{binding}.reason")))
                    || binding.starts_with("SourceError{") && arm.starts_with("reason")
            }) {
                return Err(format!(
                    "{}: map request reasons through status::reason",
                    path.display()
                )
                .into());
            }
        }
    }
    Ok(())
}

#[test]
fn request_reason_mapping_cannot_be_copied() -> Check {
    let root = tempfile::tempdir()?;
    let source = root.path().join("crates/biomcp-query/src");
    fs::create_dir_all(&source)?;
    for copied in [
        "fn reason(error: RequestError) -> Reason { match error { RequestError::Source(e) => e.reason, _ => Reason::Unavailable } }",
        "match reply { Err(RequestError::Source(e)) => Section::unavailable(source, e.reason), Err(_) => Section::unavailable(source, Reason::Unavailable) }",
        "match error { RequestError::Source(SourceError { reason, .. }) => reason, _ => Reason::Unavailable }",
    ] {
        fs::write(source.join("section.rs"), copied)?;
        if check(root.path()).is_ok() {
            return Err("a copied request reason mapping passed the repository gate".into());
        }
    }
    fs::write(
        source.join("section.rs"),
        "if matches!(error, RequestError::Source(e) if e.reason == Reason::NotFound) { absent() }",
    )?;
    check(root.path())
}
