//! Summary metadata pages never request or return whole sample membership.
use super::{
    launch::Environment,
    network_stub::Stub,
    recurrence::{args, call, cbio, config},
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        "variant-recurrence/bounded-summary-list-discovery",
        || prove().map_err(Into::into),
    )]
}
fn prove() -> Result<(), String> {
    for (reply, expected) in [
        (paged as fn(&str) -> Result<String, String>, 2),
        (exhausted, 3),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let stub = Stub::for_sources(
            &[
                "www.cbioportal.org",
                "reg.genome.network",
                "storage.googleapis.com",
            ],
            |r| {
                if r.contains("hgnc_complete_set") {
                    "text/tab-separated-values"
                } else {
                    "application/json"
                }
            },
            reply,
        )?;
        config(&env, &stub)?;
        let value = call(&env, &args(), 0)?;
        if value.pointer("/status") != Some(&json!("partial"))
            || value.pointer("/result/selected_sample_count").is_some()
            || value.pointer("/result/observed_sample_count") != Some(&json!(1))
        {
            return Err(format!(
                "metadata absence erased observations or invented a count: {value}"
            ));
        }
        let requests = stub.finish()?;
        let pages = requests
            .iter()
            .filter(|(_, r)| r.contains("/sample-lists?"))
            .collect::<Vec<_>>();
        if pages.len() != expected
            || requests
                .iter()
                .any(|(_, r)| r.contains("GET /api/sample-lists/"))
        {
            return Err("summary walk exceeded bounds or fetched membership".into());
        }
        for (number, (_, request)) in pages.iter().enumerate() {
            if !request.contains(&format!("projection=SUMMARY&pageSize=50&pageNumber={number}&sortBy=sampleListId&direction=ASC")) {
                return Err(format!("metadata walk lost native bounds: {request}"));
            }
        }
    }
    Ok(())
}
fn metadata() -> Value {
    json!(
        (0..50)
            .map(|i| json!({"studyId":"msk_impact_2017","sampleListId":format!("synthetic-list-{i}")}))
            .collect::<Vec<_>>()
    )
}
fn paged(request: &str) -> Result<String, String> {
    if request.contains("/sample-lists?") {
        return Ok(if request.contains("pageNumber=0") {
            metadata()
        } else {
            json!([{"studyId":"msk_impact_2017","sampleListId":"msk_impact_2017_all"}])
        }
        .to_string());
    }
    cbio(request)
}
fn exhausted(request: &str) -> Result<String, String> {
    if request.contains("/sample-lists?") {
        return Ok(metadata().to_string());
    }
    cbio(request)
}
