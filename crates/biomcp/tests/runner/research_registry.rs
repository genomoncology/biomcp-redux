//! Schema contract for the production research projection paths.
#[path = "../../src/mcp/research/records/paths.rs"]
mod paths;
use biomcp_adapters::registry::records::declaration;
use biomcp_types::Source;
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    let fake = json!({"type":"object","properties":{"unrelated":{"type":"string"}}});
    if projectable(&fake, &fake, false, false).is_ok() {
        return Err("a schema without research title or URL paths passed".into());
    }
    let mut gaps = Vec::new();
    for (id, text) in super::super::schema::ARTIFACTS {
        let root: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let Some(result) = root.pointer("/properties/result") else {
            continue;
        };
        let Some(name) = reference_name(result) else {
            continue;
        };
        if !name.ends_with("Page") && !name.ends_with("Card") && !name.ends_with("Cards") {
            continue;
        }
        // Pair-level guideline links do not identify a fetchable CPIC pair record.
        // CellLineIdentityPage is section metadata, not a research search result.
        if matches!(name, "PgxPage" | "CellLineIdentityPage") {
            continue;
        }
        if name == "PathwayCard" {
            reject_broken_identity(&root, id, name)?;
        }
        let projected = rows(&root, result, name);
        if projected.is_empty() {
            gaps.push(format!("{id} {name}: missing record path"));
        }
        for row in projected {
            // Unresolved or absent source identities have no selected card to fetch.
            if matches!(
                row.pointer("/properties/identity_status/const")
                    .and_then(Value::as_str),
                Some("missing_card" | "conflict")
            ) {
                continue;
            }
            let synthetic_url = matches!(
                name,
                "GenePage"
                    | "GeneCards"
                    | "ArticlePage"
                    | "ArticleEdgePage"
                    | "ArticleCards"
                    | "VariantPage"
                    | "VariantCard"
            );
            if let Err(why) = projectable(
                &root,
                row,
                synthetic_url,
                matches!(name, "VariantPage" | "VariantCard"),
            ) && !declared_fallback(&root, row, id, name)
            {
                gaps.push(format!("{id} {name}: {why}"));
            }
        }
    }
    if gaps.is_empty() {
        Ok(())
    } else {
        Err(gaps.join("\n"))
    }
}
fn reference_name(value: &Value) -> Option<&str> {
    value
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|s| s.rsplit('/').next())
        .or_else(|| {
            value
                .get("anyOf")?
                .as_array()?
                .iter()
                .find_map(reference_name)
        })
}
fn variants<'a>(root: &'a Value, value: &'a Value) -> Vec<&'a Value> {
    if let Some(reference) = value.get("$ref").and_then(Value::as_str) {
        return root
            .pointer(reference.trim_start_matches('#'))
            .map_or_else(Vec::new, |v| variants(root, v));
    }
    if value.get("properties").is_some() {
        return vec![value];
    }
    for key in ["oneOf", "anyOf", "allOf"] {
        if let Some(items) = value.get(key).and_then(Value::as_array) {
            return items
                .iter()
                .filter(|v| v.get("type") != Some(&json!("null")))
                .flat_map(|v| variants(root, v))
                .collect();
        }
    }
    vec![value]
}
fn at<'a>(root: &'a Value, value: &'a Value, path: &[&str]) -> Vec<&'a Value> {
    variants(root, value)
        .into_iter()
        .flat_map(|v| {
            if let Some((head, tail)) = path.split_first() {
                let next = if *head == "*" {
                    v.get("items")
                } else {
                    v.get("properties").and_then(|p| p.get(*head))
                };
                next.map_or_else(Vec::new, |next| at(root, next, tail))
            } else {
                vec![v]
            }
        })
        .collect()
}
fn rows<'a>(root: &'a Value, result: &'a Value, name: &str) -> Vec<&'a Value> {
    let path: &[&str] = match name {
        "ArticlePage" | "ArticleEdgePage" => &["items", "*", "header"],
        "ArticleCards" => &["items", "*", "answer", "card"],
        "GeneCards" => &["items", "*", "answer", "gene"],
        "AuthorCards" => &["items", "*", "card"],
        "TrialCards" | "NciTrialCards" => &["items", "*", "card"],
        "ProteinCard" => &["items", "*", "header"],
        "VariantCard" => &["candidates", "*"],
        _ if name.ends_with("Page") => &["items", "*"],
        _ => &[],
    };
    at(root, result, path)
        .into_iter()
        .flat_map(|v| {
            let header = at(root, v, &["header"]);
            if header.is_empty() { vec![v] } else { header }
        })
        .collect()
}
fn scalar_path(root: &Value, row: &Value, path: &[&str], numeric: bool) -> bool {
    let branches = variants(root, row);
    !branches.is_empty()
        && branches.into_iter().all(|branch| {
            if let Some((head, tail)) = path.split_first() {
                branch
                    .get("properties")
                    .and_then(|p| p.get(*head))
                    .is_some_and(|next| scalar_path(root, next, tail, numeric))
            } else {
                let wanted = if numeric { "integer" } else { "string" };
                branch.get("type") == Some(&json!(wanted))
                    || branch
                        .get("type")
                        .and_then(Value::as_array)
                        .is_some_and(|a| a.contains(&json!(wanted)))
            }
        })
}
fn string_path(root: &Value, row: &Value, paths: &[&str]) -> bool {
    paths.iter().any(|path| {
        scalar_path(
            root,
            row,
            &path.trim_start_matches('/').split('/').collect::<Vec<_>>(),
            false,
        )
    })
}

