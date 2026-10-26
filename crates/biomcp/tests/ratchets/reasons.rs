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
            if after.starts_with(&format!("=>{binding}.reason"))
                || binding.starts_with("SourceError{") && after.starts_with("=>reason")
            {
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
