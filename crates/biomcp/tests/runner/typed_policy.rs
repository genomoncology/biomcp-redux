//! Both transports retain hosted policy while typed errors name executable next calls.
use super::super::super::{
    http::Server,
    launch::Environment,
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"mcp_tool_mode":"typed","hosted_max_use":"commercial"}))
        .map_err(|e| e.to_string())?;
    hosted(&env)?;
    boundaries(&env)
}
fn hosted(env: &Environment) -> Result<(), String> {
    let frames = [
        request(
            json!(1),
            "tools/call",
            json!({"name":"get_variant","arguments":{"input":"BRAF V600E"}}),
        ),
        request(
            json!(2),
            "tools/call",
            json!({"name":"get_trial","arguments":{"input":["melanoma"]}}),
        ),
        request(
            json!(3),
            "tools/call",
            json!({"name":"article_pdf","arguments":{"input":"28944238","save":"saved.pdf"}}),
        ),
    ];
    let replies = exchange_with_globals(env, &frames, &["--offline".into(), "--hosted".into()])?;
    let server = Server::configured_globals(
        "127.0.0.1:0",
        Some(json!({"mcp_tool_mode":"typed","hosted_max_use":"commercial"})),
        None,
        None,
        None,
        &["--offline".into(), "--hosted".into()],
    )?;
    for (index, frame) in frames.iter().enumerate() {
        let stdio = replies
            .iter()
            .find(|r| r["id"] == frame["id"])
            .ok_or("stdio reply")?;
        let (status, http) = server.request("POST", "/mcp", &frame.to_string())?;
        if status != 200
            || stdio.pointer("/result/structuredContent")
                != http.pointer("/result/structuredContent")
        {
            return Err("typed hosted transport mismatch".into());
        }
        let text = stdio
            .pointer("/result/content/0/text")
            .and_then(Value::as_str)
            .ok_or("summary")?;
        let expected = match index {
            0 => "search_variant",
            1 => "search_trial",
            _ => "hosted",
        };
        if index < 2 && !text.contains(expected) {
            return Err(format!("missing typed replacement {expected}: {text}"));
        }
        if index == 2
            && !stdio
                .pointer("/result/structuredContent")
                .ok_or("structured reply")?
                .to_string()
                .contains(expected)
        {
            return Err("typed save escaped hosted refusal".into());
        }
    }
    Ok(())
}
fn boundaries(env: &Environment) -> Result<(), String> {
    let hostile = request(
        json!(8),
        "tools/call",
        json!({"name":"get_gene","arguments":{"input":["--help"]}}),
    );
    let reply = exchange_with_globals(env, &[hostile], &["--offline".into()])?;
    if reply
        .first()
        .ok_or("missing reply")?
        .pointer("/result/structuredContent/schema")
        == Some(&json!("biomcp.help/6"))
    {
        return Err("literal input became a CLI flag".into());
    }
    // A command-line mode explicitly overrides the configured mode.
    let reply = exchange_with_globals(
        env,
        &[request(json!(4), "tools/list", json!({}))],
        &["--tool-mode".into(), "single".into(), "--offline".into()],
    )?;
    if reply
        .first()
        .ok_or("missing reply")?
        .pointer("/result/tools/0/name")
        != Some(&json!("biomcp"))
    {
        return Err("flag did not override config".into());
    }
    Ok(())
}
