//! The complete public CDC reply retains its original recorded bytes and media.
use super::{Check, words};
use crate::runner::configuration::invoke;
use crate::runner::launch::Environment;
use serde_json::{Value, json};

pub(super) fn run() -> Check {
    let original: Value = serde_json::from_str(include_str!(
        "../../../../../tests/replies/vaers/live-mmr/vaers/7afc738c58f75b7c824f50ed0651327ced1f0e4c26db7d5b684e46dec027c963.json"
    )).map_err(|e| e.to_string())?;
    // Mutations reuse the actual request and source document, including its identity checks.
    for fault in [
        "none",
        "truncated",
        "html",
        "deep",
        "oversized",
        "latin1",
        "status",
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let directory = env.root().join("replay/vaers");
        std::fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
        let mut record = original.clone();
        record["exchanges"][0]["delay_ms"] = json!(0);
        if fault == "status" {
            record["exchanges"][0]["status"] = json!(403);
        }
        let raw = record["exchanges"][0]["body"]["value"]
            .as_str()
            .ok_or("body")?;
        let body = match fault {
            "truncated" => raw[..raw.len() - 20].to_owned(),
            "html" => "<html><body>Access denied</body></html>".into(),
            "deep" => format!("{}{}{}", "<page>".repeat(70), raw, "</page>".repeat(70)),
            "oversized" => raw.replace(
                "<platform>prod</platform>",
                &format!("<platform>{}</platform>", "é".repeat(1024 * 1024)),
            ),
            "latin1" => raw.replace("MEASLES, MUMPS AND RUBELLA VIRUS VACCINE, LIVE", "café"),
            _ => raw.to_owned(),
        };
        if fault == "latin1" || fault == "oversized" {
            std::fs::write(
                directory.join("latin1.bin"),
                body.chars().map(|c| c as u8).collect::<Vec<_>>(),
            )
            .map_err(|e| e.to_string())?;
            record["exchanges"][0]["body"] = json!({"kind":"sidecar", "value":"latin1.bin"});
        } else {
            record["exchanges"][0]["body"]["value"] = json!(body);
        }
        std::fs::write(
            directory.join("7afc738c58f75b7c824f50ed0651327ced1f0e4c26db7d5b684e46dec027c963.json"),
            serde_json::to_vec(&record).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let mut args = env.globals(Some(&env.root().join("replay")), false);
        args.extend(words("MMR"));
        let valid = matches!(fault, "none" | "latin1");
        let value = invoke(&env, args.clone(), if valid { 0 } else { 4 })?;
        crate::runner::schema::Registry::embedded()?.validate(&value, &args, true)?;
        let command = args.iter().position(|v| v == "search").ok_or("command")?;
        if crate::runner::mcp::call_with_globals(&env, &args[command..], !valid, &args[..command])?
            != value
        {
            return Err(format!("CDC {fault} reply differs through MCP"));
        }
        if valid {
            if value.pointer("/result/vaccine/code") != Some(&json!("MMR")) {
                return Err(format!("complete CDC XML lost vaccine identity: {value}"));
            }
            if fault == "latin1" && value.pointer("/result/vaccine/label") != Some(&json!("café"))
            {
                return Err(format!("ISO-8859-1 label was not decoded: {value}"));
            }
        } else if value.pointer("/sources/0/reason")
            != Some(&json!(if fault == "status" {
                "rejected"
            } else {
                "bad_reply"
            }))
        {
            return Err(format!(
                "invalid CDC {fault} reply escaped refusal: {value}"
            ));
        }
    }
    Ok(())
}
