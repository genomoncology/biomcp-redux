//! Audit owned wire vocabularies while leaving publisher values untouched.
use super::{Check, rust_files};
use std::{fs, path::Path};

// These declarations describe BioMCP choices rather than publisher vocabularies.
const OWNED: &[&str] = &[
    "Reason",
    "PublicationType",
    "SubjectQualification",
    "DrugOncokbMissing",
    "AdverseEventReactionMatch",
    "ArticleMatchKind",
    "CorrectionKind",
    "DiagnosticCountUnit",
    "SourceTimestampTimezone",
    "OrcidWorksScope",
    "BundleSize",
    "ConfigurationNoticeCode",
    "RecurrenceGranularity",
    "AlphaGenomeNonfiniteKind",
    "EvidenceNotice",
    "PathwayTotalKind",
    "AlphaGenomeScorer",
    "MatchKind",
    "AuthorCountScope",
];

pub(super) fn check(root: &Path) -> Check {
    for name in ["biodata", "biomcp-types"] {
        for path in rust_files(&root.join("crates").join(name).join("src"))? {
            let text = fs::read_to_string(&path)?;
            audit(&super::source::tokens(&text))
                .map_err(|error| format!("{}: {error}", path.display()))?;
        }
    }
    Ok(())
}

fn audit(tokens: &[String]) -> Result<(), String> {
    for window in tokens.windows(3) {
        let [name, equal, value] = window else { continue };
        if name == "rename_all" && value == "\"snake_case\"" {
            return Err("owned enum values require kebab-case".into());
        }
        if name == "tag" && equal == "=" && value.starts_with('"') {
            let tag = value.trim_matches('"');
            if tag == "annotationkind"
                || !tag.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
            {
                return Err("serde tags require snake_case names, including annotation_kind".into());
            }
        }
    }
    for (index, token) in tokens.iter().enumerate() {
        if !OWNED.contains(&token.as_str()) {
            continue;
        }
        let tail = tokens.get(index + 1..).unwrap_or_default();
        let declaration = matches!(tail, [brace, ..] if brace == "{")
            || matches!(tail, [comma, _, brace, ..] if comma == "," && brace == "{");
        if !declaration {
            continue;
        }
        let Some(start) = tail.iter().position(|token| token == "{") else { continue };
        for value in tail.get(start..).unwrap_or_default().windows(3)
            .take_while(|w| w.first().is_none_or(|token| token != "}"))
        {
            let [equal, arrow, literal] = value else { continue };
            if equal == "=" && arrow == ">" {
                let literal = literal.trim_matches('"');
                if !literal.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-') {
                    return Err(format!("{token} requires kebab-case: {literal}"));
                }
            }
        }
    }
    Ok(())
}

#[test]
fn audit_rejects_owned_underscores_and_old_tags_but_allows_native_values() {
    for text in [
        "#[serde(tag = \"annotationkind\")] enum Pgx {}",
        "#[serde(rename_all = \"snake_case\")] enum Owned {}",
        "wire_enum! { Reason { NoKey => \"no_key\" } }",
    ] {
        assert!(audit(&super::source::tokens(text)).is_err(), "{text}");
    }
    assert!(
        audit(&super::source::tokens(
            "wire_enum! { Native { Value => \"source_native\" } }"
        ))
        .is_ok()
    );
}
