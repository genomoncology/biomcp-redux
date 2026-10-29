use super::{Environment, Value, Write, fs, json, mcp};
pub(super) fn surfaces(
    env: &Environment,
    phase: &str,
    args: &[String],
    globals: &[String],
    step: &super::super::Step,
) -> Result<Value, String> {
    let exit = step.exit;
    let expectations = super::super::assertions::prepare(step)?;
    let mut cli = args.to_vec();
    cli.extend_from_slice(globals);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &cli)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) || !output.stderr.is_empty() {
        return Err(format!(
            "{phase}: CLI exit {:?}; {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    super::super::assertions::check(&output.stdout, step, &expectations)?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    papers_total(phase, args, &value)?;
    absence(phase, &value)?;
    let retained = env.root().join("mcp-raw");
    fs::create_dir_all(&retained).map_err(|e| e.to_string())?;
    let mcp = mcp::call_retained(env, args, exit >= 2, globals, Some(&retained))?;
    if value != mcp {
        return Err(format!("{phase}: CLI/MCP parity differs"));
    }
    let rpc = fs::read(retained.join("mcp.stdout")).map_err(|e| e.to_string())?;
    let frame: Value = serde_json::from_slice(&rpc).map_err(|e| e.to_string())?;
    let text = frame
        .pointer("/result/content/0/text")
        .and_then(Value::as_str)
        .ok_or("MCP text")?;
    super::super::assertions::check(text.as_bytes(), step, &expectations)?;
    let registry = super::super::schema::Registry::embedded()?;
    registry.validate(&value, args, exit == 0)?;
    registry.validate(&mcp, args, exit == 0)?;
    let bare = serde_json::to_vec(value.get("result").unwrap_or(&Value::Null))
        .map_err(|e| e.to_string())?;
    cli.extend(["--format".into(), "md".into()]);
    let markdown = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &cli)
        .output()
        .map_err(|e| e.to_string())?;
    if markdown.status.code() != Some(exit) || markdown.stdout.is_empty() {
        return Err("Markdown output absent".into());
    }
    let sizes = json!({"cli":output.stdout.len(),"bare":bare.len(),"mcp-text":text.len(),"markdown":markdown.stdout.len(),"mcp-rpc":rpc.len()});
    writeln!(std::io::stderr(), "AUTHOR_COST {phase} {sizes}").map_err(|e| e.to_string())?;
    Ok(value)
}

fn papers_total(phase: &str, args: &[String], value: &Value) -> Result<(), String> {
    if args.first().is_some_and(|a| a == "author")
        && args.get(1).is_some_and(|a| a == "papers")
        && args.get(2).is_some_and(|a| {
            a.starts_with("semantic-scholar:") || a.starts_with("semanticscholar:")
        })
        && value.get("page").is_some()
    {
        return Err(format!("{phase}: native papers endpoint supplied no total"));
    }
    Ok(())
}

fn absence(phase: &str, value: &Value) -> Result<(), String> {
    if value.get("status") == Some(&json!("absent"))
        && value
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|steps| {
                steps
                    .iter()
                    .any(|step| step.get("kind") == Some(&json!("fresh")))
            })
    {
        return Err(format!("{phase}: source absence offered a failure retry"));
    }
    Ok(())
}