fn projectable(
    root: &Value,
    row: &Value,
    synthetic_url: bool,
    variant: bool,
) -> Result<(), &'static str> {
    if !string_path(
        root,
        row,
        if variant {
            paths::VARIANT_IDS
        } else {
            paths::TITLES
        },
    ) {
        return Err("missing title path");
    }
    if !synthetic_url && !string_path(root, row, paths::URLS) {
        return Err("missing canonical URL path");
    }
    Ok(())
}

fn enum_strings<'a>(root: &'a Value, node: &'a Value) -> Vec<&'a str> {
    variants(root, node)
        .into_iter()
        .flat_map(|v| {
            if let Some(items) = v.get("items") {
                enum_strings(root, items)
            } else if let Some(value) = v.get("const").and_then(Value::as_str) {
                vec![value]
            } else {
                v.get("enum")
                    .and_then(Value::as_array)
                    .map_or_else(Vec::new, |a| a.iter().filter_map(Value::as_str).collect())
            }
        })
        .collect()
}

fn sources(root: &Value, row: &Value, name: &str) -> Vec<Source> {
    if row
        .pointer("/properties/identity_status/const")
        .and_then(Value::as_str)
        == Some("who_ivd")
    {
        return vec![Source::WhoIvd];
    }
    let fixed = match name {
        "NciTrialPage" | "NciTrialCards" => Some(Source::Nci),
        "AuthorPage" | "AuthorCards" => Some(Source::SemanticScholar),
        "StudyPage" | "StudyCard" => Some(Source::Cbioportal),
        "PgxCard" => Some(Source::Cpic),
        "CellLinePage" | "CellLineCard" => Some(Source::Cellosaurus),
        "GwasPage" | "GwasAssociationCard" => Some(Source::Gwas),
        "DrugWhoPage" | "DrugWhoCard" => Some(Source::WhoPq),
        _ => match row
            .pointer("/properties/kind/const")
            .and_then(Value::as_str)
        {
            Some("who_ivd_product") => Some(Source::WhoIvd),
            Some("clearance") => Some(Source::Fda510k),
            _ => None,
        },
    };
    if let Some(source) = fixed {
        return vec![source];
    }
    let source_paths: &[&[&str]] = if name == "PathwayCard" {
        &[&["pathway", "id", "source"]]
    } else if name == "AdverseEventPage" {
        return vec![Source::Faers, Source::Maude];
    } else {
        &[&["source"]]
    };
    if name == "PathwayCard" {
        return source_paths
            .iter()
            .flat_map(|path| at(root, row, path))
            .flat_map(|v| enum_strings(root, v))
            .filter_map(|s| s.parse().ok())
            .collect();
    }
    at(root, root, &["query", "source"])
        .into_iter()
        .flat_map(|v| enum_strings(root, v))
        .filter_map(|s| s.parse().ok())
        .collect()
}
fn declared_fallback(root: &Value, row: &Value, id: &str, name: &str) -> bool {
    let entity = id
        .strip_prefix("biomcp.")
        .and_then(|v| v.split(['_', '-']).next())
        .unwrap_or_default();
    let entity = if id.starts_with("biomcp.cell-line-") {
        "cell-line"
    } else if entity == "adverse" {
        "adverse-event"
    } else if entity == "nci" {
        "trial"
    } else {
        entity
    };
    let Some(kind) = paths::kind(entity) else {
        return false;
    };
    let sources = sources(root, row, name);
    !sources.is_empty()
        && sources.into_iter().all(|source| {
            declaration(source, kind).is_some_and(|policy| {
                // Explicit no-page declarations retain the complete row in counted metadata.
                if policy.template.is_none() {
                    return true;
                }
                let identity = paths::IDS.iter().any(|path| {
                    let parts = path.trim_start_matches('/').split('/').collect::<Vec<_>>();
                    scalar_path(root, row, &parts, false) || scalar_path(root, row, &parts, true)
                });
                identity && !policy.display_name.is_empty()
            })
        })
}

fn reject_broken_identity(root: &Value, id: &str, name: &str) -> Result<(), String> {
    let mut broken = root.clone();
    if let Some(properties) = broken
        .pointer_mut("/$defs/PathwayId/oneOf/1/properties")
        .and_then(Value::as_object_mut)
    {
        properties.remove("id");
    } else {
        return Err("pathway tagged ID fixture changed".into());
    }
    if rows(
        &broken,
        broken
            .pointer("/properties/result")
            .ok_or("broken result")?,
        name,
    )
    .iter()
    .any(|row| declared_fallback(&broken, row, id, name))
    {
        return Err("a tagged source identity without its ID passed projection coverage".into());
    }
    Ok(())
}
