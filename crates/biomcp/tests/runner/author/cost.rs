use super::*;
pub(super) fn surfaces(
    env: &Environment,
    phase: &str,
    args: &[String],
    globals: &[String],
    exit: i32,
) -> Result<Value, String> {
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
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
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
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/author-command-budgets.json");
    if path.is_file() {
        let budgets: std::collections::BTreeMap<String, usize> =
            serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        for (surface, size) in sizes.as_object().ok_or("size map")? {
            let name = format!("{phase} [{surface}]");
            let measured = budgets
                .get(&name)
                .ok_or_else(|| format!("missing measured cost {name}"))?;
            if size.as_u64().ok_or("size")? as usize > (measured * 115).div_ceil(100) {
                return Err(format!("{name}: exceeds measured fifteen-percent cap"));
            }
        }
    }
    Ok(value)
}
