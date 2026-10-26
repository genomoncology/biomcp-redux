use std::io::Write;
use super::*;
pub(super) fn run(measure: bool) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for verb in ["search", "get"] {
        let words = ["help", verb, "cell-line"];
        let value = invoke(&env, &words, 0)?;
        let mcp = super::super::mcp::call_with_globals(&env, &words.map(String::from), false, &[])?;
        if value != mcp {
            return Err("help CLI/MCP mismatch".into());
        }
        for (surface, output) in [("cli", &value), ("mcp", &mcp)] {
            let phase = format!("help/{verb}");
            let bytes = serde_json::to_vec(output).map_err(|e| e.to_string())?.len()
                + usize::from(surface == "cli");
            let _ = writeln!(std::io::stderr(), "CELL_LINE_MEASURE {phase} [{surface}] {bytes}");
            if !measure {
                budget(&phase, surface, output)?;
            }
        }
        let output = env
            .command(
                env!("CARGO_BIN_EXE_biomcp"),
                &["help", verb, "cell-line", "--format", "md"].map(String::from),
            )
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err("Markdown help failed".into());
        }
        let markdown = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
        let _ = writeln!(std::io::stderr(), 
            "CELL_LINE_MEASURE help/{verb} [markdown] {}",
            markdown.len()
        );
        if !measure && markdown.len() > ceiling(&format!("help/{verb}"), "markdown")? {
            return Err("Markdown help exceeds measured cap".into());
        }
        for argument in value
            .pointer("/result/commands/0/arguments")
            .and_then(Value::as_array)
            .ok_or("arguments")?
        {
            let name = argument
                .get("name")
                .and_then(Value::as_str)
                .ok_or("argument name")?;
            if !markdown.contains(name) {
                return Err(format!("Markdown omitted {name}"));
            }
        }
    }
    Ok(())
}
