//! Feature mcp-hosted: hosted HTTP requests declare use before accessing sources.
use super::{http::Server, request_keys::call};
use crate::runner::transcript::trial;
use libtest_mimic::Trial;
use serde_json::json;

pub(super) fn trials() -> Vec<Trial> {
    vec![trial("mcp-hosted/use/source-admission", || admission().map_err(Into::into))]
}
fn admission() -> Result<(), String> {
    let mut server = Server::configured_globals(
        "127.0.0.1:0", None, None, None, None, &["--hosted".into()],
    )?;
    let args = ["get", "pgx", "--gene", "DPYD", "--source", "clinpgx", "--offline"];
    for header in ["", "x-biomcp-use: commercial\r\n"] {
        let reply = call(&server, &args, header, false)?;
        if !reply.to_string().contains("disabled-source") {
            return Err(format!("commercial request admitted a noncommercial source: {reply}"));
        }
    }
    let academic = call(&server, &args, "x-biomcp-use: academic\r\n", false)?;
    if academic.to_string().contains("disabled-source") {
        return Err(format!("academic source was disabled: {academic}"));
    }
    let frame = json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"biomcp","arguments":{"args":args}}});
    let (status, reply, _) = server.raw("POST", "/mcp", &server.address.to_string(),
        "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2025-11-25\r\nx-biomcp-use: invalid\r\n", &frame.to_string())?;
    if status != 400 || !reply.is_some_and(|reply| {
        ["x-biomcp-use", "commercial", "academic"].iter().all(|word| reply.to_string().contains(word))
    }) {
        return Err("invalid declaration did not name header and allowed values".into());
    }
    server.stop()
}
