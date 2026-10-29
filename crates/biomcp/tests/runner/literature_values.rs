//! Feature article sources: native recordings retain records after one bad metadata value.
use super::response_cases::checked_case;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;

pub(super) fn trials() -> Vec<Trial> {
    [
        ("europepmc", preprint as fn() -> Check),
        ("litsense", litsense),
        ("pubtator", pubtator),
        ("pmc-oa", pmc),
        ("semantic-scholar", edges),
    ]
    .into_iter()
    .map(|(name, run)| {
        transcript_trial(format!("literature-values/{name}"), move || {
            run().map_err(|e| e.to_string().into())
        })
    })
    .collect()
}

fn native(
    capture: &str,
    args: &[&str],
    source: &str,
    pointer: &str,
    field: &str,
    retained: Value,
    missing: &str,
) -> Check {
    let positive = checked_case(capture, args, |_, _| {}, retained.clone(), 0)?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "native positive {source}: schema={}",
        positive["schema"]
    )?;
    let mut edited = 0;
    let value = checked_case(
        capture,
        args,
        |name, record| {
            if name == source
                && let Some(value) = record.pointer_mut(pointer)
            {
                *value = json!({"bad": true});
                edited += 1;
            }
        },
        retained,
        0,
    )?;
    if edited != 1 {
        return Err(format!("expected one edited {source} field, got {edited}").into());
    }
    if value.pointer(missing).is_some() {
        return Err(format!("bad {source} field survived at {missing}").into());
    }
    let problems = value["problems"]
        .as_array()
        .ok_or("missing field problems")?;
    if problems
        .iter()
        .filter(|p| p["source"] == source && p["field"] == field)
        .count()
        != 1
    {
        return Err(format!("expected one {source} {field} problem: {value}").into());
    }
    Ok(())
}
fn preprint() -> Check {
    native(
        "article/preprint/live",
        &["get", "article", "PPR1064174"],
        "europepmc",
        "/exchanges/0/body/value/result/firstPublicationDate",
        "date",
        json!({"/result/items/0/answer/card/europepmc_id":"PPR1064174"}),
        "/result/items/0/answer/card/date",
    )
}
fn litsense() -> Check {
    native(
        "article/litsense/licensed-subset",
        &[
            "search",
            "article",
            "xqzvw zzyzxq kinase",
            "--source",
            "litsense",
        ],
        "litsense",
        "/exchanges/0/body/value/0/section",
        "section",
        json!({"/result/items/0/pmid":"34718584"}),
        "/result/items/0/sentence/section",
    )
}
fn pubtator() -> Check {
    native(
        "article/annotations/live",
        &["get", "article", "33123537", "annotations"],
        "pubtator",
        "/exchanges/0/body/value/PubTator3/0/passages/0/annotations/0/text",
        "text",
        json!({"/result/items/0/answer/card/pmid":"33123537"}),
        "/result/items/0/sections/annotations/data/unused",
    )
}
fn pmc() -> Check {
    native(
        "article/fulltext/live-introduction",
        &["get", "article", "28944238", "fulltext"],
        "pmc-oa",
        "/exchanges/0/body/value/is_manuscript",
        "manuscript",
        json!({"/result/items/0/sections/fulltext/data/parts/0/title":"Introduction"}),
        "/result/items/0/sections/fulltext/data/manuscript",
    )
}
fn edges() -> Check {
    native(
        "article/edges/live",
        &[
            "article",
            "references",
            "33123537",
            "--source",
            "semantic-scholar",
        ],
        "semantic-scholar",
        "/exchanges/0/body/value/data/0/isInfluential",
        "influential",
        json!({"/result/items/0/pmid":"32597970"}),
        "/result/items/0/influential",
    )
}
