//! Version negotiation over the actual HTTP server.
use super::{Check, Server, legacy};
use serde_json::json;

pub(super) fn check(server: &Server) -> Check {
    let base = "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\n";
    let host = server.address.to_string();
    for revision in ["2025-11-25", "2025-06-18", "2025-03-26"] {
        let explicit = format!("MCP-Protocol-Version: {revision}\r\n");
        for version_header in [explicit, String::new()] {
            let headers = format!("{base}{version_header}");
            let init = json!({"jsonrpc":"2.0","id":7,"method":"initialize","params":{
                "protocolVersion":revision,"capabilities":{},"clientInfo":{"name":"local-proof","version":"1"}}});
            let call = json!({"jsonrpc":"2.0","id":8,"method":"tools/call",
                "params":{"name":"biomcp","arguments":{"args":["help"]}}});
            for (request, pointer, expected) in [
                (init, "/result/protocolVersion", json!(revision)),
                (call, "/result/isError", json!(false)),
                (
                    legacy(9, "unavailable-method", json!({})),
                    "/error/code",
                    json!(-32601),
                ),
            ] {
                let fallback = version_header.is_empty() && pointer != "/result/protocolVersion";
                let transport_revision = if fallback { "2025-03-26" } else { revision };
                let body = request.to_string();
                let (status, reply, response) =
                    server.raw("POST", "/mcp", &host, &headers, &body)?;
                if status != 200
                    || reply.as_ref().and_then(|v| v.pointer(pointer)) != Some(&expected)
                    || !response
                        .contains(&format!("MCP-Protocol-Version: {transport_revision}\r\n"))
                    || response.to_ascii_lowercase().contains("session-id")
                {
                    return Err(format!("legacy {revision}: {status} {reply:?}"));
                }
            }
        }
    }
    let headers = format!("{base}MCP-Protocol-Version: 2024-11-05\r\n");
    let body = legacy(9, "tools/list", json!({})).to_string();
    let (status, reply, _) = server.raw("POST", "/mcp", &host, &headers, &body)?;
    if status != 400
        || reply.as_ref().and_then(|v| v.pointer("/error/code")) != Some(&json!(-32022))
    {
        return Err(format!("unsupported revision: {status} {reply:?}"));
    }
    november_2024_fallback(server, base)
}

fn november_2024_fallback(server: &Server, base: &str) -> Check {
    let init = legacy(
        10,
        "initialize",
        json!({
            "protocolVersion":"2024-11-05","capabilities":{},
            "clientInfo":{"name":"local-proof","version":"1"}
        }),
    )
    .to_string();
    let host = server.address.to_string();
    for version_header in [
        "",
        "MCP-Protocol-Version: 2024-11-05\r\n",
        "MCP-Protocol-Version: 2025-03-26\r\n",
    ] {
        let headers = format!("{base}{version_header}");
        let (status, reply, response) = server.raw("POST", "/mcp", &host, &headers, &init)?;
        if status != 200
            || reply
                .as_ref()
                .and_then(|reply| reply.pointer("/result/protocolVersion"))
                != Some(&json!("2025-11-25"))
            || !response.contains("MCP-Protocol-Version: 2025-11-25\r\n")
        {
            return Err(format!(
                "HTTP did not offer its legacy fallback: {status} {reply:?}"
            ));
        }
    }
    Ok(())
}
