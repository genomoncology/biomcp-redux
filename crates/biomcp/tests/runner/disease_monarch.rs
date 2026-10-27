//! Feature disease-monarch: native association facts stay source-owned.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};
mod evidence;
mod failures;
mod native;
pub(super) use native::run as native;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![Trial::test(
        "disease-monarch/native-endpoints-and-paging",
        || exact().map_err(Into::into),
    )];
    trials.push(Trial::test(
        "disease-monarch/phenotype-evidence-and-independent-native-qualifiers",
        || phenotype().map_err(Into::into),
    ));
    trials.push(Trial::test(
        "disease-monarch/five-checked-genes-keep-pivots-and-native-page",
        || navigation().map_err(Into::into),
    ));
    trials.push(Trial::test(
        "disease-monarch/only-distinct-checked-native-gene-identities-pivot",
        || checked_identities().map_err(Into::into),
    ));
    trials.push(Trial::test(
        "disease-monarch/native-aggregators-keep-missing-null-and-arrays",
        || aggregator_presence().map_err(Into::into),
    ));
    trials.push(Trial::test(
        "disease-monarch/source-defaults-and-explicit-limits",
        || defaults().map_err(Into::into),
    ));
    trials.push(Trial::test(
        "disease-monarch/evidence-preserves-unavailable-empty-and-populated",
        || evidence::run().map_err(Into::into),
    ));
    trials.extend(failures::trials());
    trials
}
fn setup(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("monarch-ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn invoke(env: &Environment, words: &[&str], exit: i32) -> Result<Value, String> {
    let output = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &words.iter().map(|s| (*s).into()).collect::<Vec<String>>(),
        )
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) {
        return Err(format!(
            "unexpected exit {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn association() -> Value {
    json!(
        {"id":"synthetic-row","subject":"HGNC:9967","subject_label":"Synthetic gene","object":"MONDO:0007958",
        "object_label":"Synthetic child","predicate":"biolink:associated_with","original_subject":"NCBIGene:5979",
        "original_object":"Orphanet:99361","primary_knowledge_source":"infores:orphanet",
        "aggregator_knowledge_source":["infores:monarchinitiative","infores:hpo-annotations"],"provided_by":"synthetic_collection",
        "has_evidence":["ECO:0000501"],"qualifiers":null,"negated":false,"subject_closure":null,"object_closure":["MONDO:0019003"]}
    )
}
fn exact() -> Result<(), String> {
    let stub = Stub::for_source_status(
        "api-v3.monarchinitiative.org",
        "application/json",
        |request| {
            if !request.starts_with("GET /v3/api/association?object=MONDO%3A0019003&subject_category=biolink%3AGene&limit=5&offset=0 ") {
                return Err(format!("unexpected request: {request}"));
            }

            let mut row = association();
            row.as_object_mut()
                .ok_or("missing synthetic row")?
                .insert("subject_taxon".into(), json!("NCBITaxon:9606"));
            Ok(json!({"limit":5,"offset":0,"total":8,"items":[row]}).to_string())
        },
        |_| 200,
    )?;
    let env = setup(&stub)?;
    let value = invoke(
        &env,
        &[
            "get",
            "disease",
            "MONDO:0019003",
            "genes",
            "--source",
            "monarch",
            "--section-limit",
            "5",
        ],
        0,
    )?;
    measure(&["exact-native"], &value)?;
    // Ceil(j12282 comparable1833 * 1.15).
    if super::source_terms::size(&value, "")? > 2108 {
        return Err("exact native mechanics page exceeds its measured bound".into());
    }
    let row = value
        .pointer("/result/items/0/association")
        .ok_or("missing association")?;
    for (key, expected) in [
        ("requested_id", json!("MONDO:0019003")),
        ("subject_taxon", json!("NCBITaxon:9606")),
        (
            "object",
            json!({"id":"MONDO:0007958","label":"Synthetic child"}),
        ),
        (
            "aggregator_knowledge_source",
            json!(["infores:monarchinitiative", "infores:hpo-annotations"]),
        ),
        ("qualifiers", json!([])),
    ] {
        if row.get(key) != Some(&expected) {
            return Err(format!("lost {key}: {row}"));
        }
    }
    if value.pointer("/page/total") != Some(&json!(8))
        || value.pointer("/page/returned") != Some(&json!(1))
    {
        return Err("association page counts changed".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("unexpected fanout".into());
    }
    Ok(())
}

fn record(env: &Environment, words: &[&str], directory: &Path, exit: i32) -> Result<Value, String> {
    let mut words = words.to_vec();
    let path = directory.to_str().ok_or("non-UTF8 path")?;
    words.extend(["--record", path]);
    let value = invoke(env, &words, exit)?;
    measure(&words, &value)?;
    Ok(value)
}
fn measure(words: &[&str], value: &Value) -> Result<(), String> {
    use std::io::Write;
    writeln!(
        std::io::stderr(),
        "Monarch {:?}: full={} comparable={}",
        words,
        serde_json::to_vec(value).map_err(|e| e.to_string())?.len(),
        super::source_terms::size(value, "")?
    )
    .map_err(|e| e.to_string())
}
fn replay(directory: &Path, mut steps: Value) -> Result<Value, String> {
    for step in steps.as_array_mut().ok_or("missing steps")? {
        let step = step.as_object_mut().ok_or("missing step")?;
        let ceiling = step
            .get("max_bytes")
            .and_then(Value::as_u64)
            .ok_or("bad ceiling")?;
        step.insert("max_tokens".into(), json!(ceiling.div_ceil(4)));
    }
    let file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
    fs::write(
        file.path(),
        serde_json::to_vec(&json!(
            {"feature":"disease-monarch","story":"Selected association rows preserve native context","replay":directory,"steps":steps}
        ))
        .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let value = super::run_case(file.path(), &super::schema::Registry::embedded()?)?;
    measure(&["replay"], &value)?;
    Ok(value)
}

fn phenotype() -> Result<(), String> {
    let stub = Stub::for_source_status(
        "api-v3.monarchinitiative.org",
        "application/json",
        |_| {
            let mut row = association();
            let map = row.as_object_mut().ok_or("missing synthetic row")?;
            for (key, value) in [
                ("subject", json!("MONDO:0019003")),
                ("object", json!("HP:0000001")),
                ("frequency_qualifier", json!("HP:0040283")),
                ("qualifiers", json!(["HP:0000002"])),
                ("onset_qualifier", json!("HP:0000003")),
                ("sex_qualifier", json!("PATO:0000384")),
                ("stage_qualifier", json!("HP:0000004")),
                ("species_context_qualifier", json!("NCBITaxon:9606")),
                ("disease_context_qualifier", json!("MONDO:0019003")),
                ("frequency_qualifier_label", json!("Synthetic frequency")),
                ("has_count", json!(24)),
                ("has_total", json!(35)),
                ("has_percentage", json!(68.57143)),
                ("has_quotient", json!(0.6857143)),
                (
                    "has_evidence_links",
                    json!([{"id":"ECO:0000501","label":"Synthetic evidence","url":"http://purl.obolibrary.org/obo/ECO_0000501"}]),
                ),
                ("publications", json!(["PMID:1"])),
                (
                    "publications_links",
                    json!([{"id":"PMID:1","label":"Synthetic publication"}]),
                ),
            ] {
                map.insert(key.into(), value);
            }
            Ok(json!({"limit":5,"offset":0,"total":1,"items":[row]}).to_string())
        },
        |_| 200,
    )?;
    let env = setup(&stub)?;
    let words = [
        "get",
        "disease",
        "MONDO:0019003",
        "phenotypes",
        "--source",
        "monarch",
        "--section-limit",
        "5",
    ];
    let directory = env.root().join("phenotypes");
    let value = record(&env, &words, &directory, 0)?;
    if value.get("next").is_some() {
        return Err("phenotype received gene or page action".into());
    }
    replay(
        &directory,
        json!(
            [{"run":words,"mcp":words,"expect":{"/result/items/0/association/evidence/0/label":"Synthetic evidence",
            "/result/items/0/association/publications/0":{"equals":{"id":"PMID:1","label":"Synthetic publication"}},
            "/result/items/0/association/has_count":24,"/result/items/0/association/has_total":35,
            "/result/items/0/association/has_percentage":68.57143,"/result/items/0/association/has_quotient":0.6857143},"max_bytes":2588}]
        ),
    )?;
    if stub.finish()?.len() != 1 {
        return Err("unexpected phenotype fanout".into());
    }
    Ok(())
}

fn navigation() -> Result<(), String> {
    let stub = Stub::for_source_status(
        "api-v3.monarchinitiative.org",
        "application/json",
        |_| {
            let mut items = Vec::new();
            for number in 1..=5 {
                let mut row = association();
                row.as_object_mut()
                    .ok_or("missing synthetic row")?
                    .insert("subject".into(), json!(format!("HGNC:{number}")));
                items.push(row);
            }
            Ok(json!({"limit":5,"offset":0,"total":6,"items":items}).to_string())
        },
        |_| 200,
    )?;
    let env = setup(&stub)?;
    let directory = env.root().join("navigation");
    let words = [
        "get",
        "disease",
        "MONDO:0019003",
        "genes",
        "--source",
        "monarch",
        "--section-limit",
        "5",
    ];
    let value = record(&env, &words, &directory, 0)?;
    let next = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("missing navigation")?;
    if next.len() != 6 {
        return Err(format!("checked-gene pivot was truncated: {next:?}"));
    }
    for (index, row) in next.iter().take(5).enumerate() {
        if row.get("args") != Some(&json!(["get", "gene", format!("HGNC:{}", index + 1)])) {
            return Err("gene source order changed".into());
        }
    }
    if next.get(5).and_then(|row| row.get("kind")) != Some(&json!("page")) {
        return Err("native page was lost".into());
    }
    replay(
        &directory,
        json!([{"run":words,"mcp":words,"expect":{"/next/5/kind":"page","/page/total":6},"max_bytes":6909}]),
    )?;
    if stub.finish()?.len() != 1 {
        return Err("gene navigation made additional source requests".into());
    }
    Ok(())
}

fn checked_identities() -> Result<(), String> {
    let stub = Stub::for_source_status(
        "api-v3.monarchinitiative.org",
        "application/json",
        |_| {
            let mut unchecked = association();
            unchecked
                .as_object_mut()
                .ok_or("missing synthetic row")?
                .insert("subject".into(), json!("native:unresolved"));
            let mut ncbi = association();
            ncbi.as_object_mut()
                .ok_or("missing synthetic row")?
                .insert("subject".into(), json!("NCBIGene:5979"));
            Ok(json!({"limit":5,"offset":0,"total":4,"items":[association(),association(),unchecked,ncbi]}).to_string())
        },
        |_| 200,
    )?;
    let env = setup(&stub)?;
    let directory = env.root().join("checked-identities");
    let words = [
        "get",
        "disease",
        "MONDO:0019003",
        "genes",
        "--source",
        "monarch",
        "--section-limit",
        "5",
    ];
    let value = record(&env, &words, &directory, 0)?;
    let next = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("missing checked gene navigation")?;
    if next.len() != 2
        || next.first().and_then(|row| row.get("args"))
            != Some(&json!(["get", "gene", "HGNC:9967"]))
        || next.get(1).and_then(|row| row.get("args")) != Some(&json!(["get", "gene", "5979"]))
    {
        return Err(format!("invented or duplicated gene identity: {next:?}"));
    }
    replay(
        &directory,
        json!(
            [{"run":words,"mcp":words,"expect":{"/page/total":4,"/page/returned":4,
            "/result/items/2/association/subject/id":"native:unresolved"},"max_bytes":5484}]
        ),
    )?;
    if stub.finish()?.len() != 1 {
        return Err("gene pivots fetched genes".into());
    }
    Ok(())
}

fn aggregator_presence() -> Result<(), String> {
    for value in [
        None,
        Some(json!(null)),
        Some(json!([])),
        Some(json!(["infores:monarchinitiative"])),
    ] {
        let expected = value.clone();
        let stub = Stub::for_source_status(
            "api-v3.monarchinitiative.org",
            "application/json",
            move |_| {
                let mut row = association();
                let map = row.as_object_mut().ok_or("missing native row")?;
                map.remove("aggregator_knowledge_source");
                if let Some(value) = value.clone() {
                    map.insert("aggregator_knowledge_source".into(), value);
                }
                Ok(json!({"limit":5,"offset":0,"total":1,"items":[row]}).to_string())
            },
            |_| 200,
        )?;
        let env = setup(&stub)?;
        let words = [
            "get",
            "disease",
            "MONDO:0019003",
            "genes",
            "--source",
            "monarch",
            "--section-limit",
            "5",
        ];
        let result = invoke(&env, &words, 0)?;
        if result.pointer("/result/items/0/association/aggregator_knowledge_source")
            != expected.as_ref()
        {
            return Err("native missing/null/list aggregators collapsed".into());
        }
        if super::source_terms::size(&result, "")? > 1983 {
            return Err("aggregator mechanics page exceeds measured bound".into());
        }
        if stub.finish()?.len() != 1 {
            return Err("unexpected aggregator source request count".into());
        }
    }
    Ok(())
}

fn default_reply(request: &str) -> Result<String, String> {
    let target = request.split_whitespace().nth(1).ok_or("missing target")?;
    let parameter = |key: &str| target.split('&').find_map(|part| part.strip_prefix(key));
    let limit: usize = parameter("limit=")
        .ok_or("missing limit")?
        .parse()
        .map_err(|_| "invalid limit")?;
    let offset: usize = parameter("offset=")
        .ok_or("missing offset")?
        .parse()
        .map_err(|_| "invalid offset")?;
    let phenotype = target.contains("?subject=");
    let items = (offset..(offset + limit).min(6))
        .map(|i| {
            let mut row = association();
            let fields = row.as_object_mut().ok_or("missing synthetic association")?;
            fields.insert("id".into(), json!(format!("synthetic-{i}")));
            if phenotype {
                fields.insert("subject".into(), json!("MONDO:0019003"));
                fields.insert("object".into(), json!(format!("HP:{:07}", i + 1)));
            } else {
                fields.insert("subject".into(), json!(format!("HGNC:{}", i + 1)));
            }
            Ok(row)
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(json!({"limit":limit,"offset":offset,"total":6,"items":items}).to_string())
}
fn defaults() -> Result<(), String> {
    let stub = Stub::for_source(
        "api-v3.monarchinitiative.org",
        "application/json",
        default_reply,
    )?;
    let env = setup(&stub)?;
    for (section, default) in [("genes", 2), ("phenotypes", 1)] {
        for explicit in [false, true] {
            default_walk(&env, section, if explicit { 5 } else { default }, explicit)?;
        }
    }
    for value in ["0", "null", "51"] {
        let words = [
            "get",
            "disease",
            "MONDO:0019003",
            "genes",
            "--source",
            "monarch",
            "--section-limit",
            value,
        ];
        let file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
        fs::write(
            file.path(),
            serde_json::to_vec(&json!(
                {"feature":"disease-monarch","story":"Invalid explicit limits fail before source access","steps":[{"run":words,"mcp":words,"exit":2,
                "is_error":true,"expect":{"/status":"usage","/error/code":"usage.arguments","/error/option":"--section-limit"},"max_bytes":171}]}
            ))
            .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        super::run_case(file.path(), &super::schema::Registry::embedded()?)?;
    }
    if stub.finish()?.len() != 13 {
        return Err("default or explicit limit made an extra request".into());
    }
    Ok(())
}

fn default_walk(
    env: &Environment,
    section: &str,
    count: usize,
    explicit: bool,
) -> Result<(), String> {
    for offset in (0..6).step_by(count) {
        let offset_text = offset.to_string();
        let mut words = vec![
            "get",
            "disease",
            "MONDO:0019003",
            section,
            "--source",
            "monarch",
        ];
        if explicit {
            words.extend(["--section-limit", "5"]);
        }
        words.extend(["--section-offset", &offset_text]);
        let returned = count.min(6 - offset);
        let ids = (offset..offset + returned)
            .map(|i| format!("synthetic-{i}"))
            .collect::<Vec<_>>();
        let directory = env.root().join(format!("{section}-{explicit}-{offset}"));
        record(env, &words, &directory, 0)?;
        let mut expect = json!(
            {"/query/section_limit":count,"/page/offset":offset,"/page/returned":returned,"/page/total":6,
            "/result/items/*/association/attribution/record":ids}
        );
        if offset + count < 6 {
            expect
                .as_object_mut()
                .ok_or("missing expectation")?
                .insert("/next/*/kind".into(), json!({"includes":"page"}));
        }
        let cap = match (section, explicit) {
            ("genes", false) => 3324,
            ("genes", true) => 6922,
            ("phenotypes", false) => 2083,
            _ => 6646,
        };
        replay(
            &directory,
            json!([{"run":words,"mcp":words,"expect":expect,"max_bytes":cap}]),
        )?;
    }
    Ok(())
}
