//! Existing native recordings expose the private mapping fact through public adapter calls.
use super::super::{response_cases::copy_capture, transcript::trial};
use biomcp_adapters::{Refusal, gencc, gnomad, hgnc, mygene, pubmed};
use biomcp_fetch::{Fetch, FetchConfig, Mode};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::time::Duration;
pub(super) type Check<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        trial(
            "gene-identity/query-resolution-keeps-actual-refusals",
            || query_resolution().map_err(|e| e.to_string().into()),
        ),
        trial(
            "gene-identity/refused-annotation-hits-carry-batch-facts",
            || mygene_hits().map_err(|e| e.to_string().into()),
        ),
        trial(
            "gene-constraint/refusal-facts-distinguish-native-absence",
            || constraints().map_err(|e| e.to_string().into()),
        ),
        trial(
            "gene-sections/refused-submissions-and-sync-rows-carry-facts",
            || submissions().map_err(|e| e.to_string().into()),
        ),
        trial("article-search/refused-summaries-carry-facts", || {
            summaries().map_err(|e| e.to_string().into())
        }),
    ]
}

pub(super) fn replay(
    capture: &str,
    change: impl FnMut(&str, &mut Value),
) -> Check<(tempfile::TempDir, Fetch)> {
    let root = tempfile::tempdir()?;
    let recordings = root.path().join("recordings");
    copy_capture(capture, &recordings, change)?;
    let mut secrets = biomcp_fetch::Secrets::default();
    for name in [
        biomcp_fetch::SecretName::OncokbApiKey,
        biomcp_fetch::SecretName::NciApiKey,
    ] {
        secrets.insert(name, biomcp_fetch::Secret::new("synthetic-replay-key"));
    }
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay { dir: recordings },
        cache: Some(root.path().join("cache.sqlite")),
        secrets,
        ..Default::default()
    })?;
    Ok((root, fetch))
}

fn mygene_hits() -> Check {
    for case in ["optional", "unknown", "duplicate", "mismatched", "absent"] {
        let mut changed = false;
        let (_root, fetch) = replay("mygene/synthetic-twochunks", |source, recording| {
            if source == "mygene"
                && let Some(body) = recording.pointer_mut("/exchanges/0/body/value")
                && body.is_array()
            {
                changed = true;
                let mut row = body[0].clone();
                match case {
                    "optional" => row["summary"] = json!({"bad_shape":true}),
                    "unknown" => row["query"] = json!("999"),
                    "mismatched" => row["entrezgene"] = json!("999"),
                    "absent" => row = json!({"query":"201","notfound":true}),
                    _ => {}
                }
                *body = if case == "duplicate" {
                    json!([row.clone(), row])
                } else {
                    json!([row])
                };
            }
        })?;
        if !changed {
            return Err("MyGene recording did not contain the selected native row".into());
        }
        let leg = fetch.leg(&mygene::SPEC, fetch.deadline(Duration::from_secs(20)));
        let mut batches = mygene::genes(&leg, &[biodata::NcbiGeneId::new(201)?]);
        let batch = batches.pop().ok_or("missing MyGene batch")?;
        let rows = batch.result?;
        let expected = match case {
            "unknown" | "duplicate" | "mismatched" => Refusal::Refused,
            _ => Refusal::None,
        };
        if batch.refusals != expected || rows.len() != usize::from(case == "optional") {
            return Err(
                format!("MyGene {case}: actual batch refusal or retained row was lost").into(),
            );
        }
        if case == "optional"
            && (rows[0].refusals != Refusal::None
                || rows[0].value.summary.is_some()
                || !rows[0]
                    .problems
                    .iter()
                    .any(|p| p.field.as_ref() == "summary"))
        {
            return Err("MyGene retained optional-field problem became a record refusal".into());
        }
    }
    Ok(())
}

