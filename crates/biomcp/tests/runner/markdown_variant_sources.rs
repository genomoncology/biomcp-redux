//! Positive section views reuse the existing synthetic source responders.
use super::{launch::Environment, markdown_shapes::{Shape, check_environment}, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::json;
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    ["civic", "hotspots", "oncokb"].into_iter().map(|source| {
        Trial::test(format!("markdown-view/variant-{source}-data"), move || positive(source).map_err(Into::into))
    }).collect()
}
fn positive(source: &str) -> Result<(), String> {
    let (hosts, reply): (&[&str], fn(&str) -> Result<String, String>) = match source {
        "civic" => (&["civicdb.org"], super::variant_civic::civic_body),
        "hotspots" => (&["myvariant.info", "www.cancerhotspots.org"], super::variant_hotspots::body),
        _ => (&["myvariant.info", "www.oncokb.org"], super::oncokb::reply),
    };
    let stub = Stub::for_sources(hosts, |_| "application/json", reply)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca,
        "keys":{"oncokb_api_key":"synthetic-markdown-key"}});
    let shape = match source {
        "civic" => Shape { args: &["get","variant","CA123643","civic","--disable-sources","myvariant"], replies:"", required: &["## Result","civic","BRAF V600E","dabrafenib"], max_bytes:1 },
        "hotspots" => Shape { args: &["get","variant","chr7:g.140753336A>T","--build","GRCh38","hotspots"], replies:"", required: &["## Result","hotspots","ENST00000288602","V600","candidate"], max_bytes:1 },
        _ => Shape { args: &["get","variant","chr7:g.140753336A>T","oncokb","--build","GRCh38","--tumor-type","Synthetic tumor","--section-limit","50"], replies:"", required: &["## Result","oncokb","Oncogenic","Synthetic mutation effect."], max_bytes:1 },
    };
    check_environment(&shape, &env, &[], config)
}
