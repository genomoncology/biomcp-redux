use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    check_case(body, 3)?;
    check_case(labels_only, 0)
}

fn check_case(reply: fn(&str) -> Result<String, String>, count: usize) -> Result<(), String> {
    let (value, _) = super::call(
        "myvariant.info",
        reply,
        &[
            "get",
            "variant",
            "chr7:g.140453136A>T",
            "predict",
            "--build",
            "GRCh37",
        ],
    )?;
    let data = value
        .pointer("/result/candidates/0/sections/predict/data")
        .ok_or_else(|| format!("prediction data missing: {value}"))?;
    if data.get("context")
        != Some(&json!([
            {"field":"cadd.sift.cat","values":["synthetic_array_label"]},
            {"field":"dbnsfp.polyphen2.hdiv.pred","values":["synthetic_label_without_score"]}
        ]))
    {
        return Err(format!("unpaired native labels were lost: {data}"));
    }
    let scores = data
        .get("scores")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
    if scores.len() != count || scores.iter().any(|score| score.get("label").is_some()) {
        return Err(format!(
            "labels were assigned to unproven numeric positions: {data}"
        ));
    }
    Ok(())
}

fn labels_only(request: &str) -> Result<String, String> {
    let mut value: Value = serde_json::from_str(&body(request)?).map_err(|e| e.to_string())?;
    if request.contains("cadd.phred") {
        value
            .pointer_mut("/cadd/sift")
            .and_then(Value::as_object_mut)
            .ok_or("synthetic sift object missing")?
            .remove("val");
        value
            .get_mut("dbnsfp")
            .and_then(Value::as_object_mut)
            .ok_or("synthetic dbnsfp object missing")?
            .remove("revel");
    }
    Ok(value.to_string())
}

fn body(request: &str) -> Result<String, String> {
    if request.contains("cadd.phred") {
        Ok(json!({"_id":"chr7:g.140453136A>T",
            "cadd":{"sift":{"val":[0,1],"cat":"synthetic_array_label"}},
            "dbnsfp":{"polyphen2":{"hdiv":{"pred":"synthetic_label_without_score"}},"revel":{"score":0.2}}
        })
        .to_string())
    } else {
        super::component_body(request)
    }
}