fn constraints() -> Check {
    for case in [
        "optional",
        "wrong-id",
        "absent",
        "bad-record",
        "missing-container",
        "empty-container",
        "invalid-container",
    ] {
        let mut changed = false;
        let (_root, fetch) = replay("variant/adapter-values/gnomad", |source, recording| {
            if source == "gnomad"
                && let Some(gene) = recording.pointer_mut("/exchanges/0/body/value/data/g0")
            {
                changed = true;
                match case {
                    "wrong-id" => gene["gene_id"] = json!("ENSG00000111111"),
                    "absent" => *gene = Value::Null,
                    "bad-record" => *gene = json!({"gene_id":true}),
                    "missing-container" => gene["gnomad_constraint"] = Value::Null,
                    "empty-container" => gene["gnomad_constraint"] = json!({}),
                    "invalid-container" => gene["gnomad_constraint"] = json!("invalid"),
                    _ => {}
                }
                if case == "absent" {
                    recording["exchanges"][0]["body"]["value"]["errors"] =
                        json!([{"message":"Gene not found"}]);
                }
            }
        })?;
        if !changed {
            return Err("constraint recording did not contain g0".into());
        }
        let leg = fetch.leg(&gnomad::SPEC, fetch.deadline(Duration::from_secs(20)));
        let mut rows = gnomad::constraints(&leg, &["ENSG00000141510".parse()?])?;
        let mapped = rows
            .pop()
            .ok_or("missing constraint source identity")?
            .constraint;
        let expected = if matches!(case, "wrong-id" | "bad-record") {
            Refusal::Refused
        } else {
            Refusal::None
        };
        let present = matches!(case, "optional" | "empty-container");
        if mapped.refusals != expected || mapped.value.is_some() != present {
            return Err(format!(
                "constraint {case}: absence, container or actual identity refusal changed"
            )
            .into());
        }
        if case == "optional" && !mapped.problems.iter().any(|p| p.field.as_ref() == "pli") {
            return Err("constraint optional-field control did not reach its problem".into());
        }
        if case == "invalid-container"
            && !mapped
                .problems
                .iter()
                .any(|p| p.field.as_ref() == "gnomad_constraint")
        {
            return Err("constraint invalid-container control did not reach its problem".into());
        }
    }
    Ok(())
}

fn submissions() -> Check {
    for case in ["optional", "identity", "decoder", "absent"] {
        let mut changed = false;
        let (_root, fetch) = replay("gene/gencc", |source, recording| {
            if source == "gencc"
                && let Some(body) = recording.pointer_mut("/exchanges/1/body/value")
                && let Some(native) = body.as_str()
            {
                let replacement = match case {
                    "identity" => native.replacen(
                        "SGC-113621,1,HGNC:8109,ODC1",
                        "SGC-113621,1,invalid,ODC1",
                        1,
                    ),
                    "decoder" => {
                        native.replacen("SGC-113621,1,HGNC:8109,ODC1", "SGC-113621,1,invalid,", 1)
                    }
                    "optional" => native.replacen("2021-08-04 12:08:26", "   ", 1),
                    _ => native.to_owned(),
                };
                changed = case == "absent" || replacement != native;
                *body = json!(replacement);
            }
        })?;
        if !changed {
            return Err(format!("GenCC {case}: native fixture edit did not apply").into());
        }
        let synchronized = gencc::sync(&fetch)?;
        let sync_expected = if case == "decoder" {
            Refusal::Refused
        } else {
            Refusal::None
        };
        if synchronized.refusals != sync_expected {
            return Err(format!("GenCC {case}: sync lost its actual decoder rejection").into());
        }
        let leg = fetch.leg(&gencc::SPEC, fetch.deadline(Duration::from_secs(20)));
        let (id, symbol) = if case == "absent" {
            ("HGNC:31586", "MIR21")
        } else {
            ("HGNC:8109", "ODC1")
        };
        let mapped = gencc::gene(&leg, id.parse()?, &symbol.parse()?)?;
        let expected = if case == "identity" {
            Refusal::Refused
        } else {
            Refusal::None
        };
        if mapped.refusals != expected || mapped.value.is_some() != (case != "absent") {
            return Err(
                format!("GenCC {case}: submission absence and refusal were conflated").into(),
            );
        }
        if case == "optional"
            && !mapped
                .problems
                .iter()
                .any(|p| p.field.as_ref() == "submitted_as_date")
        {
            return Err("GenCC optional-field control did not reach its problem".into());
        }
    }
    Ok(())
}

