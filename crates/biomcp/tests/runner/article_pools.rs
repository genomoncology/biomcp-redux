//! Feature article-search: partial snapshots page independently and never answer a new query.
use super::{
    configuration::{cached, invoke},
    launch::Environment,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![super::transcript::trial(
        "article-search/whole-pool-publication",
        || {
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
            )?;
            partial(
                "article/recommendations/single",
                &["article", "recommendations", "34096690", "--limit", "5"],
                "/recommendedPapers/0/paperId",
            )?;
            partial(
                "article/edges/live",
                &[
                    "article",
                    "references",
                    "33123537",
                    "--source",
                    "semantic-scholar",
                    "--offset",
                    "112",
                    "--limit",
                    "1",
                ],
                "/data/0/citedPaper/paperId",
            )?;
            disabled().map_err(Into::into)
        },
    )]
}

fn partial(recording: &str, command: &[&str], damage: &str) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = env.root().join("replay");
    super::response_cases::copy_capture(recording, &replay, |_, record| {
        if !damage.is_empty() {
            for exchange in record["exchanges"].as_array_mut().into_iter().flatten() {
                if let Some(id) = exchange["body"]["value"].pointer_mut(damage) {
                    *id = json!("invalid-paper-id");
                }
            }
        }
    })?;
    let cache = env.root().join("cache");
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
    let second = invoke(&env, cached(command, &cache, Some(&replay)), 0)?;
    if second.pointer("/page/pool/id") == Some(&json!(id)) {
        return Err(format!("new query reused partial pool for {recording}"));
    }
    Ok(())
}

fn disabled() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cache = env.root().join("cache");
    let replay = super::launch::fixtures().join("replies/article/fold/failed-pubmed");
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
    // The newly enabled PubMed leg fails, but must be reached instead of rejecting an old pool.
    let second = invoke(&env, changed.clone(), 2)?;
    if second["sources"][0]["source"] != "pubmed" || second["sources"][0]["state"] != "failed" {
        return Err(format!(
            "new source policy did not reach enabled source: {second}"
        ));
    }
    changed.extend(["--pool".into(), id.into()]);
    let explicit = invoke(&env, changed, 3)?;
    if explicit["status"] != "failed" {
        return Err("explicit pool did not retain failure branch".into());
    }
    Ok(())
}
