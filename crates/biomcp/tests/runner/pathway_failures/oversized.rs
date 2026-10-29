//! Oversized source bodies exercise the real bounded reader, not replay-file limits.
use super::{Check, ROUTES, failed, fixtures};
use crate::runner::pathway_budgets as budgets;
use crate::runner::{network_stub::Stub, variant::source_environment};
use serde_json::Value;
pub(super) fn run() -> Check {
    for &(route, args) in ROUTES {
        let stub = Stub::with_headers(
            &["reactome.org", "storage.googleapis.com"],
            |_| "application/json",
            move |request| {
                if request.contains(route) {
                    Ok(" ".repeat(8 * 1024 * 1024 + 1) + &body(request)?)
                } else {
                    body(request)
                }
            },
            |_, bytes| bytes,
            |_| 200,
            |_| String::new(),
        )?;
        let env = source_environment(&stub)?;
        let value = budgets::invoke(
            "pathway_failures/oversized/run/1",
            &env,
            args.iter().map(|s| (*s).to_owned()).collect(),
            4,
        )
        .map_err(|e| format!("oversized {route}: {e}"))?;
        failed(&value, "bad-reply").map_err(|e| format!("oversized {route}: {e}"))?;
        let requests = stub.finish()?;
        if !requests.iter().any(|(_, request)| request.contains(route)) {
            return Err(format!("oversized case never reached {route}"));
        }
        let db = rusqlite::Connection::open(env.root().join(format!(
            "cache/biomcp/cache-{}-{}.sqlite",
            biomcp_fetch::CACHE_NAMESPACE_VERSION,
            biomcp_query::store::CACHE_NAMESPACE_VERSION
        )))
        .map_err(|e| e.to_string())?;
        let count: i64 = db
            .query_row(
                "SELECT count(*) FROM reply WHERE source='reactome' AND instr(url,?1)>0",
                [route],
                |r| r.get(0),
            )
            .map_err(|e| e.to_string())?;
        if count != 0 {
            return Err(format!("oversized {route} entered reply cache"));
        }
    }
    Ok(())
}
fn body(request: &str) -> Result<String, String> {
    if request.contains("hgnc") {
        let directory = crate::runner::launch::fixtures().join("replies/pathway/native/hgnc");
        for entry in std::fs::read_dir(directory).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let wire: Value =
                serde_json::from_slice(&std::fs::read(entry.path()).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            if wire
                .get("url")
                .and_then(Value::as_str)
                .is_some_and(|s| s.contains("withdrawn.txt") == request.contains("withdrawn.txt"))
            {
                return wire
                    .pointer("/exchanges/0/body/value")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .ok_or("HGNC body".into());
            }
        }
        return Err("HGNC setup body missing".into());
    }
    fixtures::body(request)
}
