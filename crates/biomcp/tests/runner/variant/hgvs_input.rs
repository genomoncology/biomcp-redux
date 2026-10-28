//! Feature variant-identity: implausible HGVS fails before source access.
use crate::runner::{configuration::invoke, network_stub::Stub};
use serde_json::json;

pub(super) fn check() -> Result<(), String> {
    let stub = Stub::for_source(
        "reg.genome.network",
        "application/json",
        |_| Ok("{}".into()),
    )?;
    let environment = super::source_environment(&stub)?;
    for args in [
        vec!["get", "variant", "NM_005228.5:c.???"],
        vec![
            "variant",
            "normalize",
            "--source",
            "car",
            "--hgvs",
            "NM_005228.5:c.2369C>T word",
        ],
    ] {
        let args = args.into_iter().map(str::to_owned).collect();
        let value = invoke(&environment, args, 2)?;
        if value.get("schema") != Some(&json!("biomcp.error/1")) {
            return Err(format!(
                "implausible HGVS did not return a usage error: {value}"
            ));
        }
    }
    let requests = stub.finish()?;
    if !requests.is_empty() {
        return Err(format!("implausible HGVS reached a source: {requests:?}"));
    }
    Ok(())
}
