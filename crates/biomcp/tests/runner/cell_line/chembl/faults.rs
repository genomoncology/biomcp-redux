use super::{
    MEASURE, Stub, Trial, Value, compared, fixture, fs, invoke, invoke_measured, json,
    measure_bytes, parity, path, reply, setup, stub, words,
};
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = Vec::new();
    for assay in [false, true] {
        for fault in ["429", "500", "broken", "html", "timeout"] {
            trials.push(Trial::test(
                format!("cell-line-chembl/fault/{}/{fault}", surface(assay)),
                move || failure(assay, fault).map_err(Into::into),
            ));
        }
        for fault in ["mismatch", "missing", "malformed", "bad-page", "next"] {
            trials.push(Trial::test(
                format!("cell-line-chembl/cache/{}/{fault}", surface(assay)),
                move || admission(assay, fault).map_err(Into::into),
            ));
        }
        trials.push(Trial::test(
            format!("cell-line-chembl/cache/{}/disabled", surface(assay)),
            move || disabled(assay).map_err(Into::into),
        ));
        trials.push(Trial::test(
            format!("cell-line-chembl/provenance/{}/500", surface(assay)),
            move || release(assay).map_err(Into::into),
        ));
    }
    trials.push(Trial::test("cell-line-chembl/synthetic/zero", || {
        zero().map_err(Into::into)
    }));
    trials.push(Trial::test("cell-line-chembl/synthetic/more", || {
        more().map_err(Into::into)
    }));
    trials
}
fn failed(value: &Value) -> Result<(), String> {
    path(value, "/status", json!("failed"))?;
    path(value, "/sources/0/source", json!("chembl"))?;
    path(value, "/sources/0/state", json!("failed"))?;
    path(value, "/sources/0/use", json!("attribution"))?;
    if value.get("result").is_some() {
        return Err("failure became a valid count or empty metadata".into());
    }
    if value
        .pointer("/sources/0/terms_url")
        .and_then(Value::as_str)
        .is_none_or(str::is_empty)
    {
        return Err("failure lost terms".into());
    }
    Ok(())
}
fn surface(assay: bool) -> &'static str {
    if assay { "assays" } else { "metadata" }
}
fn failure(assay: bool, fault: &str) -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", |_| Ok("{}".into()))?;
    let env = setup(&stub)?;
    let directory = env.root().join("seed");
    let words = words(assay);
    let mut args = words.clone();
    args.extend(["--record", directory.to_str().ok_or("recording")?]);
    invoke(&env, &args, 4)?;
    if stub.finish()?.len() != 1 {
        return Err("fault seed fanout".into());
    }
    for entry in fs::read_dir(directory.join("chembl")).map_err(|e| e.to_string())? {
        let file = entry.map_err(|e| e.to_string())?.path();
        let mut record: Value =
            serde_json::from_slice(&fs::read(&file).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let body = match fault {
            "broken" => "{",
            "html" => "<html>verification</html>",
            _ => "{}",
        };
        let exchange = json!(
            {"status":fault.parse::<u16>().unwrap_or(200),"content_type":if fault=="html"{"text/html"}else{"application/json"},
            "delay_ms":if fault=="timeout"{30000}else{0},"retry_after_ms":null,"body":{"kind":"text","value":body}}
        );
        *record.get_mut("exchanges").ok_or("exchanges")? =
            json!([exchange.clone(), exchange.clone(), exchange]);
        fs::write(file, record.to_string()).map_err(|e| e.to_string())?;
    }
    let mut replay = words.clone();
    replay.extend(["--replay", directory.to_str().ok_or("recording")?]);
    let (value, bytes) = invoke_measured(&env, &replay, 4)?;
    failed(&value)?;
    compared(
        &format!("chembl/fault/{}/{fault}", surface(assay)),
        &env,
        &words,
        4,
        MEASURE,
        &directory,
        (value, bytes),
    )?;
    Ok(())
}
fn bad(assay: bool, fault: &str) -> Result<String, String> {
    let mut value: Value =
        serde_json::from_str(&fixture(if assay { "assays" } else { "CVCL_2119" })?)
            .map_err(|e| e.to_string())?;
    if fault == "next" {
        *value.pointer_mut("/page_meta/next").ok_or("native next")? = if assay {
            Value::Null
        } else {
            json!("/chembl/api/data/cell_line.json?cellosaurus_id=CVCL_2119&limit=5&offset=5")
        };
        return Ok(value.to_string());
    }
    if fault == "bad-page" {
        *value
            .pointer_mut("/page_meta/limit")
            .ok_or("native limit")? = json!(2);
        return Ok(value.to_string());
    }
    let row = value
        .pointer_mut(if assay { "/assays/0" } else { "/cell_lines/0" })
        .and_then(Value::as_object_mut)
        .ok_or("row")?;
    let field = if assay {
        "cell_chembl_id"
    } else {
        "cellosaurus_id"
    };
    match fault {
        "mismatch" => {
            row.insert(
                field.into(),
                json!(if assay { "CHEMBL3308378" } else { "CVCL_0004" }),
            );
        }
        "missing" => {
            row.remove(field);
        }
        _ => {
            row.insert(field.into(), json!(17));
        }
    }
    Ok(value.to_string())
}
fn admission(assay: bool, fault: &str) -> Result<(), String> {
    let bad = bad(assay, fault)?;
    let mut served = 0;
    let stub = Stub::for_source_status(
        "www.ebi.ac.uk",
        "application/json",
        move |request| {
            if request.contains("/status.json") {
                return fixture("status");
            }
            served += 1;
            if served == 1 {
                Ok(bad.clone())
            } else {
                reply(request)
            }
        },
        |_| 200,
    )?;
    let env = setup(&stub)?;
    let words = words(assay);
    let (value, bytes) = invoke_measured(&env, &words, 4)?;
    failed(&value)?;
    if value
        .get("problems")
        .and_then(Value::as_array)
        .is_none_or(Vec::is_empty)
    {
        return Err("identity rejection lost problem".into());
    }
    budget_phase(
        &format!("chembl/admission/{}/{fault}/failed", surface(assay)),
        bytes,
    )?;
    let (healthy, healthy_bytes) = invoke_measured(&env, &words, 0)?;
    path(&healthy, "/sources/0/state", json!("ok"))?;
    budget_phase(
        &format!("chembl/admission/{}/{fault}/healthy", surface(assay)),
        healthy_bytes,
    )?;
    if stub.finish()?.len() != 3 {
        return Err("rejected identity entered cache".into());
    }
    Ok(())
}
fn budget_phase(phase: &str, bytes: usize) -> Result<(), String> {
    measure_bytes(phase, "cli", bytes, MEASURE)
}
fn disabled(assay: bool) -> Result<(), String> {
    let stub = stub()?;
    let env = setup(&stub)?;
    let words = words(assay);
    let (_value, bytes) = invoke_measured(&env, &words, 0)?;
    budget_phase(&format!("chembl/disabled/{}/prime", surface(assay)), bytes)?;
    let mut args = words.clone();
    args.extend(["--disable-sources", "chembl"]);
    let (value, warm_bytes) = invoke_measured(&env, &args, 4)?;
    path(&value, "/sources/0/state", json!("skipped"))?;
    path(&value, "/sources/0/reason", json!("disabled"))?;
    if value.get("result").is_some() {
        return Err("disabled source supplied cached facts".into());
    }
    budget_phase(
        &format!("chembl/disabled/{}/warm", surface(assay)),
        warm_bytes,
    )?;
    if stub.finish()?.len() != 2 {
        return Err("disabled source called network".into());
    }
    Ok(())
}
fn release(assay: bool) -> Result<(), String> {
    let stub = Stub::for_source_status(
        "www.ebi.ac.uk",
        "application/json",
        |request| {
            if request.contains("/status.json") {
                Ok("{}".into())
            } else {
                reply(request)
            }
        },
        |request| {
            if request.contains("/status.json") {
                500
            } else {
                200
            }
        },
    )?;
    let env = setup(&stub)?;
    let value = parity(
        &format!("chembl/release/{}/500", surface(assay)),
        &env,
        &words(assay),
        0,
        MEASURE,
    )?;
    path(&value, "/status", json!("partial"))?;
    path(&value, "/sources/0/state", json!("failed"))?;
    if value.pointer("/result/release").is_some() || value.pointer("/result/fetched").is_none() {
        return Err("failed release lost labelled retrieval time".into());
    }
    path(&value, "/problems/0/field", json!("release"))?;
    stub.finish()?;
    Ok(())
}
fn zero() -> Result<(), String> {
    let stub = Stub::for_source_status(
        "www.ebi.ac.uk",
        "application/json",
        |request| {
            if request.contains("/status.json") {
                fixture("status")
            } else {
                Ok(json!({"assays":[],"page_meta":{"limit":1,"offset":0,"total_count":0,"next":null,"previous":null}}).to_string())
            }
        },
        |_| 200,
    )?;
    let env = setup(&stub)?;
    let value = parity(
        "chembl/synthetic/assay-zero",
        &env,
        &words(true),
        0,
        MEASURE,
    )?;
    path(&value, "/result/assay_count", json!(0))?;
    path(&value, "/status", json!("complete"))?;
    stub.finish()?;
    Ok(())
}
fn more() -> Result<(), String> {
    let stub = Stub::for_source_status(
        "www.ebi.ac.uk",
        "application/json",
        |request| {
            if request.contains("/status.json") {
                return fixture("status");
            }
            let mut value: Value =
                serde_json::from_str(&fixture("CVCL_2119")?).map_err(|e| e.to_string())?;
            let original = value.pointer("/cell_lines/0").ok_or("native row")?.clone();
            let mut rows = Vec::new();
            for index in 0..5 {
                let mut row = original.clone();
                *row.get_mut("cell_chembl_id").ok_or("cell ID")? =
                    json!(format!("CHEMBL{}", 3706573 + index));
                *row.get_mut("cell_id").ok_or("cell record")? = json!(5547 + index);
                rows.push(row);
            }
            *value.get_mut("cell_lines").ok_or("rows")? = json!(rows);
            *value.pointer_mut("/page_meta/total_count").ok_or("total")? = json!(6);
            *value.pointer_mut("/page_meta/next").ok_or("native next")? =
                json!("/chembl/api/data/cell_line.json?cellosaurus_id=CVCL_2119&limit=5&offset=5");
            Ok(value.to_string())
        },
        |_| 200,
    )?;
    let env = setup(&stub)?;
    let value = parity("chembl/synthetic/more", &env, &words(false), 0, MEASURE)?;
    path(&value, "/result/items/total", json!(6))?;
    path(&value, "/status", json!("partial"))?;
    path(
        &value,
        "/next/0/args",
        json!([
            "get",
            "cell-line",
            "CVCL_2119",
            "chembl",
            "--section-offset",
            "5"
        ]),
    )?;
    stub.finish()?;
    Ok(())
}
