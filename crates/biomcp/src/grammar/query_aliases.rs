//! Existing CLI aliases share one declaration with installed argument replay.
//!
//! Forms agents type are accepted without being advertised: the text of a search as
//! `-q`/`--query`, trial `-c` and `-i`, and a positional section as `--section`. The reply's
//! query names the canonical field, so nothing is accepted silently.
use serde_json::Value;

#[derive(Clone, Copy)]
pub(crate) struct Alias {
    pub(crate) field: &'static str,
    pub(crate) argument: &'static str,
    pub(crate) long: Option<&'static str>,
    pub(crate) short: Option<char>,
    /// Shown in help; the forms added for agents stay out of it.
    pub(crate) visible: bool,
}

pub(crate) fn declarations(entity: &str, schema: &Value) -> Vec<Alias> {
    let mut aliases = Vec::new();
    let id = schema
        .get("$id")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let property = |name: &str| schema.get("properties").and_then(|p| p.get(name));
    let has = |name| property(name).is_some();
    let required = |name: &str| {
        schema
            .get("required")
            .and_then(Value::as_array)
            .is_some_and(|names| names.iter().any(|known| known == name))
    };
    // A required positional stays the one spelling, so its usage line is unchanged.
    let first = |name: &str| {
        property(name).and_then(|p| p.get("x-positional")) == Some(&Value::from(0))
            && !required(name)
    };
    if id == "biomcp.article-search/3" {
        aliases.push(Alias {
            field: "text",
            argument: "query_alias",
            long: Some("query"),
            short: Some('q'),
            visible: true,
        });
    } else if id.contains("-search")
        && let Some(field) = ["text", "query"].into_iter().find(|name| first(name))
    {
        aliases.push(Alias {
            field,
            argument: "query_alias",
            long: Some("query"),
            short: Some('q'),
            visible: false,
        });
    }
    if id.starts_with("biomcp.trial-search/") {
        for (field, short) in [("condition", 'c'), ("intervention", 'i')] {
            aliases.push(Alias {
                field,
                argument: field,
                long: None,
                short: Some(short),
                visible: false,
            });
        }
    }
    if property("section")
        .is_some_and(|p| p.get("x-positional").is_some() && p.get("x-trailing-section").is_none())
    {
        aliases.push(Alias {
            field: "section",
            argument: "section_alias",
            long: Some("section"),
            short: None,
            visible: false,
        });
    }
    if entity == "gwas" && has("query") {
        aliases.push(Alias {
            field: "gene",
            argument: "gene",
            long: None,
            short: Some('g'),
            visible: true,
        });
    }
    let short_limit = matches!(entity, "cell-line" | "author" | "gwas") && has("query")
        || entity != "gwas"
            && (id.starts_with("biomcp.pathway-")
                || id.starts_with("biomcp.author-")
                || id == "biomcp.phenotype-search-query/1")
            && has("limit");
    if short_limit {
        aliases.push(Alias {
            field: "limit",
            argument: "limit",
            long: None,
            short: Some('l'),
            visible: true,
        });
    }
    aliases
}
