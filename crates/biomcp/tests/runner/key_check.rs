//! Exercise source credential checks through the CLI and hosted HTTP.
use super::{key_registration::invoke, launch::Environment, transcript::trial};
use libtest_mimic::Trial;

pub(super) fn trials() -> Vec<Trial> {
    vec![trial("source-key-check/keyless", || {
        let env = Environment::new()?;
        let reply = invoke(&env, &["--offline", "key", "check", "clinicaltrials"], "", 0)?;
        if reply.pointer("/result/status").and_then(serde_json::Value::as_str) != Some("no_key_required") {
            return Err("ClinicalTrials.gov did not report that it needs no key".into());
        }
        Ok(())
    })]
}
