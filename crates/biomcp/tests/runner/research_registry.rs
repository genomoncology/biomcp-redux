//! Schema contract for the production research projection paths.
#[path = "../../src/mcp/research/records/paths.rs"]
mod paths;
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
        let projected = rows(&root, result, name);
        if projected.is_empty() {
            gaps.push(format!("{id} {name}: missing record path"));
        }
        for row in projected {
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
            ) {
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
fn string_path(root: &Value, row: &Value, paths: &[&str]) -> bool {
    paths.iter().any(|path| {
        at(
            root,
            row,
            &path.trim_start_matches('/').split('/').collect::<Vec<_>>(),
        )
        .iter()
        .any(|v| {
            v.get("type") == Some(&json!("string"))
                || v.get("type")
                    .and_then(Value::as_array)
                    .is_some_and(|a| a.contains(&json!("string")))
        })
    })
}
fn projectable(root: &Value, row: &Value, synthetic_url: bool) -> Result<(), &'static str> {
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
