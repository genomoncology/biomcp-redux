//! Feature drug-label: fallback continuation counts native rows before identity filtering.
use super::super::network_stub::Stub;
use super::{Check, both, environment, field, identity, record};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("drug-label/fallback-native-pagination", || {
        walk().map_err(Into::into)
    })]
}
fn walk() -> Check {
    let mut rejected = identity(0)?;
    *rejected.get_mut("brand").ok_or("brand")? = json!("Gleevec Extra");
    let first = identity(1)?;
    let second = identity(2)?;
    let mut accepted = [record(&first), record(&second)];
    for row in &mut accepted {
        *row.pointer_mut("/openfda/brand_name")
            .ok_or("brand array")? = json!(["Other name", "GLEEVEC"]);
    }
    let rows = [
        record(&rejected),
        record(&rejected),
        record(&rejected),
        accepted[0].clone(),
        accepted[1].clone(),
    ];
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| {
            let line = request.lines().next().ok_or("request")?;
            let exact = line.contains("openfda.brand_name.exact%3A%22gleevec%22");
            if exact {
                return Ok(
                    json!({"error":{"code":"NOT_FOUND","message":"No matches found!"}}).to_string(),
                );
            }
            let offset = if line == target(0) {
                0
            } else if line == target(2) {
                2
            } else if line == target(4) {
                4
            } else if line == target(6) {
                6
            } else {
                return Err(format!("unexpected fallback page: {line}"));
            };
            let results: Vec<Value> = rows.iter().skip(offset).take(2).cloned().collect();
            Ok(
                json!({"meta":{"results":{"skip":offset,"limit":2,"total":5}},"results":results})
                    .to_string(),
            )
        },
        |request| {
            if request.contains("openfda.brand_name.exact") {
                404
            } else {
                200
            }
        },
    )?;
    let (env, stub) = environment(stub)?;
    for (offset, returned, expected_id, next) in [
        (0, 0, None, Some(2)),
        (2, 1, Some(field(&first, "set_id")), Some(4)),
        (4, 1, Some(field(&second, "set_id")), None),
        (6, 0, None, None),
    ] {
        let args = [
            "search",
            "drug",
            "gleevec",
            "--source",
            "openfda-label",
            "--limit",
            "2",
            "--offset",
            &offset.to_string(),
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        let value = both(&env, args, if returned == 0 { 1 } else { 0 })?;
        if value.pointer("/result/total") != Some(&json!(5))
            || value.pointer("/page/offset") != Some(&json!(offset))
            || value.pointer("/page/returned") != Some(&json!(returned))
        {
            return Err(format!(
                "fallback invented a filtered total or lost page position: {value}"
            ));
        }
        if let Some(id) = expected_id {
            if value.pointer("/result/items/0/id") != Some(id) {
                return Err("fallback repeated or skipped an accepted native row".into());
            }
        } else if field(&value, "status") != "absent" {
            return Err("rejected page returned a label".into());
        }
        let continuation = value
            .get("next")
            .and_then(Value::as_array)
            .and_then(|steps| steps.iter().find(|step| field(step, "kind") == "page"));
        match (next, continuation) {
            (Some(offset), Some(step)) => {
                let args = field(step, "args").as_array().ok_or("next args")?;
                let index = args
                    .iter()
                    .position(|arg| arg == "--offset")
                    .ok_or("next offset")?;
                if args.get(index + 1) != Some(&json!(offset.to_string())) {
                    return Err(
                        "fallback continuation counted accepted rows instead of native rows".into(),
                    );
                }
            }
            (None, None) => {}
            _ => return Err("fallback lost or invented a continuation".into()),
        }
    }
    if stub.finish()?.len() != 12 {
        return Err("fallback page made extra requests or failed warm reuse".into());
    }
    Ok(())
}
fn target(offset: usize) -> String {
    let skip = if offset == 0 {
        String::new()
    } else {
        format!("&skip={offset}")
    };
    format!(
        "GET /drug/label.json?search=openfda.brand_name%3A%22gleevec%22&limit=2&sort=effective_time%3Adesc{skip} HTTP/1.1"
    )
}
