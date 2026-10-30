use super::super::{configuration::invoke, network_stub::Stub};
use super::{Check, application, both, exact_setup, setup_status};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![
        transcript_trial(
            "drug-approvals/invalid-protocol-update-rejects-reply",
            || protocol_date().map_err(Into::into),
        ),
        transcript_trial(
            "drug-approvals/refuse-wrong-application-before-cache",
            || wrong().map_err(Into::into),
        ),
        transcript_trial("drug-approvals/disabled-before-io", || {
            disabled().map_err(Into::into)
        }),
        transcript_trial("drug-approvals/native-absence-is-source-local", || {
            absence().map_err(Into::into)
        }),
    ];
    for (name, status, body) in [
        ("broken", 200, "{"),
        (
            "error",
            200,
            r#"{"error":{"code":"NOT_FOUND","message":"No matches found!"}}"#,
        ),
        (
            "wrong-404",
            404,
            r#"{"error":{"code":"NOT_FOUND","message":"route missing"}}"#,
        ),
    ] {
        trials.push(transcript_trial(
            format!("drug-approvals/reject-{name}"),
            move || reject(status, body).map_err(Into::into),
        ));
    }
    trials
}
fn args() -> Vec<String> {
    [
        "get",
        "drug",
        "NDA021335",
        "approvals",
        "--source",
        "drugsfda",
    ]
    .map(str::to_owned)
    .to_vec()
}
fn protocol_date() -> Check {
    let (env, stub) = super::setup(
        json!({"meta":{"last_updated":"2024-02-30", "results":{"skip":0,"limit":1,"total":1}},
            "results":[application()]}),
        "application_number%3A%22NDA021335%22",
    )?;
    let value = both(&env, args(), 4)?;
    if value.get("result").is_some()
        || value.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
    {
        return Err(format!(
            "invalid protocol date became a usable application: {value}"
        ));
    }
    stub.finish()?;
    Ok(())
}
fn rejected(value: &Value) -> Check {
    super::budget(super::measure(value)?, (386, 275))?;
    if value.get("status") != Some(&json!("failed"))
        || value.get("result").is_some()
        || value.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
    {
        return Err(format!("bad native reply became source absence: {value}"));
    }
    Ok(())
}
fn reject(status: u16, body: &'static str) -> Check {
    let (env, stub) = setup_status(body.into(), "application_number%3A%22NDA021335%22", status)?;
    rejected(&both(&env, args(), 4)?)?;
    if stub.finish()?.len() != 2 {
        return Err("invalid native reply entered cache".into());
    }
    Ok(())
}
fn absence() -> Check {
    let (env, stub) = setup_status(
        r#"{"error":{"code":"NOT_FOUND","message":"No matches found!"}}"#.into(),
        "application_number%3A%22NDA021335%22",
        404,
    )?;
    let value = both(&env, args(), 1)?;
    super::budget(super::measure(&value)?, (308, 197))?;
    if value.get("status") != Some(&json!("absent"))
        || value.pointer("/sources/0/state") != Some(&json!("ok"))
    {
        return Err("native empty became source failure".into());
    }
    if stub.finish()?.len() != 2 {
        return Err("source absence request count changed".into());
    }
    Ok(())
}
fn disabled() -> Check {
    let (env, stub) = exact_setup()?;
    let mut request = vec!["--disable-sources".into(), "drugsfda".into()];
    request.extend(args());
    let value = both(&env, request, 4)?;
    super::budget(super::measure(&value)?, (311, 200))?;
    if value.pointer("/sources/0/reason") != Some(&json!("disabled")) || !stub.finish()?.is_empty()
    {
        return Err("disabled FDA source opened a connection".into());
    }
    Ok(())
}
fn wrong() -> Check {
    let mut wrong = application();
    *wrong
        .get_mut("application_number")
        .ok_or("application number")? = json!("NDA208573");
    let mut first = true;
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |_| {
            let row = if first {
                first = false;
                wrong.clone()
            } else {
                application()
            };
            Ok(
                json!({"meta":{"results":{"skip":0,"limit":1,"total":1}},"results":[row]})
                    .to_string(),
            )
        },
        |_| 200,
    )?;
    let env = super::environment(&stub)?;
    rejected(&invoke(&env, args(), 4)?)?;
    let value = both(&env, args(), 0)?;
    super::budget(super::measure(&value)?, (1095, 984))?;
    if value.pointer("/result/identity/id") != Some(&json!("NDA021335"))
        || stub.finish()?.len() != 2
    {
        return Err("wrong application poisoned the cache".into());
    }
    Ok(())
}
