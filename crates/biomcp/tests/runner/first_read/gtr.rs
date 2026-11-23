//! Feature diagnostic-gtr: an oversized cold catalog read offers explicit synchronization.
use super::{Check, Stub, configured, invoke, words};
use crate::runner::{configuration::cached, launch::fixtures, transcript};
use libtest_mimic::Trial;
use serde_json::json;

const GET: &[&str] = &["get", "diagnostic", "GTR000527948.7", "--source", "gtr"];
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
        let message = reply["sources"][0]["notices"][0]["message"]
            .as_str()
            .unwrap_or_default();
        if reply["sources"][0]["state"] != "failed"
            || !message.contains("size limit")
            || reply["next"][0]["args"] != json!(["sync", "gtr"])
        {
            return Err(format!("missing size refusal and sync command: {reply}"));
        }
    }
    let recording = fixtures().join("replies/diagnostic/gtr-egfr");
    let cache = env.root().join("case-cache");
    invoke(&env, cached(&["sync", "gtr"], &cache, Some(&recording)), 0)?;
    let card = invoke(&env, cached(GET, &cache, None), 0)?;
    if card["result"]["header"]["id"] != "GTR000527948.7" {
        return Err(format!(
            "explicit sync did not make the requested card readable: {card}"
        ));
    }
    Ok(())
}
