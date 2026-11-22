//! Compare the CLI and default MCP call against independently prepared replay fixtures.
use super::{assertions, case_data, launch, mcp, response_cases, run_sequence, schema, transcript};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::path::Path;

const WITNESSES: &[(&str, &str, usize)] = &[
    ("1118", "trial/skill-examples.yaml", 1),
    ("1129", "drug/indication.yaml", 1),
];

pub(super) fn trials() -> Vec<Trial> {
    let mut trials: Vec<_> = WITNESSES
        .iter()
        .map(|&(ticket, file, step)| {
            let name = file.trim_end_matches(".yaml");
            transcript::trial(
                format!("mcp-parity/recent/{ticket}/{name}/{step}"),
                move || recorded(file, step).map_err(Into::into),
            )
        })
        .collect();
    trials.extend([
        transcript::trial("mcp-parity/recent/skill-guide", || {
            guide().map_err(Into::into)
        }),
        transcript::trial("mcp-parity/recent/live", || live().map_err(Into::into))
            .with_ignored_flag(true),
    ]);
    trials
}

fn recorded(file: &str, step: usize) -> Result<(), String> {
    let path = launch::fixtures().join("cases").join(file);
    let case = case_data::read_case(&path)?;
    let cli = replay(&case, step, false)?;
    let mcp = replay(&case, step, true)?;
    compare(&cli, &mcp)
}

fn replay(case: &case_data::Case, step: usize, mcp: bool) -> Result<Value, String> {
    let env = launch::Environment::new().map_err(|error| error.to_string())?;
    env.case_config(case).map_err(|error| error.to_string())?;
    let registry = schema::Registry::embedded()?;
    let recordings = response_cases::replay(&env, case.replay.as_deref(), case.cache_acquired)?;
    let mut previous = None;
    let mut previous_step = None;
    for (index, mut current) in case.steps.iter().take(step).cloned().enumerate() {
        transcript::case_step(index);
        current.offline |= recordings.is_none();
        if index + 1 == step {
            let args = current
                .run
                .take()
                .ok_or("parity witness needs a CLI command")?;
            if current.follow.is_some() || current.walk.is_some() || current.stdout.is_some() {
                return Err("parity witness must be one JSON command".into());
            }
            current.mcp = None;
            if mcp {
                current.mcp = Some(args);
                current.is_error = current.exit >= 2;
            } else {
                current.run = Some(args);
            }
            current.expect.clear();
            current.max_bytes = None;
            current.max_tokens = None;
            current.starts_within.clear();
            current.key_order.clear();
        }
        previous = Some(run_sequence(
            &current,
            &env,
            &assertions::prepare(&current)?,
            registry.as_ref(),
            recordings.as_deref(),
            previous.as_ref(),
            previous_step,
        )?);
        previous_step = case.steps.get(index);
    }
    if step > case.steps.len() {
        return Err("parity witness step is outside its case".into());
    }
    previous.ok_or_else(|| "parity witness step must be positive".into())
}

fn guide() -> Result<(), String> {
    let env = launch::Environment::new().map_err(|error| error.to_string())?;
    let cli = invoke(&env, &["skill", "render"])?;
    for mode in ["single", "typed"] {
        let (tool, arguments) = if mode == "single" {
            ("biomcp", json!({"args":["help","skill","render"]}))
        } else {
            ("help", json!({"command":"skill render"}))
        };
        let replies = mcp::exchange_with_globals(
            &env,
            &[
                mcp::request(json!(1), "tools/list", json!({})),
                mcp::request(
                    json!(2),
                    "tools/call",
                    json!({"name":tool,"arguments":arguments}),
                ),
            ],
            &["--offline".into(), "--tool-mode".into(), mode.into()],
        )?;
        let actual = replies
            .iter()
            .find(|reply| reply["id"] == 2)
            .and_then(|reply| {
                reply.pointer("/result/structuredContent/result/commands/0/description")
            })
            .and_then(Value::as_str);
        if actual != Some(cli.trim_end()) {
            return Err(format!(
                "{mode} help skill render did not return the CLI guide: {actual:?}"
            ));
        }
        if !replies
            .iter()
            .find(|reply| reply["id"] == 1)
            .and_then(|reply| reply.pointer("/result/tools"))
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .any(|item| {
                item["name"] == tool
                    && item["description"]
                        .as_str()
                        .is_some_and(|text| text.contains("skill render"))
            })
        {
            return Err(format!(
                "{mode} tool catalog does not point to the agent guide"
            ));
        }
    }
    Ok(())
}

fn invoke(env: &launch::Environment, args: &[&str]) -> Result<String, String> {
    let output = env
        .command(
            Path::new(env!("CARGO_BIN_EXE_biomcp")),
            &args.iter().map(|word| (*word).into()).collect::<Vec<_>>(),
        )
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!("{args:?}: exit {}", output.status));
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn live() -> Result<(), String> {
    let cli = invoke(
        &launch::Environment::new().map_err(|error| error.to_string())?,
        &["get", "trial", "NCT03548987"],
    )?;
    let value = serde_json::from_str(&cli).map_err(|error| error.to_string())?;
    let env = launch::Environment::new().map_err(|error| error.to_string())?;
    let args = ["get", "trial", "NCT03548987"].map(str::to_owned);
    compare(&value, &mcp::call_with_globals(&env, &args, false, &[])?)
}

fn compare(cli: &Value, mcp: &Value) -> Result<(), String> {
    match assertions::difference(cli, mcp, "") {
        Some(diff) => Err(format!("CLI/MCP parity: unequal results at {diff}")),
        None => Ok(()),
    }
}
