use super::super::{configuration::invoke, launch::Environment, network_stub::Stub};
use super::field;
use super::{Check, both, get_args, identity, page, record, setup};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::json;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![
        transcript_trial("drug-label/refuse-mismatched-set-before-cache", || {
            wrong_set().map_err(Into::into)
        }),
        transcript_trial("drug-label/disabled-before-label-request", || {
            disabled().map_err(Into::into)
        }),
    ];
    for (status, message) in [(404, "route missing"), (200, "No matches found!")] {
        trials.push(transcript_trial(
            format!("drug-label/refuse-nonnative-absence-{status}"),
            move || rejection(status, message).map_err(Into::into),
        ));
    }
    trials
}
fn rejected(value: &serde_json::Value) -> Check {
    if field(value, "status") != "failed"
        || value.get("result").is_some()
        || value.pointer("/sources/0/state") != Some(&json!("failed"))
        || value.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
    {
        return Err("invalid reply became healthy source absence".into());
    }
    Ok(())
}
fn wrong_set() -> Check {
    let id = identity(3)?;
    let other = identity(4)?;
    let expected = format!(
        "GET /drug/label.json?search=openfda.spl_set_id%3A%22{}%22&limit=1&sort=effective_time%3Adesc HTTP/1.1",
        field(&id, "set_id").as_str().ok_or("id")?
    );
    let mut count = 0;
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| {
            if request.lines().next() != Some(expected.as_str()) {
                return Err("altered identity request".into());
            }
            count += 1;
            Ok(page(record(if count == 1 { &other } else { &id })).to_string())
        },
        |_| 200,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    stub.retain(&env);
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let id = identity(3)?;
    let args = get_args(&id)?;
    rejected(&invoke(&env, args.clone(), 4)?)?;
    let value = both(&env, args, 0)?;
    if value.pointer("/result/header/id") != Some(field(&id, "set_id")) || stub.finish()?.len() != 2
    {
        return Err("unsafe identity entered cache".into());
    }
    Ok(())
}
fn rejection(status: u16, message: &'static str) -> Check {
    let (env, stub) = setup(
        json!({"error":{"code":"NOT_FOUND","message":message}}),
        "openfda.brand_name.exact%3A%22Tagrisso%22%20OR%20openfda.generic_name.exact%3A%22Tagrisso%22".into(),
        status,
    )?;
    let args = vec![
        "search",
        "drug",
        "Tagrisso",
        "--source",
        "openfda-label",
        "--limit",
        "1",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    rejected(&both(&env, args, 4)?)?;
    if stub.finish()?.len() != 2 {
        return Err("rejected response entered cache".into());
    }
    Ok(())
}
fn disabled() -> Check {
    let id = identity(3)?;
    let (env, stub) = super::get_setup(&id, record(&id))?;
    let mut args = vec!["--disable-sources".into(), "openfda-label".into()];
    args.extend(get_args(&id)?);
    args.extend([
        "label".into(),
        "--label-section".into(),
        "boxed_warning".into(),
    ]);
    let value = both(&env, args, 4)?;
    if value.pointer("/sources/0/state") != Some(&json!("skipped"))
        || value.pointer("/sources/0/reason") != Some(&json!("disabled"))
        || !stub.finish()?.is_empty()
    {
        return Err("source policy admitted a disabled label request".into());
    }
    Ok(())
}
