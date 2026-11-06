//! Restricted article text is filtered before raw and mapped persistence.
#![expect(
    clippy::unwrap_used,
    clippy::panic,
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "Isolated synthetic transport and SQLite witness"
)]
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache};
use biomcp_test_support::network_stub::Stub;
use biomcp_types::{ArticleSearch, ReplyData, Request, Use};
use std::{path::Path, time::Duration};

fn run(fetch: &Fetch) -> biomcp_query::Outcome {
    biomcp_query::run(
        &biomcp_query::Ctx {
            fetch,
            budget: Duration::from_secs(20),
            fresh: false,
            ncbi_email: None,
        },
        &Request::SearchArticle(
            serde_json::from_str::<ArticleSearch>(
                r#"{"text":"rights witness","source":["litsense"],"limit":3}"#,
            )
            .unwrap(),
        ),
    )
}
fn record(dir: &Path) {
    let stub = Stub::for_sources(&["www.ncbi.nlm.nih.gov", "eutils.ncbi.nlm.nih.gov", "pmc-oa-opendata.s3.amazonaws.com"],
        |request| if request.contains("list-type") { "application/xml" } else { "application/json" },
        |request| {
            if request.contains("sentences") { return Ok(r#"[{"pmid":1001,"section":"RESULTS","text":"permitted body witness"},{"pmid":1002,"section":"RESULTS","text":"noncommercial body witness"},{"pmid":1003,"section":"RESULTS","text":"unknown body witness"}]"#.into()); }
            if request.contains("esummary") { return Ok(r#"{"result":{"uids":["1001","1002","1003"],"1001":{"uid":"1001","title":"First paper","articleids":[{"idtype":"pmc","value":"PMC1001"}]},"1002":{"uid":"1002","title":"Second paper","articleids":[{"idtype":"pmc","value":"PMC1002"}]},"1003":{"uid":"1003","title":"Third paper","articleids":[{"idtype":"pmc","value":"PMC1003"}]}}}"#.into()); }
            let id = if request.contains("PMC1001") {1001} else if request.contains("PMC1002") {1002} else {1003};
            if request.contains("list-type") { return Ok(format!("<ListBucketResult><IsTruncated>false</IsTruncated><Contents><Key>PMC{id}.1/PMC{id}.1.json</Key></Contents></ListBucketResult>")); }
            let license = match id {1001=>"CC BY",1002=>"CC BY NC",_=>"unrecognized"};
            Ok(format!(r#"{{"pmcid":"PMC{id}","pmid":{id},"license_code":"{license}","xml_url":"https://pmc-oa-opendata.s3.amazonaws.com/PMC{id}.1/PMC{id}.1.xml"}}"#))
        }).unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Record { dir: dir.into() },
        proxy: Some(biomcp_fetch::Secret::new(format!(
            "http://{}",
            stub.address
        ))),
        network: biomcp_fetch::Network::new(
            None,
            None,
            "",
            biomcp_fetch::Certificates::from_pem(stub.certificate.as_bytes()).unwrap(),
        )
        .unwrap(),
        replay_cache: Some(ReplayCache::temporary().unwrap()),
        cache_owners: vec![biomcp_query::store::OWNER],
        ..FetchConfig::default()
    })
    .unwrap();
    let out = run(&fetch);
    let ReplyData::ArticlePage {
        body: Some(page), ..
    } = out.body
    else {
        panic!("{out:?}")
    };
    assert_eq!(
        page.items.len(),
        3,
        "record source outcomes: {:?}; local: {:?}",
        out.sources,
        out.local
    );
    // Record the license metadata without calling LitSense again.
    let deadline = fetch.command_deadline(Duration::from_secs(20));
    let leg = fetch.leg(&biomcp_adapters::pmc_oa::SPEC, deadline.deadline());
    for id in [1001, 1002, 1003] {
        let _ = biomcp_adapters::pmc_oa::read(&leg, format!("PMC{id}").parse().unwrap(), None);
    }
}
#[test]
fn restricted_keeps_every_paper_and_persists_only_permitted_body_text() {
    let input = tempfile::tempdir().unwrap();
    record(input.path());
    let cache = ReplayCache::temporary_with_owners(&[biomcp_query::store::OWNER]).unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: input.path().into(),
        },
        disabled_uses: [Use::PerRecord].into(),
        replay_cache: Some(cache),
        cache_owners: vec![biomcp_query::store::OWNER],
        ..FetchConfig::default()
    })
    .unwrap();
    let out = run(&fetch);
    let ReplyData::ArticlePage {
        body: Some(page), ..
    } = out.body
    else {
        panic!("{out:?}")
    };
    assert_eq!(
        page.items
            .iter()
            .filter_map(|row| row.pmid)
            .map(|id| id.to_string())
            .collect::<Vec<_>>(),
        ["1001", "1002", "1003"]
    );
    assert_eq!(
        page.items
            .iter()
            .filter(|row| row.sentence.is_some())
            .count(),
        1,
        "keep all three papers but only the permitted sentence"
    );
    let connection = fetch.cache_access().unwrap().connect().unwrap();
    let disk: String = connection
        .query_row("SELECT group_concat(item) FROM pool_item", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert!(!disk.contains("noncommercial body witness"));
    assert!(!disk.contains("unknown body witness"));
}
