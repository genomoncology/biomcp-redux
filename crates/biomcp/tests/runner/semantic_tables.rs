//! Boundary tables run the same command runner with response-only fixture edits.
use super::response_cases::{case, checked_case};
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;
const CAPTURE: &str = "article/edges/live-references";
const ARGS: &[&str] = &[
    "article",
    "references",
    "33123537",
    "--source",
    "semantic-scholar",
];
pub(super) fn check() -> Check {
    for (contexts, flag, expected) in [
        (
            None,
            None,
            json!({"/result/items/0/key/namespace":"semantic_scholar"}),
        ),
        (
            Some(json!([])),
            Some(false),
            json!({"/result/items/0/contexts/total":0,"/result/items/0/contexts/items":[],"/result/items/0/influential":false}),
        ),
        (
            Some(json!([
                "First invented context.",
                "Second invented context."
            ])),
            Some(true),
            json!({"/result/items/0/contexts/total":2,"/result/items/0/contexts/items":["First invented context."],"/result/items/0/influential":true}),
        ),
    ] {
        let value = checked_case(
            CAPTURE,
            ARGS,
            |source, record| {
                if source != "semantic-scholar" {
                    return;
                }
                let mut edge = json!({"citedPaper":{"paperId":"0123456789abcdef0123456789abcdef01234567","externalIds":{},"title":"Invented article title.","year":2020}});
                if let Some(value) = &contexts {
                    edge["contexts"] = value.clone();
                }
                if let Some(value) = flag {
                    edge["isInfluential"] = value.into();
                }
                record["exchanges"][0]["body"]["value"] = json!({"data":[edge]});
            },
            expected,
            0,
        )?;
        if contexts.is_none() && value.pointer("/result/items/0/contexts").is_some() {
            return Err("missing contexts were invented".into());
        }
        if flag.is_none() && value.pointer("/result/items/0/influential").is_some() {
            return Err("missing influential flag was invented".into());
        }
    }
    anonymous()?;
    for value in [
        json!({"data":[],"next":0}),
        json!({"data":[{"citedPaper":{"paperId":"bad","externalIds":{}}}]}),
        json!({"data":[{"citingPaper":{"paperId":"0123456789abcdef0123456789abcdef01234567","externalIds":{}}}]}),
    ] {
        case(
            CAPTURE,
            ARGS,
            |source, record| {
                if source == "semantic-scholar" {
                    record["exchanges"][0]["body"]["value"] = value.clone();
                }
            },
            json!({"/status":"failed","/sources/0/reason":"bad_reply"}),
            4,
        )?;
    }
    case(
        CAPTURE,
        ARGS,
        |source, record| {
            if source == "semantic-scholar" {
                record["exchanges"][0]["body"]["value"] = json!({"data":[]});
            }
        },
        json!({"/status":"complete","/page/total":0,"/page/truncated":false,"/result/items":[]}),
        0,
    )
}

fn anonymous() -> Check {
    let value = checked_case(
        CAPTURE,
        ARGS,
        |source, record| {
            if source != "semantic-scholar" {
                return;
            }
            let rows = [
                "First anonymous source row.",
                "Second anonymous source row.",
            ]
            .map(|title| {
                json!({
                    "citedPaper":{"paperId":null,"externalIds":null,"title":title,"year":2009},
                    "intents":["background"],"isInfluential":false,"contexts":["Invented context."]
                })
            });
            record["exchanges"][0]["body"]["value"] = json!({"data":rows});
        },
        json!({"/status":"complete","/page/total":2,"/result/items/*/header/title":["First anonymous source row.","Second anonymous source row."],"/next/*/kind":{"excludes":"disambiguate"}}),
        0,
    )?;
    for i in 0..2 {
        if value.pointer(&format!("/result/items/{i}/key")).is_some() {
            return Err("anonymous row gained an identity".into());
        }
    }
    Ok(())
}
