//! Project hosted discovery and recovery through the same execution authority.
use crate::command::Context;
use biomcp_types::{Help, Source};
use serde_json::{Value, json};
use std::{ffi::OsString, io};

pub(super) fn project(out: &mut crate::command::Outcome, cx: Context<'_>) -> io::Result<()> {
    if !cx.globals.hosted {
        return Ok(());
    }
    let value = out.value_mut()?;
    help(value, cx)?;
    next(value, cx);
    notices(value, cx)
}

fn help(value: &mut Value, cx: Context<'_>) -> io::Result<()> {
    if value.get("schema") == Some(&json!("biomcp.help/6")) {
        let mut help: Help = serde_json::from_value(value["result"].clone())?;
        let excluded = super::typed::catalog::excluded(cx)?;
        help.commands
            .retain(|command| offered(&command.command, cx));
        for command in &mut help.commands {
            if command.command.is_empty() {
                command.description = instructions(cx)?;
            }
            if command.command == ["help", "sources"] {
                command.description = sources(cx)?;
            }
            command.arguments.retain(|argument| {
                !matches!(
                    argument.name.as_str(),
                    "--fresh"
                        | "--config"
                        | "--source_pacing"
                        | "--save"
                        | "--replace"
                        | "--replay"
                        | "--record"
                        | "--case-cache"
                        | "--observe"
                )
            });
            if let [verb, entity] = command.command.as_slice() {
                command.sections.retain(|section| {
                    super::typed::catalog::section_allowed(verb, entity, &section.word, &excluded)
                });
            }
            for argument in &mut command.arguments {
                if matches!(
                    argument.name.trim_start_matches('-'),
                    "section" | "sections"
                ) && let [verb, entity] = command.command.as_slice()
                {
                    argument.choices.retain(|word| {
                        super::typed::catalog::section_allowed(verb, entity, word, &excluded)
                    });
                    if argument.default.as_ref().is_some_and(|word| {
                        !super::typed::catalog::section_allowed(verb, entity, word, &excluded)
                    }) {
                        argument.default = None;
                    }
                }
                argument.choices.retain(|word| {
                    !word
                        .parse::<Source>()
                        .is_ok_and(|source| excluded.contains(&source))
                });
                if argument.default.as_ref().is_some_and(|word| {
                    word.parse::<Source>()
                        .is_ok_and(|source| excluded.contains(&source))
                }) {
                    argument.default = None;
                }
            }
            command.examples.retain(|args| next_allowed(args, cx));
        }
        value["result"] = serde_json::to_value(help)?;
    }
    Ok(())
}

fn next(value: &mut Value, cx: Context<'_>) {
    if let Some(steps) = value.get_mut("next").and_then(Value::as_array_mut) {
        steps.retain_mut(|step| {
            let Some(args) = step.get_mut("args").and_then(Value::as_array_mut) else {
                return false;
            };
            // These are inherited server globals, never client instructions.
            let mut index = 0;
            while index < args.len() {
                if matches!(
                    args[index].as_str(),
                    Some("--config" | "--replay" | "--record" | "--case-cache" | "--observe")
                ) {
                    let end = (index + 2).min(args.len());
                    args.drain(index..end);
                } else {
                    index += 1;
                }
            }
            let words = args
                .iter()
                .filter_map(Value::as_str)
                .map(String::from)
                .collect::<Vec<_>>();
            next_allowed(&words, cx)
        });
        if steps.is_empty()
            && let Some(fields) = value.as_object_mut()
        {
            fields.remove("next");
        }
    }
}

fn notices(value: &mut Value, cx: Context<'_>) -> io::Result<()> {
    if let Some(rows) = value.get_mut("sources").and_then(Value::as_array_mut) {
        for row in rows {
            if row.get("reason") != Some(&json!("disabled")) {
                continue;
            }
            let source: Source = serde_json::from_value(row["source"].clone())?;
            let notice = serde_json::to_value(biomcp_types::SourceNotice::RequestGuidance {
                message: reason(source, cx)?.into(),
                retry_after_seconds: None,
            })?;
            if let Some(fields) = row.as_object_mut() {
                fields
                    .entry("notices")
                    .or_insert_with(|| json!([]))
                    .as_array_mut()
                    .ok_or_else(|| io::Error::other("source notices"))?
                    .push(notice);
            }
        }
    }
    Ok(())
}

