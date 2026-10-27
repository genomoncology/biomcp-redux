//! Native Article API review never authorizes asset or publisher-origin requests.
// Released src/cli/article/{dispatch,assets,export,workflow}.rs route these operations.
pub(super) fn validate(args: &[String]) -> Result<(), String> {
    let words = args.iter().map(String::as_str).collect::<Vec<_>>();
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
            "--mode" => matches!(*value, "compact" | "full"),
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
    libtest_mimic::Trial::test("parity/native-article-operation-containment", || {
        for (command, allowed) in [
            (vec!["get", "article", "123"], true),
            (vec!["get", "article", "123", "annotations", "tldr"], true),
            (
                vec![
                    "batch",
                    "article",
                    "123",
                    "--mode",
                    "full",
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
                    "full",
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
