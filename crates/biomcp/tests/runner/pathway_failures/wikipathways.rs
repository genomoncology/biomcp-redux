//! Synthetic sync faults keep both previously accepted catalogs intact.
use super::{Check, Environment, call, fixtures};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("pathway-failures/synthetic-wikipathways-sync", || failures().map_err(Into::into)),
    Trial::test("pathway-failures/synthetic-partial-independent-legs", || partial().map_err(Into::into))]
}
fn failures() -> Check {
    let template=Environment::new().map_err(|e|e.to_string())?;
    let root=fixtures::seed(&template)?;
    for (label,status,body,delay) in [
        ("429",429,"{}",0), ("500",500,"{}",0),
        ("timeout",200,"{}",121000), ("invalid JSON",200,"{",0),
        ("HTML success",200,"<html>upstream failed</html>",0), ("oversized",200,"",0),
    ] {
        let env=Environment::new().map_err(|e|e.to_string())?;
        fixtures::replace_source(&root,"wikipathways","getPathwayInfo",&fixtures::catalog(false).to_string(),200,0)?;
        call(&env,&root,&["sync","wikipathways"],0)?;
        let prior=call(&env,&root,&["get","pathway","WP382"],0)?;
        let huge;
        let body=if label=="oversized" {huge=" ".repeat(8*1024*1024+1);&huge} else {body};
        fixtures::replace_source(&root,"wikipathways","getPathwayInfo",body,status,delay)?;
        let failed=call(&env,&root,&["sync","wikipathways"],4)?;
        if failed.get("status")!=Some(&json!("failed")) || failed.pointer("/sources/0/state")!=Some(&json!("failed"))
            || failed.pointer("/sources/0/source")!=Some(&json!("wikipathways"))
            || failed.pointer("/sources/0/terms")!=Some(&json!("CC0 1.0 waiver")) {
            return Err(format!("{label}: sync did not report its failed source receipt: {failed}"));
        }
        let retained=call(&env,&root,&["get","pathway","WP382"],0)?;
        if retained.get("result")!=prior.get("result") {return Err(format!("{label}: failed replacement changed old copy"));}
    }
    Ok(())
}
fn partial() -> Check {
    let env=Environment::new().map_err(|e|e.to_string())?;
    let root=fixtures::seed(&env)?;
    call(&env,&root,&["sync","wikipathways"],0)?;
    fixtures::replace(&root,"/search/query","{}",500,0)?;
    let value=call(&env,&root,&["search","pathway","Synthetic","--source","reactome,wikipathways"],0)?;
    let sources=value.get("sources").and_then(Value::as_array).ok_or("sources")?;
    if value.get("status")!=Some(&json!("partial"))
        || value.pointer("/result/items/0/id/source")!=Some(&json!("wikipathways"))
        || !sources.iter().any(|v|v.get("source")==Some(&json!("reactome"))&&v.get("state")==Some(&json!("failed")))
        || !sources.iter().any(|v|v.get("source")==Some(&json!("wikipathways"))&&v.get("state")==Some(&json!("ok"))) {
        return Err(format!("independent successful/failed legs lost their outcome: {value}"));
    }
    Ok(())
}