pub(super) fn instructions(cx: Context<'_>) -> io::Result<String> {
    let excluded = super::typed::catalog::excluded(cx)?;
    let mut rows = vec!["Use the biomcp tool with command words in args; use help for arguments and help sources for this server's source policy.".to_owned()];
    for (verb, entity, id) in crate::grammar::command_paths() {
        let schema: Value = serde_json::from_str(
            crate::schema::find(id).ok_or_else(|| io::Error::other("query schema"))?,
        )?;
        if crate::command::mcp_policy::always_refused(&schema, true) {
            continue;
        }
        let sources = crate::help::coverage::command_sources(verb, entity, &schema)
            .into_iter()
            .filter(|word| {
                !word
                    .parse::<Source>()
                    .is_ok_and(|source| excluded.contains(&source))
            })
            .collect::<Vec<_>>()
            .join(", ");
        rows.push(format!("{verb} {entity}: {sources}"));
    }
    Ok(rows.join("\n"))
}

fn next_allowed(words: &[String], cx: Context<'_>) -> bool {
    let args = words.iter().map(OsString::from).collect::<Vec<_>>();
    crate::grammar::parse(cx.root, &args).is_ok_and(|mut parsed| {
        parsed.globals.inherit(cx.globals).is_ok()
            && parsed.action.is_some()
            && crate::command::mcp_policy::refusal(&parsed, true).is_none()
    })
}

fn offered(path: &[String], cx: Context<'_>) -> bool {
    if path.is_empty() {
        return true;
    }
    if crate::grammar::command_paths()
        .iter()
        .any(|(verb, entity, id)| {
            let command = [verb.to_string(), entity.to_string()];
            command.starts_with(path)
                && crate::schema::find(id)
                    .and_then(|text| serde_json::from_str::<Value>(text).ok())
                    .is_some_and(|schema| {
                        !crate::command::mcp_policy::always_refused(&schema, true)
                    })
        })
    {
        return true;
    }
    next_allowed(path, cx)
}

fn reason(source: Source, cx: Context<'_>) -> io::Result<String> {
    let operator = crate::config::excluded_sources(cx.globals, cx.settings)
        .map_err(|e| io::Error::other(e.0))?;
    let component_disabled = |hosted| {
        cx.fetch.as_ref().is_ok_and(|fetch| {
            let effective = if hosted {
                crate::command::hosted_use::restrict(
                    fetch.restrict_sources(Default::default()),
                    Some(cx),
                )
            } else {
                fetch.restrict_sources(Default::default())
            };
            biomcp_query::sources::components()
                .any(|component| component.source == source && component.disabled(&effective))
        })
    };
    let explanation = if operator.contains(&source) || component_disabled(false) {
        "The server operator disabled this source or a required component.".to_owned()
    } else if crate::command::hosted_use::disabled(cx.declared_use, cx.secrets).contains(&source)
        || component_disabled(true)
    {
        let ceiling =
            crate::config::hosted_max_use(cx.settings).map_err(|e| io::Error::other(e.0))?;
        let effective = cx
            .declared_use
            .unwrap_or(crate::command::DeclaredUse::Academic);
        format!(
            "This source or a required component is disabled by the requested hosted use policy (permitted use: {}). The server ceiling permits {} use.",
            effective.allowed(),
            ceiling.allowed()
        )
    } else {
        "This request disabled the source or a required component.".to_owned()
    };
    Ok(format!(
        "{explanation} A self-hosted install can enable it subject to the publisher's terms."
    ))
}

fn sources(cx: Context<'_>) -> io::Result<String> {
    let excluded = super::typed::catalog::excluded(cx)?;
    let mut rows = Vec::new();
    for spec in biomcp_query::sources::sources() {
        let source = spec.source;
        rows.push(if excluded.contains(&source) {
            format!("{source}: {}", reason(source, cx)?)
        } else { format!("{source}: served by this server; publisher terms and credential requirements apply.") });
    }
    Ok(rows.join("\n"))
}
