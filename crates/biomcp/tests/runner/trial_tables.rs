//! Registry failure and pre-request boundaries through the isolated real command.
#![expect(
    clippy::indexing_slicing,
    reason = "Checked captures fail directly when a fixture shape changes"
)]
use super::response_cases::{checked_case, checked_steps};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("trial-search/native-timestamp-and-query-echo", || {
            timestamp().map_err(Into::into)
        }),
        transcript_trial("trial-search/missing-study-array-is-not-an-empty-page", || {
            missing_studies().map_err(Into::into)
        }),
        transcript_trial("trial-search/gene-and-registry-share-command-budget", || {
            sequential_budget(false).map_err(Into::into)
        }),
        transcript_trial("trial-search/version-timeout-preserves-search-rows", || {
            sequential_budget(true).map_err(Into::into)
        }),
        transcript_trial("trial-search/count-failure-preserves-continuation-rows", || {
            count_failure().map_err(Into::into)
        }),
        transcript_trial("trial-search/disabled-cache-and-source-usage", || {
            policy().map_err(Into::into)
        }),
        transcript_trial("trial-search/source-failures-never-assert-zero", || {
            failures().map_err(Into::into)
        }),
    ]
}

fn missing_studies() -> Check {
    for continuation in [false, true] {
        let mut args = vec!["search", "trial", "--condition", "melanoma"];
        if continuation {
            args.extend(["--next-page", "ZVNj7o2Elu8o3lpzV8ikv62tmpOQJJxuZPGs2Pg"]);
        }
        let output = checked_steps(
            if continuation {
                "trial/walk"
            } else {
                "trial/condition"
            },
            |_, record| {
                if record["url"].as_str().is_some_and(|url| {
                    url.contains("/studies?") && (!continuation || url.contains("pageToken="))
                }) && let Some(body) = record["exchanges"][0]["body"]["value"].as_object_mut()
                {
                    body.remove("studies");
                }
            },
            json!([
                {"run":args,"exit":4,"expect":{"/status":"failed","/sources/0/reason":"bad-reply"}},
                {"run":args,"offline":true,"exit":4,"expect":{"/status":"failed","/sources/0/reason":"offline"}}
            ]),
        )?;
        if output.get("page").is_some() || output.get("result").is_some() {
            return Err("missing studies became a successful empty page".into());
        }
    }
    Ok(())
}

fn sequential_budget(version_timeout: bool) -> Check {
    checked_case(
        "trial/gene",
        &[
            "search",
            "trial",
            "--condition",
            "melanoma",
            "--gene",
            "BRAF",
        ],
        |source, record| {
            record["exchanges"][0]["delay_ms"] = json!(if source == "hgnc" {
                12000
            } else if record["url"]
                .as_str()
                .is_some_and(|url| url.contains("/studies?"))
            {
                if version_timeout { 4000 } else { 12000 }
            } else if version_timeout {
                6000
            } else {
                0
            });
        },
        if version_timeout {
            json!({"/status":"partial","/result/items":{"count":10},"/page/total":354,"/sources/1/reason":"deadline-exceeded"})
        } else {
            json!({"/status":"failed","/sources/0/state":"ok","/sources/1/reason":"deadline-exceeded"})
        },
        if version_timeout { 0 } else { 4 },
    )?;
    Ok(())
}
fn failures() -> Check {
    let args = ["search", "trial", "--condition", "melanoma"];
    for (kind, reason) in [
        ("429", "rate-limited"),
        ("500", "unavailable"),
        ("cut", "bad-reply"),
        ("error", "bad-reply"),
        ("deadline", "deadline-exceeded"),
    ] {
        let output = checked_steps(
            "trial/condition",
            |_, record| {
                if !record["url"]
                    .as_str()
                    .is_some_and(|url| url.contains("/studies?"))
                {
                    return;
                }
                let mut exchange = record["exchanges"][0].clone();
                exchange["delay_ms"] = json!(0);
                match kind {
                    "429" => {
                        exchange["status"] = json!(429);
                        exchange["retry_after_ms"] = json!(1);
                    }
                    "500" => exchange["status"] = json!(500),
                    "cut" => exchange["body"] = json!({"kind":"text","value":"{\"studies\":["}),
                    "error" => {
                        exchange["body"] =
                            json!({"kind":"json","value":{"error":"synthetic provider failure"}})
                    }
                    _ => exchange["delay_ms"] = json!(15001),
                }
                record["exchanges"] = json!(vec![
                    exchange;
                    if matches!(kind, "429" | "500") { 3 } else { 1 }
                ]);
            },
            json!([
                {"run":args,"exit":4,"expect":{"/status":"failed","/sources/0/source":"ctgov","/sources/0/reason":reason}},
                {"run":args,"offline":true,"exit":4,"expect":{"/status":"failed","/sources/0/reason":"offline"}}
            ]),
        )?;
        if output.get("page").is_some()
            || output.get("result").is_some()
            || output["sources"][0].get("count").is_some()
        {
            return Err(format!("{kind} invented a count or result").into());
        }
    }
    Ok(())
}

