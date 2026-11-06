//! Feature article-search: partial snapshots page independently and never answer a new query.
use super::{
    configuration::{cached, invoke},
    launch::Environment,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        super::transcript::trial("article-search/whole-pool-publication/search", || {
            partial(
                "article/fold/failed-pubmed",
                &[
                    "search",
                    "article",
                    "AKT1",
                    "--author",
                    "Carpten",
                    "--source",
                    "pubmed,europepmc",
                ],
                "",
            )
            .map_err(Into::into)
        }),
        super::transcript::trial(
            "article-search/whole-pool-publication/recommendations",
            || {
                partial(
                    "article/recommendations/single",
                    &["article", "recommendations", "34096690", "--limit", "5"],
                    "doi",
                )
                .map_err(Into::into)
            },
        ),
        super::transcript::trial("article-search/whole-pool-publication/edges", || {
            partial(
                "article/edges/live",
                &[
                    "article",
                    "references",
                    "34096690",
                    "--source",
                    "semantic-scholar",
                    "--offset",
                    "16",
                    "--limit",
                    "1",
                ],
                "doi",
            )
            .map_err(Into::into)
        }),
        super::transcript::trial("article-search/whole-pool-publication/disabled", || {
            disabled().map_err(Into::into)
        }),
    ]
}

fn partial(recording: &str, command: &[&str], damage: &str) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = env.root().join("replay");
    super::response_cases::copy_capture(recording, &replay, |source, record| {
        if !damage.is_empty()
            && source == "pubmed"
            && record["url"]
                .as_str()
                .is_some_and(|url| url.contains("%5Bdoi%5D"))
        {
            let mut failure = record["exchanges"][0].clone();
            failure["status"] = json!(500);
            record["exchanges"] = json!([failure.clone(), failure.clone(), failure]);
        }
    })
    .map_err(|e| e.to_string())?;
    let cache = env.root().join("article-case-cache");
    let first = invoke(&env, cached(command, &cache, Some(&replay)), 0)?;
    if first["status"] != "partial" {
        return Err(format!("partial fixture did not remain useful: {first}"));
    }
    let id = first
        .pointer("/page/pool/id")
        .and_then(Value::as_str)
        .ok_or("partial pool absent")?;
    let mut explicit = cached(command, &cache, Some(&replay));
    explicit.extend(["--pool".into(), id.into()]);
    let page = invoke(&env, explicit, 0)?;
    if page.pointer("/page/pool/id") != Some(&json!(id)) || page["result"] != first["result"] {
        return Err("explicit partial pool lost stable rows".into());
    }
    // Recovery must be observed by a new search, while the explicit partial pool stays fixed.
    super::response_cases::copy_capture(recording, &replay, |_, record| {
        if damage.is_empty() {
            for exchange in record["exchanges"].as_array_mut().into_iter().flatten() {
                exchange["status"] = json!(200);
            }
        }
    })
    .map_err(|e| e.to_string())?;
    let second = invoke(&env, cached(command, &cache, Some(&replay)), 0)?;
    if second["status"] != "complete" {
        return Err(format!(
            "new query did not observe source recovery: {second}"
        ));
    }
    if second.pointer("/page/pool/id") == Some(&json!(id)) {
        return Err(format!("new query reused partial pool for {recording}"));
    }
    Ok(())
}

fn disabled() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cache = env.root().join("article-case-cache");
    let replay = env.root().join("replay");
    super::response_cases::copy_capture("article/fold/failed-pubmed", &replay, |_, record| {
        for exchange in record["exchanges"].as_array_mut().into_iter().flatten() {
            exchange["status"] = json!(200);
        }
    })
    .map_err(|e| e.to_string())?;
    let command = [
        "search",
        "article",
        "AKT1",
        "--author",
        "Carpten",
        "--source",
        "pubmed,europepmc",
    ];
    let mut enabled = cached(&command, &cache, Some(&replay));
    enabled.extend(["--disable-sources".into(), "pubmed".into()]);
    let first = invoke(&env, enabled, 0)?;
    let id = first
        .pointer("/page/pool/id")
        .and_then(Value::as_str)
        .ok_or("complete pool absent")?;
    let mut changed = cached(&command, &cache, Some(&replay));
    changed.extend(["--disable-sources".into(), "europepmc".into()]);
    let second = invoke(&env, changed.clone(), 0)?;
    if second["status"] != "complete"
        || second["sources"][0]["source"] != "pubmed"
        || second["sources"][0]["state"] != "ok"
    {
        return Err(format!(
            "new source policy did not reach enabled source: {second}"
        ));
    }
    changed.extend(["--pool".into(), id.into()]);
    let explicit = invoke(&env, changed, 4)?;
    if explicit["status"] != "failed"
        || !explicit["sources"].as_array().is_some_and(|sources| {
            sources
                .iter()
                .any(|source| source["source"] == "europepmc" && source["reason"] == "disabled")
        })
    {
        return Err("explicit pool did not retain failure branch".into());
    }
    Ok(())
}
