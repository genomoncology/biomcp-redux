//! Stored native author snapshots must survive a change of adapter representation.
use super::{Check, Value};
use crate::runner::launch::{Environment, arguments, fixtures};
use std::path::Path;

pub(super) fn check(previous: &Path) -> Check {
    for (capture, words) in [
        (
            "author/synthetic",
            vec!["search", "author", "Louis Williams"],
        ),
        (
            "author/papers",
            vec!["author", "papers", "semantic-scholar:123", "details"],
        ),
    ] {
        let env = Environment::new()?;
        let args: Vec<_> = words.into_iter().map(String::from).collect();
        let replay = fixtures().join("replies").join(capture);
        let before = command(
            &env,
            previous,
            &arguments(&args, &env.globals(Some(&replay), false)),
        )?;
        let pool = before
            .pointer("/page/pool/id")
            .and_then(Value::as_str)
            .ok_or("old CLI did not publish a pool")?;
        let mut offline = args;
        offline.extend(["--pool".into(), pool.into()]);
        let after = command(
            &env,
            Path::new(env!("CARGO_BIN_EXE_biomcp")),
            &arguments(&offline, &env.globals(None, true)),
        )?;
        for pointer in [
            "/result",
            "/page/pool/id",
            "/page/pool/created",
            "/page/pool/expires",
        ] {
            if before.pointer(pointer) != after.pointer(pointer) {
                return Err(
                    format!("Previous {capture} snapshot changed {pointer}: {after}").into(),
                );
            }
        }
        if after["status"] != "complete" {
            return Err(format!("Previous {capture} snapshot was refused: {after}").into());
        }
    }
    Ok(())
}
fn command(
    env: &Environment,
    program: &Path,
    args: &[String],
) -> Result<Value, Box<dyn std::error::Error>> {
    let output = env.command(program, args).output()?;
    if !output.status.success() {
        return Err(format!(
            "Snapshot command failed: {}",
            String::from_utf8_lossy(&output.stdout)
        )
        .into());
    }
    Ok(serde_json::from_slice(&output.stdout)?)
}
