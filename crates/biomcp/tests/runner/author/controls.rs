//! Feature author-search: forbidden routes never reach provider or cache boundaries.
use super::super::configuration::invoke;
use super::*;
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
    let path = super::super::launch::fixtures().join("cases/author/retired.yaml");
    let case: super::super::Case =
        serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    for step in case.steps {
        let mut args = step.run.clone().ok_or("retirement command")?;
        args.extend(["--case-cache".into(), blocked.to_string_lossy().into()]);
        let value = invoke(&env, args.clone(), 2)?;
        let raw = serde_json::to_vec(&value).map_err(|e| e.to_string())?;
        super::super::assertions::check(&raw, &step, &super::super::assertions::prepare(&step)?)?;
        if mcp::call_with_globals(&env, &args, true, &[])? != value {
            return Err("author retirement CLI/MCP mismatch".into());
        }
    }
    for id in ["semantic-scholar:123", "orcid:0000-0002-4299-3018"] {
        let source = if id.starts_with("orcid:") {
            "orcid"
        } else {
            "semantic-scholar"
        };
        for selector in [source, "noncommercial"] {
            for words in [vec!["get", "author", id], vec!["author", "papers", id]] {
                let mut args: Vec<String> = words.into_iter().map(String::from).collect();
                args.extend([
                    "--disable-sources".into(),
                    selector.into(),
                    "--case-cache".into(),
                    blocked.to_string_lossy().into(),
                ]);
                let value = invoke(&env, args.clone(), 4)?;
                if value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
                    return Err(format!("disabled author touched cache: {value}"));
                }
                if mcp::call_with_globals(&env, &args, true, &[])? != value {
                    return Err("disabled author CLI/MCP mismatch".into());
                }
            }
        }
    }
    if listener.accept().is_ok() {
        return Err("forbidden author route opened provider connection".into());
    }
    Ok(())
}
