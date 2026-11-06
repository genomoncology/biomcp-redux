use super::run;
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache};
use biomcp_test_support::network_stub::Stub;
use biomcp_types::ReplyData;
use std::{path::Path, time::Duration};
pub(super) fn record(dir: &Path) {
    let stub = Stub::for_sources(
        &[
            "www.ncbi.nlm.nih.gov",
            "eutils.ncbi.nlm.nih.gov",
            "pmc-oa-opendata.s3.amazonaws.com",
            "www.ebi.ac.uk",
        ],
        |request| {
            if request.contains("list-type") || request.contains("efetch") {
                "application/xml"
            } else {
                "application/json"
            }
        },
        response,
    )
    .unwrap();
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
    let _ = biomcp_test_support::evidence::retain(dir).unwrap();
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
    let leg = fetch.leg(
        &biomcp_adapters::pubmed::SPEC,
        fetch.deadline(Duration::from_secs(20)),
    );
    let ids = ["1002".parse().unwrap()];
    let (articles, _) = biomcp_adapters::pubmed::fetch_articles(&leg, &ids, None).unwrap();
    assert_eq!(articles[0].article.abstract_parts.len(), 1);
    biomcp_adapters::pubmed::summaries(&leg, &ids, None).unwrap();
    let leg = fetch.leg(
        &biomcp_adapters::pubtator::SPEC,
        fetch.deadline(Duration::from_secs(20)),
    );
    biomcp_adapters::pubtator::annotations(
        &leg,
        &[
            "1001".parse().unwrap(),
            "1002".parse().unwrap(),
            "1003".parse().unwrap(),
        ],
    )
    .unwrap();
    let leg = fetch.leg(
        &biomcp_adapters::europepmc::SPEC,
        fetch.deadline(Duration::from_secs(20)),
    );
    for id in ["PPR1002", "PPR1003"] {
        let (article, _) = biomcp_adapters::europepmc::core(&leg, &id.parse().unwrap()).unwrap();
        assert_eq!(article.value.abstract_parts.len(), 1);
    }
    let _ = biomcp_test_support::evidence::retain(dir).unwrap();
    stub.finish().unwrap();
}

fn response(request: &str) -> Result<String, String> {
    if request.contains("sentences") {
        return Ok(r#"[{"pmid":1001,"section":"RESULTS","text":"permitted body witness"},{"pmid":1002,"section":"RESULTS","text":"noncommercial body witness"},{"pmid":1003,"section":"RESULTS","text":"unknown body witness"}]"#.into());
    }
    if request.contains("esummary") {
        return Ok(r#"{"result":{"uids":["1001","1002","1003"],"1001":{"uid":"1001","title":"First paper","articleids":[{"idtype":"pmc","value":"PMC1001"}]},"1002":{"uid":"1002","title":"Second paper","articleids":[{"idtype":"pmc","value":"PMC1002"}]},"1003":{"uid":"1003","title":"Third paper","articleids":[{"idtype":"pmc","value":"PMC1003"}]}}}"#.into());
    }
    if request.contains("efetch") {
        return Ok("<PubmedArticleSet><PubmedArticle><MedlineCitation><PMID>1002</PMID><Article><ArticleTitle>Second paper</ArticleTitle><Abstract><AbstractText>noncommercial abstract witness</AbstractText></Abstract></Article></MedlineCitation><PubmedData><ArticleIdList><ArticleId IdType=\"pmc\">PMC1002</ArticleId></ArticleIdList></PubmedData></PubmedArticle></PubmedArticleSet>".into());
    }
    if request.contains("biocjson") {
        return Ok(serde_json::json!({"PubTator3":[
        {"id":"1001","passages":[{"infons":{"type":"body"},"text":"permitted passage witness","annotations":[]}]},
        {"id":"1002","passages":[{"infons":{"type":"abstract"},"text":"noncommercial passage witness","annotations":[]}]},
        {"id":"1003","passages":[{"infons":{"type":"body"},"text":"unknown passage witness","annotations":[]},{"infons":{"type":"abstract"},"text":"unknown abstract passage retained","annotations":[]}]}
    ]}).to_string());
    }
    if request.contains("PPR") {
        let (id, license) = if request.contains("PPR1002") {
            ("PPR1002", "CC BY NC")
        } else {
            ("PPR1003", "unrecognized")
        };
        return Ok(serde_json::json!({"result":{"id":id,"source":"PPR","title":"Preprint identity","license":license,"abstractText":format!("{id} abstract witness")}}).to_string());
    }
    let id = if request.contains("PMC1001") {
        1001
    } else if request.contains("PMC1002") {
        1002
    } else {
        1003
    };
    if request.contains("list-type") {
        return Ok(format!(
            "<ListBucketResult><IsTruncated>false</IsTruncated><Contents><Key>PMC{id}.1/PMC{id}.1.json</Key></Contents></ListBucketResult>"
        ));
    }
    let license = match id {
        1001 => "CC BY",
        1002 => "CC BY NC",
        _ => "unrecognized",
    };
    Ok(format!(
        r#"{{"pmcid":"PMC{id}","pmid":{id},"license_code":"{license}","xml_url":"https://pmc-oa-opendata.s3.amazonaws.com/PMC{id}.1/PMC{id}.1.xml"}}"#
    ))
}
