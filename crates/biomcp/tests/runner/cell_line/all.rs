//! Feature cell-line-all: bounded independent first pages with visible source outcomes.
use super::*;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("cell-line-all/bounded-first-pages", || native().map_err(Into::into))]
}
fn reply(request: &str) -> Result<String, String> {
    let target = request.split_whitespace().nth(1).ok_or("target")?;
    if target.starts_with("/chembl/") {
        return chembl::reply(request);
    }
    if target.starts_with("/release-info?") {
        return fixture("portable-cellosaurus-cellosaurus-release.json");
    }
    for id in ["CVCL_2119", "CVCL_0004"] {
        if canonical(target) == canonical(&format!("/cell-line/{id}?format=json&fields=ac%2Cid%2Csy%2Cox%2Cdi%2Cca%2Csx%2Cag%2Cdr%2Cvar%2Cprob%2Ccaution%2Cdt")) {
            return fixture(&format!("portable-cellosaurus-cell-{id}.json"));
        }
    }
    Err(format!("unproved all request: {target}"))
}
fn stub() -> Result<Stub, String> {
    Stub::for_sources(&["api.cellosaurus.org", "www.ebi.ac.uk"], |_| "application/json", reply)
}
fn native() -> Result<(), String> {
    let stub = stub()?;
    let env = setup(&stub)?;
    for id in ["CVCL_2119", "CVCL_0004"] {
        let value = invoke(&env, &["get", "cell-line", id, "all"], 0)?;
        for section in ["names", "identity", "xrefs", "variants", "chembl"] {
            path(&value, &format!("/result/sections/{section}/state"), json!("data"))?;
        }
    }
    let requests = stub.finish()?;
    if requests.len() != 6 || requests.iter().any(|(_,r)| r.contains("/assay.json")) {
        return Err(format!("all fanout changed: {}", requests.len()));
    }
    Ok(())
}