fn policy() -> Check {
    let args = ["search", "trial", "--condition", "melanoma"];
    let output = checked_steps(
        "trial/condition",
        |_, _| {},
        json!(
            [ {"run":args,"expect":{"/status":"complete","/page/total":3780}},{"run":["--disable-sources","ctgov","search","trial",
            "--condition","melanoma"],"exit":4,"expect":{"/status":"failed","/sources/0/state":"skipped","/sources/0/reason":"disabled"}} ]
        ),
    )?;
    if output.get("page").is_some() || output.get("result").is_some() {
        return Err("disabled cached source returned a result".into());
    }
    checked_case(
        "trial/condition",
        &args,
        |_, record| {
            if record["url"]
                .as_str()
                .is_some_and(|url| url.contains("/studies?"))
            {
                record["exchanges"][0]["status"] = json!(400);
                record["exchanges"][0]["body"] =
                    json!({"kind":"json","value":{"error":"Invalid status OPEN"}});
            }
        },
        json!({"/status":"usage","/error/code":"usage.arguments"}),
        2,
    )?;
    Ok(())
}

fn count_failure() -> Check {
    let first = checked_case(
        "trial/condition",
        &["search", "trial", "--condition", "melanoma"],
        |_, _| {},
        json!({"/page/total":3780}),
        0,
    )?;
    let mut args = vec!["search", "trial", "--condition", "melanoma", "--next-page"];
    let token = first
        .get("next")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|step| step.get("args").and_then(Value::as_array))
        .find_map(|args| args.windows(2).find(|pair| pair[0] == "--next-page"))
        .and_then(|pair| pair[1].as_str())
        .ok_or("missing source continuation")?;
    args.push(token);
    let output = checked_steps(
        "trial/walk",
        |_, record| {
            if record["url"]
                .as_str()
                .is_some_and(|url| url.contains("pageSize=1&"))
            {
                let mut exchange = record["exchanges"][0].clone();
                exchange["status"] = json!(500);
                record["exchanges"] = json!([exchange.clone(), exchange.clone(), exchange]);
            }
        },
        json!(
            [{"run":args,"mcp":args,"expect":{"/status":"partial","/result/items":{"count":10},"/sources/0/reason":"unavailable",
            } }]
        ),
    )?;
    if output.get("page").is_some() || output["sources"][0].get("count").is_some() {
        return Err("failed native count asserted a total".into());
    }
    Ok(())
}

fn timestamp() -> Check {
    for (native, timezone) in [
        ("2026-10-23T09:00:04", "source-gave-none"),
        ("2026-10-23T09:00:04Z", "in-value"),
        ("2026-10-23T09:00:04+03:00", "in-value"),
        ("2026-10-23T09:00:04-04:00", "in-value"),
    ] {
        let output = checked_case(
            "trial/condition",
            &["search", "trial", "--condition", "melanoma"],
            |_, record| {
                if let Some(value) = record.pointer_mut("/exchanges/0/body/value/dataTimestamp") {
                    *value = json!(native);
                }
            },
            json!({"/result/source_updated/value":native,
                "/result/source_updated/timezone":timezone,
                "/query/condition":["melanoma"],"/query/limit":10}),
            0,
        )?;
        if output["result"].get("request").is_some()
            || output["result"].get("count_request").is_some()
            || output["query"].get("has_results").is_some()
            || output["query"].get("count_only").is_some()
        {
            return Err("trial reply echoed provider parameters or unset false flags".into());
        }
    }
    Ok(())
}
