//! Installed typed replies preserve configured representations and the JSON-value cap.
use super::super::{
    http::Server,
    launch::{self, Environment},
    mcp::{exchange_wire, exchange_with_globals, http_wire, request},
};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    forms()?;
    cap()
}
fn forms() -> Result<(), String> {
    let capture = launch::fixtures().join("replies/trial/condition");
    let mut reference = None;
    for form in [
        None,
        Some("once"),
        Some("both"),
        Some("structured-only"),
        Some("text-only"),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let mut config = json!({"mcp_tool_mode":"typed"});
        if let Some(form) = form {
            config
                .as_object_mut()
                .ok_or("config object")?
                .insert("mcp_reply_form".into(), json!(form));
        }
        env.config(&config).map_err(|e| e.to_string())?;
        let frames = [
            request(
                json!(1),
                "tools/call",
                crate::runner::protocol::typed_catalog::entity(
                    "search_trial",
                    json!({"condition":["melanoma"]}),
                ),
            ),
            request(
                json!(2),
                "tools/call",
                crate::runner::protocol::typed_catalog::entity(
                    "get_trial",
                    json!({"input":["not-an-nct"]}),
                ),
            ),
            request(
                json!(3),
                "tools/call",
                crate::runner::protocol::typed_catalog::entity(
                    "get_trial",
                    json!({"input":["NCT07491445"],"section":["summary","eligibility"],"source":"ctgov"}),
                ),
            ),
        ];
        let globals = env.globals(Some(&capture), false);
        let replies = exchange_wire(&env, &frames, &globals, None)?;
        let values = frames
            .iter()
            .enumerate()
            .map(|(index, frame)| {
                let result = replies
                    .iter()
                    .find(|reply| reply.get("id") == frame.get("id"))
                    .and_then(|reply| reply.get("result"))
                    .ok_or("tool result missing")?;
                checked(result, form.unwrap_or("once"), index)
            })
            .collect::<Result<Vec<_>, String>>()?;
        http_forms(config, &capture, &frames, form.unwrap_or("once"))?;
        if let Some(reference) = &reference {
            if reference != &values {
                return Err("reply form changed typed JSON".into());
            }
        } else {
            reference = Some(values);
        }
    }
    Ok(())
}
fn http_forms(
    config: Value,
    capture: &std::path::Path,
    frames: &[Value],
    form: &str,
) -> Result<(), String> {
    let server = Server::configured_globals(
        "127.0.0.1:0",
        Some(config),
        Some(capture.to_owned()),
        None,
        None,
        &[],
    )?;
    for (index, frame) in frames.iter().enumerate() {
        let reply = http_wire(&server, frame, "")?;
        checked(
            reply.get("result").ok_or("HTTP result missing")?,
            form,
            index,
        )?;
    }
    Ok(())
}
fn checked(result: &Value, form: &str, index: usize) -> Result<Value, String> {
    let value = super::reply_form::check_content(result, form)?;
    if result.get("isError") != Some(&json!(index != 0)) {
        return Err("typed reply lost error status".into());
    }
    if index == 0 && value.get("status") != Some(&json!("complete")) {
        return Err(format!("recorded search did not succeed: {value}"));
    }
    if index == 2
        && value.get("next")
            != Some(&json!([
                {"kind":"configure","name":"trial","arguments":{"action":"get","get":{"id":["NCT07491445"],"section":["eligibility"]}}},
                {"kind":"configure","name":"help","arguments":{"command":"get trial"}}
            ]))
    {
        return Err(format!("typed reply lost ordered recovery steps: {value}"));
    }
    Ok(value)
}
fn cap() -> Result<(), String> {
    for (length, refused) in [(80_000, false), (151_000, true)] {
        let text = "x".repeat(length);
        let mut reference = None;
        for mode in ["single", "typed"] {
            let env = Environment::new().map_err(|e| e.to_string())?;
            env.config(&json!({"mcp_tool_mode":mode}))
                .map_err(|e| e.to_string())?;
            let recordings = super::super::remote_tools::recording(&env, &text)?;
            let params = if mode == "single" {
                json!({"name":"biomcp","arguments":{"args":["get","gene","BRAF"]}})
            } else {
                crate::runner::protocol::typed_catalog::entity(
                    "get_gene",
                    json!({"input":["BRAF"]}),
                )
            };
            let replies = exchange_with_globals(
                &env,
                &[request(json!(1), "tools/call", params)],
                &env.globals(Some(&recordings), false),
            )?;
            let value = replies
                .first()
                .and_then(|reply| reply.pointer("/result/structuredContent"))
                .ok_or("cap reply missing")?;
            let broad = value.pointer("/error/code") == Some(&json!("usage.too_broad"));
            if broad
                && mode == "typed"
                && value
                    .get("next")
                    .is_some_and(|next| next.to_string().contains("\"action\":\"search\""))
            {
                return Err("too-broad reply acquired a search suggestion".into());
            }
            if broad != refused {
                return Err(format!(
                    "{mode} cap differed at recorded summary length {length}: {}",
                    value.get("error").unwrap_or(&Value::Null)
                ));
            }
            if let Some(reference) = &reference {
                if super::typed_followups::data(reference) != super::typed_followups::data(value) {
                    return Err("single and typed cap replies differ".into());
                }
            } else {
                reference = Some(value.clone());
            }
        }
    }
    Ok(())
}
