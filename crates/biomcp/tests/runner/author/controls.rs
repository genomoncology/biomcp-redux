//! Feature author-search: forbidden routes never reach provider or source-cache boundaries.
use super::super::configuration::invoke;
use super::{Environment, Value, Write, fs, json, mcp};
pub(super) fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    env.config(
        &json!({"proxy": format!("http://{}", listener.local_addr().map_err(|e| e.to_string())?)}),
    )
    .map_err(|e| e.to_string())?;
    let blocked = env.root().join("blocked-cache");
    fs::write(&blocked, b"not a directory").map_err(|e| e.to_string())?;
    let globals = vec![
        "--offline".into(),
        "--case-cache".into(),
        blocked.to_string_lossy().into(),
    ];
    retirements(&env, &globals)?;
    for text in ["", " "] {
        let args = vec!["search".into(), "author".into(), text.into()];
        let value = surfaces(&env, &args, 2, &globals)?;
        if value.pointer("/error/code") != Some(&json!("usage.arguments")) {
            return Err("empty author search crossed input boundary".into());
        }
    }
    disabled(&env)?;
    if listener.accept().is_ok() {
        return Err("forbidden author route opened provider connection".into());
    }
    Ok(())
}
fn retirements(env: &Environment, globals: &[String]) -> Result<(), String> {
    for name in ["author", "article"] {
        let path = super::super::launch::fixtures()
            .join(format!("native/case-inputs/{name}/retired.yaml"));
        let case: super::super::Case =
            serde_saphyr::from_str(&fs::read_to_string(path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        for step in case.steps {
            let args = step.run.as_ref().ok_or("retirement command")?;
            if name == "article" && !args.starts_with(&["article".into(), "authors".into()]) {
                continue;
            }
            let value = surfaces(env, args, 2, globals)?;
            let raw = serde_json::to_vec(&value).map_err(|e| e.to_string())?;
            writeln!(
                std::io::stderr(),
                "AUTHOR_RETIREMENT_COST {args:?} {}",
                raw.len() + 1
            )
            .map_err(|e| e.to_string())?;
            super::super::assertions::check(
                &raw,
                &step,
                &super::super::assertions::prepare(&step)?,
            )?;
        }
    }
    Ok(())
}
fn surfaces(
    env: &Environment,
    args: &[String],
    exit: i32,
    globals: &[String],
) -> Result<Value, String> {
    let mut cli = args.to_vec();
    cli.extend_from_slice(globals);
    let value = invoke(env, cli, exit)?;
    if mcp::call_with_globals(env, args, true, globals)? != value {
        return Err(format!("author CLI/MCP mismatch: {args:?}"));
    }
    Ok(value)
}
fn disabled(env: &Environment) -> Result<(), String> {
    for (source, id) in [
        ("semantic-scholar", "semantic-scholar:123"),
        ("orcid", "orcid:0000-0002-4299-3018"),
    ] {
        for selector in [source, "noncommercial"] {
            let mut commands = vec![vec!["get", "author", id], vec!["author", "papers", id]];
            if source == "semantic-scholar" {
                commands.push(vec!["search", "author", "name"]);
            }
            for words in commands {
                let mut args: Vec<String> = words.into_iter().map(String::from).collect();
                args.extend(["--disable-sources".into(), selector.into()]);
                let value = surfaces(env, &args, 4, &[])?;
                if value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
                    return Err(format!("disabled author touched source: {value}"));
                }
            }
        }
    }
    Ok(())
}
