//! Feature drug-ema: first reads preserve known medicine dates and disclose unavailable referral dates.
use super::{
    configuration::{cached, invoke},
    first_read::{bodies, configured},
    launch::Environment,
    network_stub::Stub,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};

const GET: &[&str] = &[
    "get",
    "drug",
    "EMEA/H/C/005248",
    "regulatory",
    "--source",
    "ema",
];
const CAPTURE: &str = "drug/ema-dates";
const REFERRALS: &str = "/en/documents/report/referrals-output-json-report_en.json";

pub(super) fn trials() -> Vec<Trial> {
    vec![
        super::transcript::trial("drug-ema/first-read-fetches-referrals", || {
            live_read(200).map_err(Into::into)
        }),
        super::transcript::trial("drug-ema/first-read-reports-failed-referrals", || {
            live_read(403).map_err(Into::into)
        }),
        super::transcript::trial("drug-ema/cold-replay-reports-missing-referrals", || {
            replay_read().map_err(Into::into)
        }),
    ]
}

fn live_read(referral_status: u16) -> Result<(), String> {
    let recordings = bodies(&format!("{CAPTURE}/ema"))?;
    let stub = Stub::for_source_status(
        "www.ema.europa.eu",
        "application/json",
        move |request| {
            recordings
                .iter()
                .find(|(path, _)| request.contains(path))
                .map(|(_, body)| body.clone())
                .ok_or_else(|| format!("unexpected EMA request: {request}"))
        },
        move |request| {
            if request.contains(REFERRALS) {
                referral_status
            } else {
                200
            }
        },
    )?;
    let (env, stub) = configured(stub, json!({"source_data_max_age_days":1000000}))?;
    let reply = invoke(&env, GET.iter().map(|arg| (*arg).into()).collect(), 0)?;
    let requests = stub.finish()?;
    if !requests
        .iter()
        .any(|(_, request)| request.contains(REFERRALS))
    {
        return Err(format!(
            "cold read did not request the referrals file: {reply}"
        ));
    }
    if referral_status == 200 {
        if reply.get("status") != Some(&json!("complete"))
            || reply.pointer("/result/regulatory/data/records/0/dates/5/date")
                != Some(&json!("2026-05-29"))
        {
            return Err(format!(
                "cold read did not give the synced decision date: {reply}"
            ));
        }
        Ok(())
    } else {
        partial(&reply)
    }
}

fn replay_read() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"source_data_max_age_days":1000000}))
        .map_err(|e| e.to_string())?;
    let capture = env.root().join("recordings");
    super::response_cases::copy_capture(CAPTURE, &capture, |_, record| {
        if record["url"]
            .as_str()
            .is_some_and(|url| url.ends_with(REFERRALS))
        {
            record["exchanges"][0]["status"] = json!(403);
        }
    })
    .map_err(|e| e.to_string())?;
    let cache = env.root().join("case-cache");
    let sync = invoke(&env, cached(&["sync", "ema"], &cache, Some(&capture)), 4)?;
    if sync.pointer("/sources/0/reason") != Some(&json!("rejected")) {
        return Err(format!(
            "the replayed referrals request was not rejected: {sync}"
        ));
    }
    partial(&invoke(&env, cached(GET, &cache, Some(&capture)), 0)?)
}

fn partial(reply: &Value) -> Result<(), String> {
    let notices = reply
        .pointer("/sources/0/notices")
        .map(Value::to_string)
        .unwrap_or_default();
    if reply.get("status") != Some(&json!("partial"))
        || reply.pointer("/sources/0/source") != Some(&json!("ema"))
        || reply.pointer("/result/regulatory/data/records/0/dates/0/date")
            != Some(&json!("2026-05-28"))
        || !notices.contains("referrals file")
        || notices.contains("No European Commission decision date is reported")
    {
        return Err(format!(
            "incomplete referrals read did not preserve the medicine dates and name the missing file: {reply}"
        ));
    }
    Ok(())
}
