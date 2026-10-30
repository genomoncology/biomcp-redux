//! Public aliases preserve formatted help and unknown commands offer safe executable recovery.
use super::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::Value;

type Check = Result<(), Box<dyn std::error::Error>>;

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        "schema-discovery/help-aliases-and-safe-recovery",
        || check().map_err(|error| error.to_string().into()),
    )]
}

fn check() -> Check {
    let env = Environment::new()?;
    for format in ["json", "md"] {
        let help = run(&env, &["--format", format, "help"])?;
        if help.len() > 4096 {
            return Err(format!("root {format} help exceeds 4 KB: {}", help.len()).into());
        }
        for alias in [vec![], vec!["-h"], vec!["--help"], vec!["help", "--help"]] {
            let mut args = vec!["--format", format];
            args.extend(alias);
            if run(&env, &args)? != help {
                return Err(format!("help alias changed {format} output: {args:?}").into());
            }
        }
        for path in [
            vec!["get", "gene"],
            vec!["version"],
            vec!["schema"],
            vec!["skill", "install"],
        ] {
            let mut alias = vec!["--format", format];
            alias.extend(path.iter().copied());
            alias.push("--help");
            let mut scoped = vec!["--format", format, "help"];
            scoped.extend(path);
            if run(&env, &alias)? != run(&env, &scoped)? {
                return Err("scoped help alias changed its content".into());
            }
        }
    }
    let plain = run(&env, &["help"])?;
    let value: Value = serde_json::from_slice(&plain)?;
    if value
        .pointer("/result/commands")
        .and_then(Value::as_array)
        .ok_or("missing help commands")?
        .iter()
        .any(|command| command.pointer("/command/0").and_then(Value::as_str) == Some("list"))
    {
        return Err("retired list remains in public help".into());
    }
    check_metadata(&env, &value)?;
    for args in [
        vec!["discover", "SENSITIVE-input-never-echo"],
        vec!["help", "frobnicate"],
        vec!["serve", "--bogus"],
    ] {
        let words: Vec<_> = args.iter().map(|word| (*word).to_owned()).collect();
        let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &words).output()?;
        let text = std::str::from_utf8(&output.stdout)?;
        let value: Value = serde_json::from_str(text)?;
        if output.status.code() != Some(2)
            || value.pointer("/error/code").and_then(Value::as_str) != Some("usage.unknown_command")
            || text.contains("SENSITIVE-input-never-echo")
            || !output.stderr.is_empty()
        {
            return Err("unknown command leaked input or lost its usage identity".into());
        }
        let next = value
            .pointer("/next/0/args")
            .and_then(Value::as_array)
            .ok_or("unknown command omitted executable recovery")?;
        let next: Vec<_> = next.iter().filter_map(Value::as_str).collect();
        run(&env, &next)?;
    }
    Ok(())
}

fn check_metadata(env: &Environment, value: &Value) -> Check {
    let commands = value
        .pointer("/result/commands")
        .and_then(Value::as_array)
        .ok_or("commands")?;
    for command in commands {
        let Some(arguments) = command.get("arguments") else {
            continue;
        };
        for argument in arguments.as_array().ok_or("arguments")? {
            if argument["value_type"].as_str().is_none_or(str::is_empty) {
                return Err("argument has no value type".into());
            }
        }
    }
    check_defaults(commands)?;
    let scoped: Value = serde_json::from_slice(&run(env, &["help", "get", "phenotype"])?)?;
    let phenotype = scoped
        .pointer("/result/commands/0")
        .ok_or("phenotype command")?;
    if !phenotype["arguments"]
        .as_array()
        .ok_or("arguments")?
        .iter()
        .any(|a| a["name"] == "--fresh")
    {
        return Err("next-step fresh option missing from scoped help".into());
    }
    let author = commands
        .iter()
        .find(|c| c["command"] == serde_json::json!(["search", "author"]))
        .ok_or("author")?;
    let limit = author["arguments"]
        .as_array()
        .ok_or("arguments")?
        .iter()
        .find(|a| a["name"] == "--limit")
        .ok_or("limit")?;
    if limit.get("default").is_none()
        || !limit["aliases"]
            .as_array()
            .is_some_and(|a| a.contains(&Value::from("-l")))
    {
        return Err("help omits default or alias".into());
    }
    check_rendered_topics(env)?;
    Ok(())
}

fn check_rendered_topics(env: &Environment) -> Check {
    let exit = run(env, &["help", "exit"])?;
    let exit: Value = serde_json::from_slice(&exit)?;
    let description = exit
        .pointer("/result/commands/0/description")
        .and_then(Value::as_str)
        .ok_or("exit description")?;
    for code in 0..=5 {
        if !description.contains(&format!("{code}:")) {
            return Err("exit topic omits a process code".into());
        }
    }
    let markdown = String::from_utf8(run(env, &["--format", "md", "help", "search", "author"])?)?;
    if !markdown.contains("# biomcp search author")
        || !markdown.contains("| Name | Description |")
        || !markdown.contains("```sh\nbiomcp search author")
    {
        return Err(
            "Markdown help lost its command heading, ordered columns or shell examples".into(),
        );
    }
    Ok(())
}

fn run(env: &Environment, args: &[&str]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let words: Vec<_> = args.iter().map(|word| (*word).to_owned()).collect();
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &words).output()?;
    if !output.status.success() {
        return Err(format!("help failed for {args:?}: {output:?}").into());
    }
    Ok(output.stdout)
}

// Reply query contracts expose the same declared defaults to outside callers.
fn check_defaults(commands: &[Value]) -> Check {
    for command in commands {
        for identity in command["reply_schemas"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
        {
            let Some((_, text)) = super::schema::ARTIFACTS
                .iter()
                .find(|(id, _)| *id == identity)
            else {
                continue;
            };
            let schema: Value = serde_json::from_str(text)?;
            let Some(mut query) = schema.pointer("/properties/query") else {
                continue;
            };
            if let Some(reference) = query["$ref"].as_str() {
                query = schema
                    .pointer(reference.trim_start_matches('#'))
                    .ok_or("query reference")?;
            }
            for (name, property) in query["properties"].as_object().into_iter().flatten() {
                let Some(default) = property.get("default") else {
                    continue;
                };
                let name = if property.get("x-positional").is_some() {
                    name.clone()
                } else {
                    format!("--{}", name.replace('_', "-"))
                };
                let Some(argument) = command["arguments"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .find(|a| a["name"] == name)
                else {
                    continue;
                };
                let expected = default
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| default.to_string());
                if argument["default"].as_str() != Some(&expected) {
                    return Err(format!("help default differs from {identity} query {name}").into());
                }
            }
        }
    }
    Ok(())
}
