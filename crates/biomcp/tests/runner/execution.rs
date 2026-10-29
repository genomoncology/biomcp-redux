//! Execute canonical case steps and assert their actual emitted bytes.
use super::{Step, Value, assertions, launch, mcp, pathway_budgets, schema};
use std::io::Write;
pub(super) fn run_step(
    step: &Step,
    environment: &launch::Environment,
    expectations: &assertions::Expectations<'_>,
    registry: &schema::Registry,
    replay: Option<&std::path::Path>,
) -> Result<Value, String> {
    let globals = environment.globals(replay, step.offline);
    let cli = step
        .run
        .as_ref()
        .map(|args| {
            let command_args = launch::arguments(args, &globals);
            let output = environment
                .command(env!("CARGO_BIN_EXE_biomcp"), &command_args)
                .output()
                .map_err(|error| error.to_string())?;
            if output.status.code() != Some(step.exit) {
                return Err(format!(
                    "exit: expected {}, actual {:?}; command: {:?}; stderr: {}; stdout: {}",
                    step.exit,
                    output.status.code(),
                    args,
                    String::from_utf8_lossy(&output.stderr),
                    String::from_utf8_lossy(&output.stdout)
                        .chars()
                        .take(2000)
                        .collect::<String>()
                ));
            }
            if !output.stderr.is_empty() {
                return Err("error output: expected empty stderr".into());
            }
            let value = serde_json::from_slice::<Value>(&output.stdout)
                .map_err(|error| error.to_string())?;
            registry.validate(&value, args, output.status.success())?;
            super::reply_readback::check(&output.stdout, &value, false)?;
            pathway_budgets::help(&value, args, "cli")?;
            assertions::check(&output.stdout, step, expectations)?;
            if step.full_output_bytes {
                writeln!(
                    std::io::stderr(),
                    "RAW_OUTPUT cli {:?} {}",
                    args,
                    output.stdout.len()
                )
                .map_err(|e| e.to_string())?;
            }
            Ok(value)
        })
        .transpose()?;
    let mut answer = cli.clone();
    if let Some(args) = &step.mcp {
        let (result, text) =
            mcp::call_text_with_globals(environment, args, step.is_error, &globals)?;
        registry.validate(&result, args, !step.is_error)?;
        super::reply_readback::check(text.as_bytes(), &result, true)?;
        pathway_budgets::help(&result, args, "mcp")?;
        assertions::check(text.as_bytes(), step, expectations)?;
        if step.full_output_bytes {
            writeln!(
                std::io::stderr(),
                "RAW_OUTPUT mcp-text {:?} {}",
                args,
                text.len()
            )
            .map_err(|e| e.to_string())?;
        }
        answer = Some(result.clone());
        if let Some(cli) = cli.as_ref() {
            parity(cli, &result, args)?;
        }
    }
    if let Some(call) = &step.mcp_call {
        let value = super::research::call_checked(
            environment,
            &call.name,
            call.arguments.clone(),
            &globals,
            step.is_error,
        )?;
        assertions::check(
            &serde_json::to_vec(&value).map_err(|e| e.to_string())?,
            step,
            expectations,
        )?;
        answer = Some(value);
    }
    answer.ok_or_else(|| "step requires run, mcp or mcp_call".into())
}

fn parity(cli: &Value, result: &Value, args: &[String]) -> Result<(), String> {
    let mut comparable = result.clone();
    // Repeated explicit syncs may acquire the same source in different seconds.
    if args.first().is_some_and(|verb| verb == "sync")
        && let Some(acquired) = cli.pointer("/result/snapshot/acquired")
        && let Some(value) = comparable.pointer_mut("/result/snapshot/acquired")
    {
        *value = acquired.clone();
    }
    if let Some(diff) = assertions::difference(cli, &comparable, "") {
        return Err(format!("CLI/MCP parity: unequal results at {diff}"));
    }
    Ok(())
}
