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
            if copied_reason(binding, after) {
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

fn copied_reason(binding: &str, after: &str) -> bool {
    let field = format!("{binding}.reason");
    let arm = after
        .strip_prefix("=>")
        .or_else(|| after.strip_prefix(")=>"));
    if let Some(arm) = arm {
        return arm
            .split("=>")
            .next()
            .is_some_and(|body| body.contains(&field))
            || binding.starts_with("SourceError{") && arm.starts_with("reason");
    }
    after
        .strip_prefix('=')
        .and_then(|condition| condition.split_once('{'))
        .is_some_and(|(_, body)| {
            let body = body.strip_prefix("return").unwrap_or(body);
            body.starts_with(&field) || body.starts_with(&format!("Some({field})"))
        })
}
