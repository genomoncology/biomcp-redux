//! Generated help retains usable flag descriptions and entity navigation in both tool modes.
use super::{launch::Environment, mcp};
use biomcp_types::{CellLineFilteredPage, CellLinePage, ResponseBody};
use serde_json::{Value, json};

pub(super) fn trial() -> libtest_mimic::Trial {
    super::transcript::trial("schema-discovery/help-contract", || {
        check().map_err(Into::into)
    })
}
fn check() -> Result<(), String> {
    complete_help()?;
    let mut failures = Vec::new();
    for mode in ["single", "typed"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        env.config(&json!({"mcp_tool_mode":mode}))
            .map_err(|e| e.to_string())?;
        for path in [
            "search trial",
            "search adverse-event",
            "drug",
            "search pgx",
            "health",
        ] {
            let (name, arguments) = if mode == "single" {
                let mut args = vec!["help"];
                args.extend(path.split_whitespace());
                ("biomcp", json!({"args":args}))
            } else {
                ("help", json!({"command":path}))
            };
            let replies = mcp::exchange_with_globals(
                &env,
                &[mcp::request(
                    json!(1),
                    "tools/call",
                    crate::runner::protocol::typed_catalog::entity(name, arguments),
                )],
                &["--offline".into()],
            )?;
            let value = replies
                .first()
                .and_then(|r| r.pointer("/result/structuredContent"))
                .ok_or("missing help reply")?;
            let commands = value.pointer("/result/commands").and_then(Value::as_array);
            let valid = match (path, commands) {
                ("drug", Some(commands)) => commands
                    .iter()
                    .any(|c| c.get("command") == Some(&json!(["search", "drug"]))),
                ("search trial" | "search adverse-event", Some(commands)) => descriptions(commands),
                ("search pgx", Some(commands)) => commands
                    .iter()
                    .flat_map(|c| {
                        c.get("arguments")
                            .and_then(Value::as_array)
                            .into_iter()
                            .flatten()
                    })
                    .any(|arg| {
                        arg.get("name") == Some(&json!("--source"))
                            && arg.get("required") == Some(&json!(true))
                    }),
                ("health", Some(commands)) => !commands.iter().any(|c| {
                    c.get("description")
                        .and_then(Value::as_str)
                        .is_some_and(|s| {
                            s.contains("timed out")
                                || s.contains("deadline exceeded")
                                || s.contains("refused this request")
                        })
                }),
                _ => false,
            };
            if !valid {
                failures.push(format!("{mode} help {path}: {value}"));
            }
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n"))
    }
}
fn descriptions(commands: &[Value]) -> bool {
    commands
        .first()
        .and_then(|c| c.get("arguments"))
        .and_then(Value::as_array)
        .is_some_and(|args| {
            args.iter().all(|arg| {
                arg.get("description")
                    .and_then(Value::as_str)
                    .is_some_and(|s| {
                        !s.is_empty()
                            && (arg
                                .get("choices")
                                .and_then(Value::as_array)
                                .is_none_or(|choices| choices.is_empty())
                                || arg.get("value_type") == Some(&json!("boolean"))
                                || s.contains("Listed choices accept any letter case"))
                            && (arg.get("name") != Some(&json!("--drug"))
                                || (s.contains("FAERS") && s.contains("repeat --drug")))
                    })
            })
        })
}

fn complete_help() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &["help".into()])
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err("complete help failed".into());
    }
    let help: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    let command = help
        .pointer("/result/commands")
        .and_then(Value::as_array)
        .and_then(|commands| commands.iter().find(|c| c["command"] == json!(["search", "cell-line"])))
        .ok_or("complete help omits cell-line search")?;
    for id in [CellLinePage::SCHEMA, CellLineFilteredPage::SCHEMA] {
        if !command["reply_schemas"]
            .as_array()
            .is_some_and(|schemas| schemas.contains(&json!(id)))
        {
            return Err(format!("complete help omits {id} for cell-line search"));
        }
    }
    Ok(())
}
