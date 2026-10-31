//! Variant sections share an absolute replay deadline without adding sibling delays.
use super::super::{launch::Environment, response_cases::copy_capture};
use biomcp_fetch::{Fetch, FetchConfig, Mode};
use biomcp_types::{Request, VariantGet, VariantResolver, VariantSection};
use serde_json::Value;
use std::time::Duration;

pub(super) fn check() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::new()?;
    let replay = env.root().join("section-delay-replay");
    let delay = Duration::from_millis(600);
    let budget = Duration::from_secs(1);
    let mut delayed = 0;
    copy_capture("variant/synthetic-components", &replay, |_, record| {
        let url = record.get("url").and_then(Value::as_str).unwrap_or("");
        let section = url.contains("chr7%3Ag.140453136A%3ET?")
            && (url.contains("cadd.phred") || url.contains("dbnsfp.gerp"));
        if section {
            delayed += 1;
        }
        if let Some(exchanges) = record.get_mut("exchanges").and_then(Value::as_array_mut) {
            for exchange in exchanges {
                exchange["delay_ms"] = if section { delay.as_millis() as u64 } else { 0 }.into();
            }
        }
    })?;
    if delayed != 2 || delay >= budget || delay + delay <= budget {
        return Err("replay must delay exactly two sections that fit only as siblings".into());
    }
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay { dir: replay },
        ..FetchConfig::default()
    })?;
    let query = VariantGet::new(
        "chr7:g.140453136A>T".parse()?,
        VariantResolver::Car,
        Some(biodata::Build::GRCh37),
    )?
    // Request order differs from declaration order, which must still be retained.
    .with_sections(Box::new([
        VariantSection::Conservation,
        VariantSection::Predict,
    ]));
    let outcome = biomcp_query::run(
        &biomcp_query::Ctx {
            fetch: &fetch,
            budget,
            ncbi_email: None,
            fresh: false,
        },
        &Request::GetVariant(query),
    );
    let body = serde_json::to_value(&outcome.body)?;
    for section in ["predict", "conservation"] {
        if body.pointer(&format!("/candidates/0/sections/{section}/state"))
            != Some(&Value::from("data"))
        {
            return Err(format!(
                "{section} did not complete under one replay deadline: {outcome:?}"
            )
            .into());
        }
    }
    if outcome.local.is_some()
        || outcome
            .sources
            .iter()
            .any(|source| source.reason == Some(biomcp_types::Reason::DeadlineExceeded))
    {
        return Err(format!("a sibling delay exhausted the shared budget: {outcome:?}").into());
    }
    Ok(())
}
