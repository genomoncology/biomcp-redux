//! Feature markdown-view: each format/configuration branch runs independently.
use super::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
type Check = Result<(), Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug)]
enum Config {
    Absent,
    Default,
    Explicit,
}

pub(super) fn trials() -> Vec<Trial> {
    let mut trials = Vec::new();
    for (command, words, exit) in [
        ("valid", vec!["health", "--offline"], 0),
        ("missing-input", vec!["get", "gene", "--offline"], 2),
        (
            "typed-invalid",
            vec!["get", "trial", "not-a-trial", "--offline"],
            2,
        ),
    ] {
        for format in [None, Some("json"), Some("md")] {
            for config in [Config::Absent, Config::Default, Config::Explicit] {
                let words = words.clone();
                let name = format!(
                    "terminal-format/{command}/{}/{config:?}",
                    format.unwrap_or("absent")
                );
                trials.push(transcript_trial(name, move || {
                    check(&words, exit, format, config).map_err(|e| e.to_string().into())
                }));
            }
        }
    }
    for (name, words, exit) in [
        (
            "help-cutoff",
            vec!["get", "gene", "--help", "--format", "md"],
            0,
        ),
        (
            "help-cutoff-explicit-json",
            vec![
                "--format", "json", "get", "gene", "--help", "--format", "md",
            ],
            0,
        ),
        (
            "positional-format-word",
            vec!["get", "gene", "--offline", "--", "--format"],
            4,
        ),
        (
            "positional-format-value",
            vec!["get", "gene", "--offline", "--", "--format", "md"],
            4,
        ),
    ] {
        trials.push(transcript_trial(
            format!("terminal-format/{name}"),
            move || check(&words, exit, None, Config::Absent).map_err(|e| e.to_string().into()),
        ));
    }
    trials
}

fn check(words: &[&str], exit: i32, format: Option<&str>, config: Config) -> Check {
    let env = Environment::new()?;
    let mut args: Vec<String> = words.iter().map(|word| (*word).into()).collect();
    match config {
        Config::Absent => (),
        Config::Default => env.config(&json!({"format":"md"}))?,
        Config::Explicit => {
            env.config(&json!({"format":"json"}))?;
            let path = env.root().join("explicit-config.json");
            fs::write(&path, br#"{"format":"md"}"#)?;
            args.extend(["--config".into(), path.to_string_lossy().into()]);
        }
    }
    if let Some(format) = format {
        args.extend(["--format".into(), format.into()]);
    }
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &args).output()?;
    assert_eq!(
        output.status.code(),
        Some(exit),
        "{args:?}: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let markdown = format == Some("md") || format.is_none() && !matches!(config, Config::Absent);
    let value = serde_json::from_slice::<Value>(&output.stdout);
    if markdown {
        assert!(
            value.is_err(),
            "expected Markdown for {args:?}: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        assert!(
            output.stdout.starts_with(b"#"),
            "Output selection must preserve the command response contract"
        );
    } else {
        assert!(value?.get("schema").is_some(), "expected JSON for {args:?}");
    }
    Ok(())
}