fn summaries() -> Check {
    for case in [
        "optional",
        "identity",
        "absent",
        "missing-container",
        "empty-container",
        "invalid-container",
    ] {
        let mut changed = false;
        let (_root, fetch) = replay(
            "article/semantic-scholar/synthetic-cut",
            |source, recording| {
                if source == "pubmed"
                    && let Some(result) = recording.pointer_mut("/exchanges/0/body/value/result")
                    && let Some(result) = result.as_object_mut()
                {
                    changed = true;
                    if case == "absent" {
                        result.remove("17611497");
                    } else if let Some(row) = result.get_mut("17611497") {
                        match case {
                            "identity" => row["uid"] = json!("999"),
                            "optional" => row["title"] = json!({"bad_shape":true}),
                            "missing-container" => {
                                if let Some(fields) = row.as_object_mut() {
                                    fields.remove("authors");
                                }
                            }
                            "empty-container" => row["authors"] = json!([]),
                            "invalid-container" => row["authors"] = json!({"bad_shape":true}),
                            _ => {}
                        }
                    }
                }
            },
        )?;
        if !changed {
            return Err("PubMed recording did not contain summary rows".into());
        }
        let leg = fetch.leg(&pubmed::SPEC, fetch.deadline(Duration::from_secs(20)));
        let mapped = pubmed::summaries(
            &leg,
            &[biodata::Pmid::new(17611497)?, biodata::Pmid::new(27959700)?],
            None,
        )?;
        let expected = if case == "identity" {
            Refusal::Refused
        } else {
            Refusal::None
        };
        let retained = if matches!(case, "identity" | "absent") {
            1
        } else {
            2
        };
        if mapped.refusals != expected || mapped.value.len() != retained {
            return Err(format!(
                "PubMed {case}: summary absence, field problem or refusal changed"
            )
            .into());
        }
        if case == "optional" && !mapped.problems.iter().any(|p| p.field.as_ref() == "title") {
            return Err("PubMed optional-field control did not reach its problem".into());
        }
        if case == "invalid-container"
            && !mapped
                .problems
                .iter()
                .any(|p| p.field.as_ref() == "authors")
        {
            return Err("PubMed invalid-container control did not reach its problem".into());
        }
    }
    Ok(())
}

fn query_resolution() -> Check {
    for refused in [false, true] {
        let mut changed = false;
        let (_root, fetch) = replay("gene/gencc", |source, recording| {
            if source == "hgnc"
                && let Some(body) = recording.pointer_mut("/exchanges/0/body/value")
                && let Some(native) = body.as_str()
            {
                changed = true;
                *body = if refused {
                    json!(format!(
                        "{native}invalid\tODC1\trefused identity\tprotein-coding gene\tApproved\n"
                    ))
                } else {
                    json!(native.replacen("protein-coding gene", "invalid optional locus group", 1))
                };
            }
        })?;
        if !changed {
            return Err("HGNC recording did not contain native complete rows".into());
        }
        hgnc::sync(&fetch, hgnc::HgncBundle::Complete)?;
        let leg = fetch.leg(&hgnc::SPEC, fetch.deadline(Duration::from_secs(20)));
        let mapped = biomcp_query::gene::resolve(&leg, &"ODC1".parse()?)?;
        let expected = if refused {
            Refusal::Refused
        } else {
            Refusal::None
        };
        if mapped.refusals != expected
            || !matches!(mapped.value, biomcp_types::GeneResolution::Resolved { .. })
        {
            return Err(
                "HGNC query aggregation lost a retained gene or its actual refusal fact".into(),
            );
        }
        if !refused
            && !mapped
                .problems
                .iter()
                .any(|p| p.field.as_ref() == "locus_group")
        {
            return Err("HGNC optional-field control did not reach its problem".into());
        }
    }
    Ok(())
}
