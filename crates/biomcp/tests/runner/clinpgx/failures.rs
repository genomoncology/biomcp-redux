//! Source failures use synthetic HTTP and recorded deadlines, never live faults.
use super::*;
pub(super) fn trials() -> Vec<Trial> {
    let mut out = vec![];
    for name in [
        "429",
        "500",
        "broken",
        "status",
        "wrong-accession",
        "unknown-field",
    ] {
        out.push(Trial::test(
            format!("clinpgx-annotations/reject-{name}"),
            move || failure(name).map_err(Into::into),
        ));
    }
    out
}
fn failure(name: &'static str) -> Check {
    let shape = if name == "wrong-accession" {
        "annotation"
    } else {
        "clinical"
    };
    let mut body = native(shape)?;
    if name == "wrong-accession" {
        *body
            .pointer_mut("/data/accessionId")
            .ok_or("synthetic identity")? = json!("PA999999");
    }
    if name == "status" {
        *body.get_mut("status").ok_or("status")? = json!("error");
    }
    if name == "unknown-field" {
        body.pointer_mut("/data/0")
            .and_then(Value::as_object_mut)
            .ok_or("row")?
            .insert("unknown_field".into(), json!(true));
    }
    let stub = Stub::for_source_status(
        "api.clinpgx.org",
        "application/json",
        move |_| {
            Ok(if name == "broken" {
                "{".into()
            } else {
                body.to_string()
            })
        },
        move |_| match name {
            "429" => 429,
            "500" => 500,
            _ => 200,
        },
    )?;
    let env = environment(&stub)?;
    let record = env.root().join("record");
    let mut command = vec!["--record".into(), record.to_string_lossy().into()];
    command.extend(args(shape));
    let value = invoke(&env, command.clone(), 4)?;
    if value.get("result").is_some_and(|v| !v.is_null())
        || value.pointer("/sources/0/state") != Some(&json!("failed"))
        || value.pointer("/sources/0/terms").is_none()
    {
        return Err("source failure became a negative fact".into());
    }
    *command.get_mut(0).ok_or("record switch")? = "--replay".into();
    let fresh = Environment::new().map_err(|e| e.to_string())?;
    let replay = both(&fresh, command, 4)?;
    if replay.get("sources") != value.get("sources") {
        return Err("recorded failure changed".into());
    }
    let mut offline = vec!["--offline".into()];
    offline.extend(args(shape));
    if both(&env, offline, 4)?.pointer("/sources/0/reason") != Some(&json!("offline")) {
        return Err("rejected response entered cache".into());
    }
    stub.finish()?;
    Ok(())
}
