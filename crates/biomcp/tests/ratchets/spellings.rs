//! Audit owned wire vocabularies while leaving publisher values untouched.
use super::{Check, rust_files};
use std::{fs, path::Path};

// These declarations describe BioMCP choices rather than publisher vocabularies.
const OWNED: &[&str] = &[
    "Reason", "PublicationType", "SubjectQualification", "DrugOncokbMissing",
    "AdverseEventMatchKind", "ArticleMatchKind", "CorrectionKind", "DiagnosticCountUnit",
    "SourceTimestampTimezone", "OrcidWorksScope", "BundleSize", "ConfigurationNoticeCode",
    "RecurrenceGranularity", "AlphaGenomeNonfiniteKind", "EvidenceNotice", "PathwayTotalKind",
    "AlphaGenomeScorer", "MatchKind", "AuthorCountScope",
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
        if window[0] == "rename_all" && window[2] == "\"snake_case\"" {
            return Err("owned enum values require kebab-case".into());
        }
        if window[0] == "tag" && window[2] == "\"annotationkind\"" {
            return Err("PGx tag requires annotation_kind".into());
        }
    }
    for (index, token) in tokens.iter().enumerate() {
        if !OWNED.contains(&token.as_str()) {
            continue;
        }
        let Some(start) = tokens[index..].iter().position(|token| token == "{") else {
            continue;
        };
        for value in tokens[index + start..].windows(3).take_while(|w| w[0] != "}") {
            if value[0] == "=" && value[1] == ">" {
                let literal = value[2].trim_matches('"');
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
    assert!(audit(&super::source::tokens(
        "wire_enum! { Native { Value => \"source_native\" } }"
    )).is_ok());
}
