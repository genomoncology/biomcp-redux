//! Entirely authored synthetic source rows; no publisher data or guidance.
use serde_json::{Value, json};
pub(super) fn scenario(name: &str) -> Result<(Vec<String>, Value), String> {
    let mut args = vec![
        if name == "pairs" { "search" } else { "get" },
        "pgx",
        "--gene",
        "DPYD",
        "--source",
        "cpic",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    if name == "pairs" {
        args.extend([
            "--limit".into(),
            "1".into(),
            "--cpic-level".into(),
            "B/C".into(),
        ]);
    } else if matches!(
        name,
        "view" | "missing-second-gene" | "table" | "guidelines" | "guidelines-many"
    ) {
        args.extend(["--section-limit".into(), "1".into()]);
    }
    let body = match name {
        "pairs" => {
            json!(
                [{"pairid":1,"drugid":"RxNorm:10324","drugname":"Synthetic drug","genesymbol":"DPYD","guidelinename":null,"guidelineurl":null,
                "cpiclevel":"B/C","clinpgxlevel":"Synthetic level","pgxtesting":null,"pmids":["12345678"],"usedforrecommendation":"No",
                "provisional":false}]
            )
        }
        "card" => {
            json!(
                [{"symbol":"DPYD","hgncid":"HGNC:2865","ncbiid":"1806","ensemblid":"ENSG00000188641","clinpgxid":"PA145","lookupmethod":"SYNTHETIC",
                "version":7}]
            )
        }
        "view" | "missing-second-gene" | "table" => {
            args.push("recommendations".into());
            if name == "table" {
                args.extend(["--drug-id".into(), "RxNorm:10324".into()]);
            }
            json!([recommendation(name)?])
        }
        "guidelines" | "guidelines-many" => {
            args.push("guidelines".into());
            guideline_rows(name)
        }
        _ => {
            args.extend(["diplotype".into(), "--diplotype".into(), "*1/*2A".into()]);
            if name == "empty" {
                json!([])
            } else {
                json!(
                    [{"genesymbol":"DPYD","diplotype":"*1/*2A","function1":"Synthetic uncertainty","function2":"n/a","activityvalue1":"n/a",
                    "activityvalue2":"n/a","totalactivityscore":"n/a","description":"Synthetic unresolved mapping","diplotypekey":{"DPYD":{"*1":1,
                    "*2A":1}},"generesult":"Synthetic result","ehrpriority":"Synthetic priority","consultationtext":"Synthetic source consultation",
                    "lookupkey":{"DPYD":"n/a"}}]
                )
            }
        }
    };
    Ok((args, body))
}

fn guideline_rows(name: &str) -> Value {
    let symbols = if name == "guidelines-many" {
        vec![
            "DPYD", "TPMT", "NUDT15", "CYP2D6", "UGT1A1", "TP53", "BRCA1", "TPMT",
        ]
    } else {
        vec!["DPYD", "TPMT"]
    };
    let genes = symbols
        .into_iter()
        .map(|symbol| json!({"symbol":symbol,"url":format!("https://example.org/gene/{symbol}")}))
        .collect::<Vec<_>>();
    json!([{"guideline_name":"Synthetic guideline","guideline_url":"https://example.org/synthetic","drugs":["Synthetic A","Synthetic B"],"genes":genes}])
}

fn recommendation(name: &str) -> Result<Value, String> {
    let mut row = json!(
        {"lookupkey":{"DPYD":"n/a","TPMT":"Synthetic other gene"},"implications":{"DPYD":"Synthetic context",
        "TPMT":"Independent synthetic context"},"phenotypes":{"DPYD":"Uncertain synthetic result","TPMT":"Synthetic label"},
        "activityscore":{},"drugrecommendation":"Synthetic source text without clinical advice.",
        "classification":"Synthetic classification","population":"Synthetic population","comments":"Keep this synthetic qualification."}
    );
    // Q16 authored partial-key control: no unavailable second gene becomes normal.
    if name == "missing-second-gene" {
        for field in ["lookupkey", "implications", "phenotypes"] {
            row.get_mut(field)
                .and_then(Value::as_object_mut)
                .ok_or("joint key")?
                .remove("TPMT");
        }
    }
    if matches!(name, "view" | "missing-second-gene") {
        row.as_object_mut()
            .ok_or("synthetic object")?
            .insert("recommendationid".into(), json!(1));
        row.as_object_mut()
            .ok_or("synthetic object")?
            .insert("drugname".into(), json!("Synthetic drug"));
        row.as_object_mut()
            .ok_or("synthetic object")?
            .insert("guidelinename".into(), json!("Synthetic guideline"));
        row.as_object_mut().ok_or("synthetic object")?.insert(
            "guidelineurl".into(),
            json!("https://example.org/synthetic"),
        );
    } else {
        row.as_object_mut()
            .ok_or("synthetic object")?
            .insert("id".into(), json!(1));
        row.as_object_mut()
            .ok_or("synthetic object")?
            .insert("drugid".into(), json!("RxNorm:10324"));
        row.as_object_mut()
            .ok_or("synthetic object")?
            .insert("guidelineid".into(), json!(2));
        row.as_object_mut()
            .ok_or("synthetic object")?
            .insert("allelestatus".into(), json!({"DPYD":"Synthetic status"}));
        row.as_object_mut()
            .ok_or("synthetic object")?
            .insert("version".into(), json!(3));
        row.as_object_mut()
            .ok_or("synthetic object")?
            .insert("dosinginformation".into(), json!(false));
        row.as_object_mut()
            .ok_or("synthetic object")?
            .insert("alternatedrugavailable".into(), json!(true));
        row.as_object_mut()
            .ok_or("synthetic object")?
            .insert("otherprescribingguidance".into(), json!(false));
    }
    Ok(row)
}
