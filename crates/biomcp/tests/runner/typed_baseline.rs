//! Compare raw single-tool wire replies with the verified unchanged product baseline.
use super::super::super::{
    launch::{self, Environment},
    mcp::request,
    process,
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, process::Stdio};

pub(super) fn check() -> Result<(), String> {
    let baseline = launch::typed_baseline()
        .ok_or("BIOMCP_TYPED_BASELINE is required for single-mode wire parity")?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let frames = [
        request(json!(1), "server/discover", json!({})),
        request(json!(2), "tools/list", json!({})),
        request(
            json!(3),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":["get","variant","BRAF V600E"]}}),
        ),
        request(
            json!(4),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":["search","trial","--condition","melanoma"]}}),
        ),
    ];
    let expected = exchange(
        &env,
        &baseline,
        &["serve".into(), "--offline".into()],
        &frames,
    )?;
    for mode in [
        vec!["serve".into(), "--offline".into()],
        vec![
            "serve".into(),
            "--offline".into(),
            "--tool-mode".into(),
            "single".into(),
        ],
    ] {
        let actual = exchange(
            &env,
            std::path::Path::new(env!("CARGO_BIN_EXE_biomcp")),
            &mode,
            &frames,
        )?;
        if actual != expected {
            return Err(format!(
                "single mode changed baseline wire bytes: frozen={expected:?}; current={actual:?}"
            ));
        }
    }
    invalid_id_witness()
}
fn exchange(
    env: &Environment,
    binary: &std::path::Path,
    args: &[String],
    frames: &[Value],
) -> Result<BTreeMap<String, String>, String> {
    let child = env
        .command(binary, args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let (status, stdout, stderr) = process::exchange(child, frames)?;
    if !status.success() || !stderr.is_empty() {
        return Err(format!(
            "baseline process failed: {}",
            String::from_utf8_lossy(&stderr)
        ));
    }
    String::from_utf8(stdout)
        .map_err(|e| e.to_string())?
        .lines()
        .map(|line| {
            let value: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
            Ok((
                value.get("id").ok_or("missing reply id")?.to_string(),
                line.to_owned(),
            ))
        })
        .collect()
}

// Diagnostic evidence only: any changed hint still needs a product ruling.
pub(super) fn invalid_id_witness() -> Result<(), String> {
    use std::io::Write;
    let baseline = launch::typed_baseline()
        .ok_or("BIOMCP_TYPED_BASELINE is required for single-mode wire parity")?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let frames = [request(
        json!(1),
        "tools/call",
        json!({"name":"biomcp",
        "arguments":{"args":["get","trial","lung cancer"]}}),
    )];
    let args = ["serve".into(), "--offline".into()];
    let mut frozen = None;
    for (label, binary) in [
        ("frozen-ed9c", baseline.as_path()),
        (
            "current-default-single",
            std::path::Path::new(env!("CARGO_BIN_EXE_biomcp")),
        ),
    ] {
        let replies = exchange(&env, binary, &args, &frames)?;
        if label == "frozen-ed9c" {
            frozen = Some(replies.clone());
        } else if frozen.as_ref() != Some(&replies) {
            for raw in replies.values() {
                writeln!(
                    std::io::stdout(),
                    "TYPED_INVALID_ID_RAW {label} mode=serve-offline isolated-config=default {raw}"
                )
                .map_err(|e| e.to_string())?;
            }
            return Err(
                "unapproved invalid-ID single-mode wire difference; PM ruling required".into(),
            );
        }
        for raw in replies.values() {
            writeln!(
                std::io::stdout(),
                "TYPED_INVALID_ID_RAW {label} mode=serve-offline isolated-config=default {raw}"
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
