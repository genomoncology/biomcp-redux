//! Header-declared oversized replies exercise source admission, not replay-file limits.
use super::{Check, ROUTES, failed, fixtures};
use crate::runner::{configuration::invoke, network_stub::Stub, variant::source_environment};
use serde_json::Value;
fn length(request: &str, bytes: usize, route: &str) -> usize {
    if request.contains(route) { 8 * 1024 * 1024 + 1 } else { bytes }
}
fn search(request: &str, bytes: usize) -> usize { length(request,bytes,"/search/query") }
fn card(request: &str, bytes: usize) -> usize { length(request,bytes,"/data/query/") }
fn participants(request: &str, bytes: usize) -> usize { length(request,bytes,"/participants/") }
fn events(request: &str, bytes: usize) -> usize { length(request,bytes,"/containedEvents") }
fn mapping(request: &str, bytes: usize) -> usize { length(request,bytes,"/mapping/UniProt/") }
pub(super) fn run() -> Check {
    let lengths: [fn(&str,usize)->usize;5]=[search,card,participants,events,mapping];
    for (&(route,args),length) in ROUTES.iter().zip(lengths) {
        let stub=Stub::with_headers(&["reactome.org","storage.googleapis.com"], |_| "application/json",body,length,|_|200,|_|String::new())?;
        let env=source_environment(&stub)?;
        let value=invoke(&env,args.iter().map(|s|(*s).to_owned()).collect(),4).map_err(|e|format!("oversized {route}: {e}"))?;
        failed(&value,"too_large").map_err(|e|format!("oversized {route}: {e}"))?;
        let requests=stub.finish()?;
        if !requests.iter().any(|(_,request)|request.contains(route)) {return Err(format!("oversized case never reached {route}"));}
        let db=rusqlite::Connection::open(env.root().join(format!("cache/biomcp/cache-{}-{}.sqlite",biomcp_fetch::CACHE_NAMESPACE_VERSION,biomcp_query::store::VERSION))).map_err(|e|e.to_string())?;
        let count: i64=db.query_row("SELECT count(*) FROM reply WHERE source='reactome' AND instr(url,?1)>0",[route],|r|r.get(0)).map_err(|e|e.to_string())?;
        if count!=0 {return Err(format!("oversized {route} entered reply cache"));}
    }
    Ok(())
}
fn body(request: &str) -> Result<String,String> {
    if request.contains("hgnc") {
        let directory=crate::runner::launch::fixtures().join("replies/pathway/native/hgnc");
        for entry in std::fs::read_dir(directory).map_err(|e|e.to_string())? {
            let entry=entry.map_err(|e|e.to_string())?;
            let wire: Value=serde_json::from_slice(&std::fs::read(entry.path()).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
            if wire.get("url").and_then(Value::as_str).is_some_and(|s|s.contains("withdrawn.txt")==request.contains("withdrawn.txt")) {
                return wire.pointer("/exchanges/0/body/value").and_then(Value::as_str).map(str::to_owned).ok_or("HGNC body".into());
            }
        }
        return Err("HGNC setup body missing".into());
    }
    fixtures::body(request)
}
