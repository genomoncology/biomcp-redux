//! Feature diagnostic-gtr: an oversized cold catalog read offers explicit synchronization.
use super::{Check, Stub, configured, invoke, words};
use crate::runner::{configuration::cached, launch::fixtures, transcript};
use libtest_mimic::Trial;
use serde_json::{Value, json};

const GET: &[&str] = &["get", "diagnostic", "GTR000527948.7", "--source", "gtr"];
const GENES: &[&str] = &[
    "get",
    "diagnostic",
    "GTR000527948.7",
    "--source",
    "gtr",
    "genes",
];
const SEARCH: &[&str] = &["search", "diagnostic", "--source", "gtr", "--gene", "EGFR"];

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript::trial(
        "diagnostic-gtr/oversized-first-read-offers-explicit-sync",
        || oversized().map_err(Into::into),
    )]
}

fn oversized() -> Check {
    let stub = Stub::with_length(
        "ftp.ncbi.nlm.nih.gov",
        |_| "application/x-gzip",
        |request| {
            if request.starts_with("GET /pub/GTR/data/test_version.gz ") {
                Ok(String::new())
            } else {
                Err(format!("unexpected GTR request: {request}"))
            }
        },
        |_, _| 4096,
    )?;
    let (env, stub) = configured(stub, json!({"auto_sync_max_bytes":1024}))?;
    let get = invoke(&env, words(GET), 4)?;
    let search = invoke(&env, words(SEARCH), 4)?;
    let requests = stub.finish()?;
    if requests.len() != 1 {
        return Err(format!("expected one first fetch, got {requests:?}"));
    }
    for reply in [get, search] {
        let message = reply
            .pointer("/sources/0/notices/0/message")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if reply.pointer("/sources/0/state") != Some(&json!("failed"))
            || !message.contains("size limit")
            || reply.pointer("/next/0/args") != Some(&json!(["sync", "gtr"]))
        {
            return Err(format!("missing size refusal and sync command: {reply}"));
        }
    }
    let recording = fixtures().join("replies/diagnostic/gtr-egfr");
    let cache = env.root().join("case-cache");
    invoke(&env, cached(&["sync", "gtr"], &cache, Some(&recording)), 0)?;
    let genes = invoke(&env, cached(GENES, &cache, None), 0)?;
    if genes.pointer("/result/test_id") != Some(&json!("GTR000527948.7"))
        || genes.pointer("/result/items/0/gene/ncbi_gene_id") != Some(&json!("1956"))
    {
        return Err(format!(
            "explicit sync did not make the requested test's gene readable: {genes}"
        ));
    }
    Ok(())
}
