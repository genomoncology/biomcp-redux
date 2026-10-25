use super::{Check, words};
use crate::runner::{configuration::invoke, launch::Environment};
use serde_json::json;
pub(super) fn run() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    env.config(
        &json!({"proxy":format!("http://{}",listener.local_addr().map_err(|e|e.to_string())?)}),
    )
    .map_err(|e| e.to_string())?;
    for text in ["FLU", "influenza", "Influenza vaccine", "iNfLuEnZa"] {
        let value = invoke(&env, words(text), 2)?;
        if value.pointer("/error/message")
            != Some(&json!(
                "Choose one exact native influenza vaccine code; tables remain separate and are never summed. One report may list several vaccines."
            ))
        {
            return Err(format!("wrong influenza guidance: {value}"));
        }
        let choices = value.pointer("/error/valid_values").and_then(|v| v.as_array())
            .ok_or("influenza choices absent")?;
        if choices.len() != 14 || choices.first() != Some(&json!("FLU(H1N1): INFLUENZA (H1N1) MONOVALENT (INJECTED)"))
            || !choices.contains(&json!("FLUX(SEASONAL): INFLUENZA VIRUS VACCINE, NO BRAND NAME"))
            || !choices.contains(&json!("FLUX(H1N1): INFLUENZA(H1N1) MONOVALENT, UNKNOWN MANUFACTURER")) {
            return Err(format!("native influenza choices changed: {value}"));
        }
        if crate::runner::mcp::call_with_globals(&env, &words(text), true, &[])? != value {
            return Err("influenza choices differ through MCP".into());
        }
    }
    for (flag, value) in [
        ("--reaction", "Synthetic"),
        ("--serious", "death"),
        ("--received-from", "2024-01-01"),
        ("--received-to", "2024-12-31"),
    ] {
        let mut args = words("MMR");
        args.extend([flag.into(), value.into()]);
        let value = invoke(&env, args, 2)?;
        if value
            .pointer("/error/message")
            .and_then(|v| v.as_str())
            .is_none_or(|v| !v.contains("--source faers"))
        {
            return Err(format!("VAERS accepted or obscured FAERS filter: {value}"));
        }
    }
    let args = ["get", "adverse-event", "1234", "--source", "vaers"]
        .map(str::to_owned)
        .to_vec();
    let value = invoke(&env, args, 2)?;
    if value
        .pointer("/error/message")
        .and_then(|v| v.as_str())
        .is_none_or(|v| !v.contains("no individual report get"))
    {
        return Err("VAERS exposed a report get".into());
    }
    if listener.accept().is_ok() {
        return Err("rejected VAERS input reached network".into());
    }
    Ok(())
}
