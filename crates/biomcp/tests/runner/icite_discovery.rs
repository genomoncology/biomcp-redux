//! Source-owned citation order and rebatching through the checked adapter boundary.
use super::super::super::{launch::Environment, network_stub::Stub};
use biomcp_fetch::{Certificates, Fetch, FetchConfig, Network, Observations};
use serde_json::json;
use std::{fs, sync::Arc, time::Duration};

pub(super) fn prove() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::new()?;
    for references in [false, true] {
        let mut sides = Vec::new();
        for count in [400, 402] {
            // Descending IDs and a repeated first ID expose sorting and failed deduplication.
            let ids = (1000..1000 + count).rev().collect::<Vec<u32>>();
            let expected = ids.iter().map(ToString::to_string).collect::<Vec<_>>();
            let mut native = ids.clone();
            native.push(ids[0]);
            let body = if references {
                json!({"data":[{"pmid":1,"citedPmids":native,"citedByPmids":[99]}]})
            } else {
                json!({"data":[{"pmid":1,"citedByPmids":native,"citedPmids":[99]}]})
            }
            .to_string();
            let stub = Stub::for_sources(
                &["icite.od.nih.gov"],
                |_| "application/json",
                move |request| {
                    Ok(if request.contains("pmids=1&") {
                        body.clone()
                    } else {
                        r#"{"data":[]}"#.into()
                    })
                },
            )?;
            stub.retain(env.shared_root());
            let path = env.root().join(format!("icite-{references}-{count}.jsonl"));
            let fetch = Fetch::open(FetchConfig {
                observations: Some(Arc::new(Observations::create(&path)?)),
                network: Network::new(
                    None,
                    Some(biomcp_fetch::Secret::new(format!(
                        "http://{}",
                        stub.address
                    ))),
                    "",
                    Certificates::from_pem(stub.certificate.as_bytes())?,
                )?,
                ..FetchConfig::default()
            })?;
            let spec = &biomcp_adapters::icite::SPEC;
            let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(20)));
            let parent = [biodata::Pmid::new(1)?];
            let mapped = if references {
                biomcp_adapters::icite::references(&leg, &parent)?
            } else {
                biomcp_adapters::icite::publications(&leg, &parent)?
            };
            let row = mapped.value.into_iter().next().ok_or("missing parent")?;
            let selected = if references {
                row.references
            } else {
                row.citing
            }
            .ok_or("missing selection")?;
            if selected.iter().map(ToString::to_string).collect::<Vec<_>>() != expected {
                return Err("command selection changed native order".into());
            }
            // Child requests use the actual source selection, not test-authored provenance.
            // Empty child bodies are valid metadata absence and preserve request observations.
            for batch in selected.chunks(biomcp_adapters::icite::BATCH_LIMIT) {
                biomcp_adapters::icite::publications(&leg, batch)?;
            }
            let rows = fs::read_to_string(path)?
                .lines()
                .map(serde_json::from_str::<serde_json::Value>)
                .collect::<Result<Vec<_>, _>>()?;
            if rows[0].pointer("/provenance/discovery/ids") != Some(&json!(expected)) {
                return Err("iCite discovery lost native list order or retained duplicates".into());
            }
            sides.push(json!({"requests": rows}));
        }
        let report = env.root().join(format!("icite-{references}.json"));
        fs::write(
            &report,
            serde_json::to_vec(&json!({"baseline":sides[0],"live":sides[1],"ordinal":1}))?,
        )?;
        let script = super::super::super::launch::fixtures()
            .parent()
            .ok_or("missing root")?
            .join("scripts/icite_discovery_proof.py");
        let output = env
            .command(
                "python3",
                &[script.display().to_string(), report.display().to_string()],
            )
            .output()?;
        if !output.status.success() {
            return Err(format!("iCite classifier proof failed: {output:?}").into());
        }
    }
    Ok(())
}
