//! Feature maude-reports: native search selectors and source boundaries through CLI and MCP.
use super::super::{configuration::invoke, launch::Environment, mcp, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("maude-reports/native-filters-and-replay", || {
            selectors().map_err(Into::into)
        }),
        transcript_trial("maude-reports/filter-source-boundaries", || {
            refusals().map_err(Into::into)
        }),
    ]
}
fn both(
    env: &Environment,
    args: Vec<String>,
    exit: i32,
    globals: &[String],
) -> Result<Value, String> {
    let cli = invoke(env, super::super::launch::arguments(&args, globals), exit)?;
    let tool =
        mcp::call_retained(env, &args, exit >= 2, globals, Some(env.root())).map_err(|error| {
            format!(
                "{args:?}: {error}; raw MCP: {}",
                std::fs::read_to_string(env.root().join("mcp.stdout")).unwrap_or_default()
            )
        })?;
    if cli != tool {
        return Err(format!("CLI/MCP differ: {cli} / {tool}"));
    }
    Ok(cli)
}
fn selectors() -> Result<(), String> {
    for (flags, expression, absent) in CASES {
        let expected = expression.to_string();
        let request = format!(
            "GET /device/event.json?search={}&limit=1&skip=0&sort=date_received%3Adesc HTTP/1.1",
            encode(expression)
        );
        let empty = *absent;
        let exit = i32::from(empty);
        let stub = Stub::for_source("api.fda.gov", "application/json", move |req| {
            if req.lines().next() != Some(request.as_str()) {
                return Err(format!("wrong native query: {req}"));
            }
            Ok(if empty {
                json!({"meta":{"results":{"skip":0,"limit":1,"total":0}},"results":[]}).to_string()
            } else {
                super::response(super::row())
            })
        })?;
        let env = super::environment(&stub)?;
        let recording = env.root().join("recording");
        let mut args = vec![
            "search".into(),
            "adverse-event".into(),
            "--limit".into(),
            "1".into(),
        ];
        args.extend(flags.iter().map(|s| (*s).to_owned()));
        args.extend(["--source".into(), "maude".into()]);
        let globals = ["--record".into(), recording.to_string_lossy().into_owned()];
        let value = invoke(&env, super::super::launch::arguments(&args, &globals), exit)?;
        if value.pointer("/result/source_query") != Some(&json!(expected))
            || value.pointer("/page/total") != Some(&json!(if empty { 0 } else { 1 }))
        {
            return Err(format!("native selector or count lost: {value}"));
        }
        let replay = Environment::new().map_err(|e| e.to_string())?;
        let globals = replay.globals(Some(&recording), false);
        let replayed = both(&replay, args, exit, &globals)?;
        if replayed.get("result") != value.get("result") {
            return Err("replay changed result".into());
        }
        if stub.finish()?.len() != 1 {
            return Err("filters opened extra source requests".into());
        }
    }
    Ok(())
}
fn encode(value: &str) -> String {
    value
        .bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                char::from(b).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn refusals() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for source in ["faers", "vaers"] {
        for flag in ["--manufacturer", "--product-code", "--problem"] {
            for first in [true, false] {
                let source_args = ["--source", source];
                let filter = [flag, "Synthetic"];
                let mut args = vec![
                    "--offline".to_owned(),
                    "search".into(),
                    "adverse-event".into(),
                    "MMR".into(),
                ];
                for pair in if first {
                    [source_args, filter]
                } else {
                    [filter, source_args]
                } {
                    args.extend(pair.map(str::to_owned));
                }
                let value = both(&env, args, 2, &[])?;
                if value.pointer("/error/code") != Some(&json!("usage.retired"))
                    || value.pointer("/error/message")
                        != Some(&json!("this adverse-event filter is not supported"))
                {
                    return Err(format!("{source} accepted {flag}: {value}"));
                }
            }
        }
    }
    for source in ["faers", "maude", "vaers"] {
        let value = both(
            &env,
            ["--offline", "search", "adverse-event", "--source", source]
                .map(str::to_owned)
                .to_vec(),
            2,
            &[],
        )?;
        if value.get("error").is_none() {
            return Err(format!("unfiltered search accepted: {value}"));
        }
    }
    Ok(())
}
const CASES: &[(&[&str], &str, bool)] = &[
    (
        &["--manufacturer", "Synthetic maker"],
        "device.manufacturer_d_name:\"Synthetic maker\"",
        false,
    ),
    (
        &["--product-code", "ABC"],
        "device.device_report_product_code.exact:\"ABC\"",
        false,
    ),
    (
        &["--problem", "False Positive Result"],
        "product_problems:\"False Positive Result\"",
        false,
    ),
    (
        &["--product-code", "ABC", "--product-code", "XYZ"],
        "(device.device_report_product_code.exact:\"ABC\" OR device.device_report_product_code.exact:\"XYZ\")",
        false,
    ),
    (
        &[
            "Synthetic device",
            "--manufacturer",
            "Synthetic maker",
            "--product-code",
            "ABC",
            "--problem",
            "False Positive Result",
        ],
        concat!(
            "device.generic_name:\"Synthetic device\" AND ",
            "device.manufacturer_d_name:\"Synthetic maker\" AND ",
            "device.device_report_product_code.exact:\"ABC\" AND ",
            "product_problems:\"False Positive Result\""
        ),
        false,
    ),
    (
        &["--product-code", "UNKNOWNCODE"],
        "device.device_report_product_code.exact:\"UNKNOWNCODE\"",
        true,
    ),
];
