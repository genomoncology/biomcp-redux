//! Native Article API review never authorizes asset or publisher-origin requests.
use crate::runner::transcript::trial as transcript_trial;
// Released src/cli/article/{dispatch,assets,export,workflow}.rs route these operations.
pub(super) fn validate(args: &[String]) -> Result<(), String> {
    let batch_entity = args.get(1).map(|entity| entity.trim().to_ascii_lowercase());
    let mut words = args.iter().map(String::as_str).collect::<Vec<_>>();
    // Released batch preflight normalizes its free-form entity before dispatch.
    if words.first() == Some(&"batch")
        && let Some(entity) = batch_entity.as_deref()
        && let Some(word) = words.get_mut(1)
    {
        *word = entity;
    }
    let allowed = match words.as_slice() {
        ["search", "article", ..] => true,
        ["get", "article", _id, sections @ ..] => sections.iter().all(|s| native_section(s)),
        ["batch", "article", _ids, options @ ..] => native_batch(options),
        ["article", "batch", _ids @ ..] => true,
        [
            "article",
            "authors" | "entities" | "citations" | "references",
            _id,
            ..,
        ] => true,
        ["article", "citation-evidence", _citing, _cited] => true,
        ["article", ..] | ["get" | "batch", "article", ..] => false,
        _ => true,
    };
    if allowed {
        Ok(())
    } else {
        Err("assessment gap: unreviewed released Article operation; native API containment does not authorize assets, fulltext, downloads or exports".into())
    }
}
fn native_section(section: &str) -> bool {
    matches!(section, "annotations" | "indexing" | "tldr")
}
fn native_batch(mut options: &[&str]) -> bool {
    while let [flag, value, rest @ ..] = options {
        if !match *flag {
            "--mode" => matches!(*value, "compact" | "detail"),
            "--sections" => value.split(',').all(native_section),
            _ => false,
        } {
            return false;
        }
        options = rest;
    }
    options.is_empty()
}

pub(super) fn trial() -> libtest_mimic::Trial {
    transcript_trial("parity/native-article-operation-containment", || {
        batch_aliases()?;
        for (command, allowed) in [
            (vec!["get", "article", "123"], true),
            (vec!["get", "article", "123", "annotations", "tldr"], true),
            (
                vec![
                    "batch",
                    "article",
                    "123",
                    "--mode",
                    "detail",
                    "--sections",
                    "annotations",
                ],
                true,
            ),
            (vec!["article", "citations", "123", "--limit", "5"], true),
            (
                vec![
                    "get",
                    "article",
                    "123",
                    "assets",
                    "--asset-view",
                    "retrievable",
                ],
                false,
            ),
            (vec!["get", "article", "123", "fulltext"], false),
            (vec!["get", "article", "123", "all"], false),
            (vec!["get", "article", "123", "--pdf"], false),
            (
                vec![
                    "batch",
                    "article",
                    "123",
                    "--mode",
                    "detail",
                    "--sections",
                    "assets",
                ],
                false,
            ),
            (
                vec!["article", "supplement-search", "123", "literal"],
                false,
            ),
            (
                vec!["article", "citation-evidence", "123", "456", "--fulltext"],
                false,
            ),
        ] {
            let args = command
                .iter()
                .map(|word| (*word).to_owned())
                .collect::<Vec<_>>();
            if validate(&args).is_ok() != allowed {
                return Err(format!("Article operation containment mismatch: {command:?}").into());
            }
        }
        Ok(())
    })
}

fn batch_aliases() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for entity in ["ARTICLE", " article ", "Article"] {
        for section in ["fulltext", "FULLTEXT", " all "] {
            let args = [
                "batch",
                entity,
                "123",
                "--mode",
                "detail",
                "--sections",
                section,
                "--pdf",
            ]
            .map(str::to_owned);
            if validate(&args).is_ok() {
                return Err("normalized batch entity bypassed native API containment".into());
            }
        }
    }
    let args = ["batch", "article", "123", "--mode", "full"].map(str::to_owned);
    if validate(&args).is_ok() {
        return Err("unknown released Article batch mode accepted".into());
    }
    Ok(())
}
