//! Feature gene-sections: malformed native metadata preserves source submission identity.
#[path = "adapter_interpretation_values/civic_gene.rs"]
mod civic_gene;
#[path = "adapter_interpretation_values/oncokb_gene.rs"]
mod oncokb_gene;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        crate::runner::transcript::trial("gene-sections/oncokb-native-type-keeps-gene", || {
            oncokb_gene::classifications().map_err(Into::into)
        }),
        crate::runner::transcript::trial("gene-sections/oncokb-invalid-version-keeps-gene", || {
            oncokb_gene::version().map_err(Into::into)
        }),
        crate::runner::transcript::trial("gene-sections/civic-invalid-values-keep-gene", || {
            civic_gene::values().map_err(|e| e.to_string().into())
        }),
        crate::runner::transcript::trial(
            "gene-sections/civic-invalid-identities-drop-only-source-gene",
            || civic_gene::identities().map_err(|e| e.to_string().into()),
        ),
        crate::runner::transcript::trial(
            "gene-sections/gencc-invalid-date-keeps-submission",
            || gencc().map_err(|error| error.to_string().into()),
        ),
        crate::runner::transcript::trial("variant-hotspots/hotspots-invalid-values-keep-candidates", || {
            hotspots().map_err(Into::into)
        }),
        crate::runner::transcript::trial("cpic-pgx/invalid-cross-reference-keeps-gene", || {
            cpic().map_err(|error| error.to_string().into())
        }),
        crate::runner::transcript::trial(
            "gene-sections/gencc-invalid-join-is-scoped-to-native-symbol",
            || gencc_identity().map_err(|error| error.to_string().into()),
        ),
    ]
}
fn gencc_identity() -> Result<(), Box<dyn std::error::Error>> {
    crate::runner::response_cases::checked_steps(
        "gene/gencc",
        |source, recording| {
            if source == "gencc"
                && let Some(body) = recording.pointer_mut("/exchanges/1/body/value")
                && let Some(text) = body.as_str()
            {
                *body = json!(text.replacen(
                    "SGC-113621,1,HGNC:8109,ODC1",
                    "SGC-113621,1,invalid,ODC1",
                    1
                ));
            }
        },
        json!([
            {"run":["sync","gencc"],"exit":0,"expect":{"/result/rows":3}},
            {"run":["get","gene","ODC1","gencc","--disable-sources","oncokb"],"exit":0,
             "expect":{
                "/result/items/0/sections/gencc/data/submissions":{"count":2},
                "/problems":{"count":1},"/problems/0/field":"gene_curie",
                "/problems/0/record":"SGC-113621"
             }},
            {"run":["get","gene","MIR21","gencc","--disable-sources","oncokb"],"exit":0,
             "expect":{"/result/items/0/sections/gencc/state":"empty","":{"omits":["problems"]}}}
        ]),
    )?;
    Ok(())
}
fn cpic() -> Result<(), Box<dyn std::error::Error>> {
    crate::runner::response_cases::checked_case(
        "pgx/cpic-synthetic",
        &["get", "pgx", "--gene", "DPYD", "--source", "cpic"],
        |source, recording| {
            if source == "cpic"
                && recording
                    .get("url")
                    .and_then(Value::as_str)
                    .is_some_and(|url| url.contains("/gene?"))
                && let Some(field) = recording.pointer_mut("/exchanges/0/body/value/0/hgncid")
            {
                *field = json!("invalid");
            }
        },
        json!({
            "/result/identity/symbol":"DPYD",
            "/result/identity":{"omits":["hgncid"]},
            "/problems":{"count":1},
            "/problems/0/field":"hgncid",
            "/problems/0/record":"DPYD"
        }),
        0,
    )?;
    Ok(())
}
fn hotspots() -> Result<(), String> {
    for (field, invalid, absent) in [
        ("transcriptId", json!(42), "transcript"),
        ("tumorCount", json!(-1), "tumor_count"),
    ] {
        let stub = crate::runner::network_stub::Stub::for_sources(
            &["myvariant.info", "www.cancerhotspots.org"],
            |_| "application/json",
            move |request| {
                let mut body: Value =
                    serde_json::from_str(&crate::runner::variant_hotspots::body(request)?)
                        .map_err(|e| e.to_string())?;
                if request.starts_with("GET /api/hotspots/") {
                    *body
                        .pointer_mut(&format!("/0/{field}"))
                        .ok_or("missing hotspot field")? = invalid.clone();
                }
                Ok(body.to_string())
            },
        )?;
        let env = crate::runner::variant::source_environment(&stub)?;
        let args = [
            "get",
            "variant",
            "chr7:g.140753336A>T",
            "--build",
            "GRCh38",
            "hotspots",
        ];
        let value =
            crate::runner::configuration::invoke(&env, args.map(str::to_owned).to_vec(), 0)?;
        let record = value
            .pointer("/result/candidates/0/sections/hotspots/data/records/0")
            .ok_or("lost hotspot row")?;
        if record.get("residue") != Some(&json!("V600")) || record.get(absent).is_some() {
            return Err(format!("bad {field} rejected its hotspot record: {value}"));
        }
        let count = value
            .get("problems")
            .and_then(Value::as_array)
            .ok_or("missing problems")?
            .iter()
            .filter(|p| p.get("field") == Some(&json!(field)))
            .count();
        if count != 1 {
            return Err(format!("bad {field} needs one problem: {value}"));
        }
        stub.finish()?;
    }
    Ok(())
}
fn gencc() -> Result<(), Box<dyn std::error::Error>> {
    let value = crate::runner::response_cases::checked_steps(
        "gene/gencc",
        |source, recording| {
            if source != "gencc" {
                return;
            }
            if let Some(body) = recording.pointer_mut("/exchanges/1/body/value")
                && let Some(text) = body.as_str()
            {
                *body = json!(text.replacen("2021-08-04 12:08:26", "   ", 1));
            }
        },
        json!([
            {"run":["sync","gencc"],"exit":0,"expect":{"/result/rows":3}},
            {"run":["get","gene","ODC1","gencc","--disable-sources","oncokb"],"exit":0,
             "expect":{
                 "/result/items/0/sections/gencc/data/submissions":{"count":3},
                 "/result/items/0/sections/gencc/data/submissions/0/id":"SGC-113621",
                 "/result/items/0/sections/gencc/data/submissions/0":{"omits":["evaluated_date"]},
                 "/problems":{"count":1},
                 "/problems/0/field":"submitted_as_date"
             }}
        ]),
    )?;
    if value.pointer("/problems/0/record") != Some(&json!("SGC-113621")) {
        return Err("GenCC field problem lost its source record".into());
    }
    Ok(())
}
