//! Mechanical record identity and publisher-link projections; no lookups.
use biomcp_types::{ResearchHit, Text};
use serde_json::Value;
fn text<'a>(value: &'a Value, paths: &[&str]) -> Option<&'a str> {
    paths.iter().find_map(|path| value.pointer(path).and_then(Value::as_str).filter(|s| !s.is_empty()))
}
fn hit(args: Vec<String>, title: &str, url: &str) -> Option<ResearchHit> {
    if !url.starts_with("https://") && !url.starts_with("http://") { return None; }
    let id = shlex::try_join(args.iter().map(String::as_str)).ok()?.into_owned();
    Some(ResearchHit { id, title: Text::new(title).ok()?, url: Text::new(url).ok()? })
}
pub(super) fn search(entity: &str, row: &Value, envelope: &Value) -> Option<ResearchHit> {
    match entity {
        "article" => article(row, row.get("header").unwrap_or(row)),
        "trial" => {
            let id = text(row, &["/nct_id", "/nci_id"])?;
            hit(vec![entity.into(), id.into()], text(row, &["/title", "/brief_title"]).unwrap_or(id), text(row, &["/url"])? )
        }
        "variant" => variant(row, envelope),
        "gene" => {
            let id = text(row, &["/hgnc_id"])?;
            hit(vec![entity.into(), id.into()], text(row, &["/name", "/symbol"]).unwrap_or(id), &format!("https://www.genenames.org/data/gene-symbol-report/#!/hgnc_id/{id}"))
        }
        _ => native(entity, row, envelope),
    }
}
fn article(identity: &Value, card: &Value) -> Option<ResearchHit> {
    let key = identity.get("key");
    let selector = text(identity, &["/pmid"]).or_else(|| key.and_then(|k| text(k, &["/id"]))).or_else(|| text(card, &["/pmid", "/pmcid", "/doi"]))?;
    let namespace = key.and_then(|k| text(k, &["/namespace"]));
    let url = if text(identity, &["/pmid"]).is_some() || text(card, &["/pmid"]).is_some() || namespace == Some("pubmed") {
        format!("https://pubmed.ncbi.nlm.nih.gov/{selector}/")
    } else if namespace == Some("pmc") || selector.starts_with("PMC") {
        format!("https://pmc.ncbi.nlm.nih.gov/articles/{selector}/")
    } else if namespace == Some("doi") || selector.starts_with("10.") { format!("https://doi.org/{selector}") }
    else { return None; };
    hit(vec!["article".into(), selector.into()], text(card, &["/title"]).unwrap_or(selector), &url)
}
fn next_args(entity: &str, id: &str, envelope: &Value) -> Option<Vec<String>> {
    envelope.get("next")?.as_array()?.iter().find_map(|step| {
        let args = step.get("args")?.as_array()?.iter().map(|s| s.as_str().map(str::to_owned)).collect::<Option<Vec<_>>>()?;
        if args.first().is_some_and(|s| s == "get") && args.get(1).is_some_and(|s| s == entity) && args.iter().any(|s| s == id) {
            Some(args.into_iter().skip(1).collect())
        } else { None }
    })
}
fn variant(row: &Value, envelope: &Value) -> Option<ResearchHit> {
    let id = text(row, &["/record/id"])?;
    hit(next_args("variant", id, envelope)?, id, text(row, &["/record/url"])? )
}
fn native(entity: &str, row: &Value, envelope: &Value) -> Option<ResearchHit> {
    let id = text(row, &["/id", "/header/id", "/concept/id", "/product/id"])?;
    let url = text(row, &["/url", "/header/url", "/attribution/url", "/record/url"])?;
    hit(next_args(entity, id, envelope)?, text(row, &["/title", "/name", "/header/name", "/concept/label"]).unwrap_or(id), url)
}
pub(super) fn fetch(entity: &str, id: &str, envelope: &Value) -> Option<ResearchHit> {
    let result = envelope.get("result")?;
    let first = result.get("items").and_then(Value::as_array).and_then(|v| v.first());
    let mut found = match entity {
        "article" => { let row = first?; article(row.get("input").unwrap_or(row), row.pointer("/answer/card")?) }
        "trial" => { let row = first?.get("card")?; let native_id = text(row, &["/nct_id", "/id"])?;
            hit(vec![entity.into(), native_id.into()], text(row, &["/title"]).unwrap_or(native_id), text(row, &["/url"])? ) }
        "variant" => { let row = result.get("candidates")?.as_array()?.first()?;
            let native_id = text(row, &["/record/id"])?;
            hit(vec![entity.into(), native_id.into()], native_id, text(row, &["/record/url"])? ) }
        "gene" => { let row = first?.pointer("/answer/gene")?; search(entity, row, envelope) }
        _ => {
            let row = result.get("header").unwrap_or(result);
            let url = text(row, &["/url", "/attribution/url", "/record/url"])?;
            hit(vec![entity.into()], text(row, &["/title", "/name", "/concept/label"]).unwrap_or(id), url)
        }
    }?;
    found.id = id.into();
    Some(found)
}
