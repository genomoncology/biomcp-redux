//! Schema output survives closed readers; Markdown omits an absent query.
use crate::runner::transcript::trial as transcript_trial;
use super::{launch::Environment, process::Process};
use libtest_mimic::Trial;
use std::{io::Read, process::Stdio};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("schema/closed-output-reader", || {
            closed_reader().map_err(Into::into)
        }),
        transcript_trial("markdown-view/absent-query", || {
            absent_query().map_err(Into::into)
        }),
    ]
}

fn closed_reader() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    // The preclosed case forces the error even when help fits a pipe buffer.
    // The second case models a reader consuming ten bytes before closing.
    for read_prefix in [false, true] {
        let (reader, writer) = std::io::pipe().map_err(|e| e.to_string())?;
        let reader = if read_prefix {
            Some(reader)
        } else {
            drop(reader);
            None
        };
        let child = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &["help".into()])
            .stdout(writer)
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        if let Some(mut reader) = reader {
            let mut prefix = [0; 10];
            reader.read_exact(&mut prefix).map_err(|e| e.to_string())?;
            drop(reader);
        }
        let (status, _, stderr) = Process::new(child)?.finish()?;
        if !status.success() || !stderr.is_empty() {
            return Err(format!(
                "closed output reader (prefix={read_prefix}): {status}; stderr={}",
                String::from_utf8_lossy(&stderr)
            ));
        }
    }
    Ok(())
}

fn absent_query() -> Result<(), String> {
    let output = Environment::new()
        .map_err(|e| e.to_string())?
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &["key".into(), "list".into(), "--format".into(), "md".into()],
        )
        .output()
        .map_err(|e| e.to_string())?;
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success()
        || !output.stderr.is_empty()
        || !text.starts_with("## Result\n")
        || text.contains("**query:**")
    {
        return Err(format!("key list must start with its result: {text}"));
    }
    Ok(())
}
