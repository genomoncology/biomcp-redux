//! Feature pathway-failures: synthetic source faults cannot become empty or cached answers.
mod absence;
mod fixtures;
mod wikipathways;
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::path::Path;
type Check = Result<(), String>;
const ROUTES: &[(&str, &[&str])] = &[
    ("/search/query", &["search", "pathway", "Synthetic"]),
    ("/data/query/", &["get", "pathway", fixtures::ID]),
    ("/participants/", &["pathway", "genes", fixtures::ID]),
    ("/containedEvents", &["pathway", "events", fixtures::ID]),
    ("/mapping/UniProt/", &["search", "pathway", "--gene", "HGNC:1097"]),
];
pub(super) fn trials() -> Vec<Trial> {
    let mut trials: Vec<_> = ROUTES.iter().map(|&(route, args)| {
        Trial::test(format!("pathway-failures/synthetic-{route}"), move || faults(route,args).map_err(Into::into))
    }).collect();
    trials.push(Trial::test("pathway-failures/synthetic-foreign-reactome-identity", || identities().map_err(Into::into)));
    trials.push(Trial::test("pathway-failures/unsupported-pairings-before-io", || pairings().map_err(Into::into)));
    trials.push(Trial::test("pathway-failures/synthetic-exact-absence-versus-foreign404", || absence::run().map_err(Into::into)));
    trials.extend(wikipathways::trials());
    trials
}
fn call(env: &Environment, root: &Path, args: &[&str], exit: i32) -> Result<Value,String> {
    let mut words=env.globals(Some(root),false);
    words.extend(args.iter().map(|s|(*s).to_owned()));
    let value=invoke(env,words,exit)?;
    if serde_json::to_vec(&value).map_err(|e|e.to_string())?.len()>4000 {return Err("source outcome exceeds size ceiling".into());}
    Ok(value)
}
fn faults(route: &str,args: &[&str]) -> Check {
    let template=Environment::new().map_err(|e|e.to_string())?;
    let root=fixtures::seed(&template)?;
    for (label,status,body,delay,reason) in [
        ("429",429,"{}",0,"rate_limited"),
        ("500",500,"{}",0,"unavailable"),
        ("timeout",200,"{}",9000,"deadline_exceeded"),
        ("invalid JSON",200,"{",0,"bad_reply"),
        ("HTML success",200,"<html>source failed</html>",0,"bad_reply"),
        ("oversized",200,"",0,"too_large"),
    ] {
        let env=Environment::new().map_err(|e|e.to_string())?;
        let huge;
        let body=if label=="oversized" {huge=" ".repeat(8*1024*1024+1);&huge} else {body};
        fixtures::replace(&root,route,body,status,delay)?;
        let value=call(&env,&root,args,4).map_err(|e|format!("{route} {label}: {e}"))?;
        failed(&value,reason).map_err(|e|format!("{route} {label}: {e}"))?;
        uncached(&env,route)?;
    }
    Ok(())
}
fn failed(value: &Value,reason: &str) -> Check {
    let source=value.get("sources").and_then(Value::as_array).and_then(|v|v.iter().find(|s|s.get("source")==Some(&json!("reactome")))).ok_or("missing failed source")?;
    if value.get("status")!=Some(&json!("failed")) || source.get("state")!=Some(&json!("failed"))
        || source.get("reason")!=Some(&json!(reason)) || source.get("terms")!=Some(&json!("CC0 1.0 database data"))
        || source.get("terms_url")!=Some(&json!("https://reactome.org/license")) {
        return Err(format!("fault became empty or lost source/terms: {value}"));
    }
    Ok(())
}
fn uncached(env: &Environment,route: &str) -> Check {
    let db=rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite")).map_err(|e|e.to_string())?;
    let count: i64=db.query_row("SELECT count(*) FROM reply WHERE source='reactome' AND instr(url,?1)>0",[route],|r|r.get(0)).map_err(|e|e.to_string())?;
    if count!=0 {return Err(format!("rejected {route} entered reply cache"));}
    Ok(())
}
fn identities() -> Check {
    let template=Environment::new().map_err(|e|e.to_string())?;
    let root=fixtures::seed(&template)?;
    for (field,foreign) in [("stId","R-HSA-123"),("speciesName","Mus musculus"),("schemaClass","Reaction")] {
        let mut card=fixtures::card();
        *card.get_mut(field).ok_or("field")?=json!(foreign);
        fixtures::replace(&root,"/data/query/",&card.to_string(),200,0)?;
        let env=Environment::new().map_err(|e|e.to_string())?;
        let value=call(&env,&root,&["get","pathway",fixtures::ID],1)?;
        if value.get("result").is_some_and(|v|!v.is_null()) || !value.get("problems").and_then(Value::as_array).is_some_and(|v|v.iter().any(|p|p.get("source")==Some(&json!("reactome"))&&p.get("field")==Some(&json!("identity")))) {
            return Err(format!("foreign {field} became a card or lost problem: {value}"));
        }
        uncached(&env,"/data/query/")?;
    }
    Ok(())
}
fn pairings() -> Check {
    let env=Environment::new().map_err(|e|e.to_string())?;
    let socket=std::net::TcpListener::bind("127.0.0.1:0").map_err(|e|e.to_string())?;
    socket.set_nonblocking(true).map_err(|e|e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",socket.local_addr().map_err(|e|e.to_string())?)})).map_err(|e|e.to_string())?;
    for words in ["get pathway R-MMU-5673001", "get pathway WP382 events", "pathway events WP382", "search pathway MAPK --source kegg", "search pathway --gene BRAF --source wikipathways", "search pathway --top-level --source wikipathways"] {
        let value=invoke(&env,words.split_whitespace().map(str::to_owned).collect(),2)?;
        if value.get("status")!=Some(&json!("usage")) || value.get("sources").is_some() {return Err(format!("unsupported pairing reached source: {value}"));}
        if serde_json::to_vec(&value).map_err(|e|e.to_string())?.len()>1000 {return Err("pairing usage exceeds ceiling".into());}
    }
    if socket.accept().is_ok() {return Err("unsupported pairing opened provider connection".into());}
    Ok(())
}
