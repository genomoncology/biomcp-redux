//! Bad independently identified rows cannot discard valid siblings.
use super::{Check, Value, checked_case, json};

pub(super) fn check() -> Check {
    for (capture, args, source, rows, identity, field) in [
        (
            "article/litsense/licensed-subset",
            vec![
                "search",
                "article",
                "xqzvw zzyzxq kinase",
                "--source",
                "litsense",
                "--disable-sources",
                "pubmed",
            ],
            "litsense",
            "/exchanges/0/body/value",
            "/pmid",
            "pmid",
        ),
        (
            "article/edges/live",
            vec![
                "article",
                "references",
                "33123537",
                "--source",
                "semantic-scholar",
                "--disable-sources",
                "pubmed",
            ],
            "semantic-scholar",
            "/exchanges/0/body/value/data",
            "/citedPaper/paperId",
            "paper_id",
        ),
        (
            "author/synthetic",
            vec!["search", "author", "Louis Williams"],
            "semantic-scholar",
            "/exchanges/0/body/value/data",
            "/authorId",
            "author_id",
        ),
        (
            "author/papers",
            vec!["author", "papers", "semantic-scholar:123", "details"],
            "semantic-scholar",
            "/exchanges/0/body/value/data",
            "/paperId",
            "paper_id",
        ),
        (
            "article/annotations/live",
            vec!["get", "article", "33123537", "citations"],
            "icite",
            "/exchanges/0/body/value/data",
            "/pmid",
            "pmid",
        ),
    ] {
        append_bad(capture, &args, source, rows, identity, field)?;
    }
    append_bad(
        "article/annotations/live",
        &["get", "article", "33123537", "annotations"],
        "pubtator",
        "/exchanges/0/body/value/PubTator3",
        "/id",
        "pmid",
    )?;
    europepmc()?;
    Ok(())
}

fn europepmc() -> Check {
    let args = [
        "search",
        "article",
        "EGFR lung cancer",
        "--source",
        "europepmc",
        "--disable-sources",
        "pubmed",
    ];
    let capture = "article/citations/synthetic-cap";
    let before = checked_case(capture, &args, |_, _| {}, json!({}), 0)?;
    let mut edits = 0;
    let after = checked_case(
        capture,
        &args,
        |name, recording| {
            if name == "europepmc"
                && let Some(id) =
                    recording.pointer_mut("/exchanges/0/body/value/resultList/result/0/id")
            {
                *id = json!({"bad":true});
                edits += 1;
            }
        },
        json!({}),
        0,
    )?;
    let sibling = before
        .pointer("/result/items/1")
        .ok_or("missing valid Europe PMC sibling")?;
    if edits != 1
        || after.pointer("/result/items/0") != Some(sibling)
        || after
            .get("problems")
            .and_then(Value::as_array)
            .is_none_or(|ps| {
                ps.iter()
                    .filter(|p| p["source"] == "europepmc" && p["field"] == "id")
                    .count()
                    != 1
            })
    {
        return Err(format!("Bad Europe PMC identity discarded siblings: {after}").into());
    }
    Ok(())
}

pub(super) fn append_bad(
    capture: &str,
    args: &[&str],
    source: &str,
    rows: &str,
    identity: &str,
    field: &str,
) -> Check {
    let before = checked_case(capture, args, |_, _| {}, json!({}), 0)?;
    let mut edits = 0;
    let after = checked_case(
        capture,
        args,
        |name, recording| {
            if name == source
                && let Some(items) = recording.pointer_mut(rows).and_then(Value::as_array_mut)
                && let Some(mut bad) = items.first().cloned()
                && let Some(value) = bad.pointer_mut(identity)
            {
                *value = json!({"bad":true});
                items.push(bad);
                edits += 1;
            }
        },
        json!({}),
        0,
    )?;
    if edits == 0 || before.get("result") != after.get("result") {
        return Err(format!("Bad {source} {field} discarded a valid sibling: {after}").into());
    }
    if after
        .get("problems")
        .and_then(Value::as_array)
        .is_none_or(|ps| {
            ps.iter()
                .filter(|p| p["source"] == source && p["field"] == field)
                .count()
                != 1
        })
    {
        return Err(format!("Bad {source} identity lacks one {field} problem: {after}").into());
    }
    Ok(())
}
