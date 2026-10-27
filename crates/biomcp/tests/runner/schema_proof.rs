use super::schema::Registry;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::sync::Arc;

pub(super) fn trials(registry: Arc<Registry>) -> Vec<Trial> {
    let raw_registry = Arc::clone(&registry);
    vec![
        Trial::test("schema/catalog-and-help-reply-discovery", || {
            discovery().map_err(Into::into)
        }),
        Trial::test("runner/validates-results-before-assertions", move || {
            results(&registry).map_err(Into::into)
        }),
        Trial::test("runner/validates-raw-artifact-request", move || {
            raw_artifacts(&raw_registry).map_err(Into::into)
        }),
        Trial::test("runner/resolves-only-registered-schema-references", || {
            references().map_err(Into::into)
        }),
    ]
}

fn results(registry: &Registry) -> Result<(), String> {
    let good = json!({"schema":"biomcp.error/1", "status":"usage",
        "error":{"code":"usage.schema","message":"use biomcp schema to list known artifact IDs"}});
    let args = vec!["unknown".into()];
    registry.validate(&good, &args, false)?;
    let mut rows = vec![
        json!({}),
        json!({"schema":42}),
        json!({"schema":"missing/1"}),
        json!({"$id":"invented/1"}),
    ];
    let mut bad_code = good.clone();
    *bad_code.pointer_mut("/error/code").ok_or("missing code")? = json!("bad");
    rows.push(bad_code);
    let mut missing = good;
    missing
        .as_object_mut()
        .ok_or("not object")?
        .remove("status");
    rows.push(missing);
    for value in rows {
        rejects(registry, &value, &args, false)?;
    }
    let long_id = json!({"schema": "🚫".repeat(10_000)});
    let diagnostic = registry
        .validate(&long_id, &args, false)
        .err()
        .ok_or("long unknown identity accepted")?;
    if diagnostic.chars().count() > 240 {
        return Err("unbounded schema identity diagnostic".into());
    }
    Ok(())
}

fn raw_artifacts(registry: &Registry) -> Result<(), String> {
    let artifact = |id: &str| -> Result<Value, String> {
        let (_, text) = super::schema::ARTIFACTS
            .iter()
            .find(|(name, _)| *name == id)
            .ok_or("missing artifact")?;
        serde_json::from_str(text).map_err(|error| error.to_string())
    };
    let source = artifact("biodata.source/1")?;
    let args = vec!["schema".into(), "biodata.source/1".into()];
    registry.validate(&source, &args, true)?;
    rejects(registry, &source, &args, false)?;
    rejects(registry, &source, &["unknown".into()], true)?;
    rejects(
        registry,
        &source,
        &["schema".into(), "missing/1".into()],
        true,
    )?;
    rejects(registry, &artifact("biomcp.error/1")?, &args, true)?;
    let mut changed = source.clone();
    changed
        .as_object_mut()
        .ok_or("not object")?
        .insert("description".into(), json!("changed"));
    rejects(registry, &changed, &args, true)?;
    let mut wrong_id = source;
    wrong_id
        .as_object_mut()
        .ok_or("not object")?
        .insert("$id".into(), json!("invented/1"));
    rejects(registry, &wrong_id, &args, true)
}

fn references() -> Result<(), String> {
    let parent = json!({"$id":"fixture.parent/1", "type":"object", "required":["schema", "child"],
        "properties":{"schema":{"const":"fixture.parent/1"},
            "child":{"$ref":"../fixture.child/1#/$defs/Count"}}});
    let child = json!({"$id":"fixture.child/1", "$defs":{"Count":{"type":"integer","minimum":3}}});
    let registry = super::schema::Registry::new(
        [
            ("fixture.parent/1".into(), parent.clone()),
            ("fixture.child/1".into(), child),
        ]
        .into_iter()
        .collect(),
    )?;
    registry.validate(&json!({"schema":"fixture.parent/1", "child":3}), &[], true)?;
    if registry
        .validate(&json!({"schema":"fixture.parent/1", "child":2}), &[], true)
        .is_ok()
    {
        return Err("cross-artifact constraint ignored".into());
    }
    for reference in [
        "https://unregistered.invalid/schema",
        "file:///unregistered/schema.json",
    ] {
        let mut external = parent.clone();
        *external
            .pointer_mut("/properties/child/$ref")
            .ok_or("missing ref")? = json!(reference);
        if super::schema::Registry::new(
            [("fixture.parent/1".into(), external)]
                .into_iter()
                .collect(),
        )
        .is_ok()
        {
            return Err("unregistered reference accepted".into());
        }
    }
    Ok(())
}

fn rejects(
    registry: &Registry,
    value: &Value,
    args: &[String],
    success: bool,
) -> Result<(), String> {
    match registry.validate(value, args, success) {
        Err(error) if error.starts_with("schema validation:") => Ok(()),
        result => Err(format!(
            "expected schema validation failure; actual {result:?}"
        )),
    }
}

fn discovery_reply(
    environment: &super::launch::Environment,
    args: &[String],
) -> Result<Value, String> {
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), args)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}

fn discovery() -> Result<(), String> {
    let environment = super::launch::Environment::new().map_err(|e| e.to_string())?;
    let invoke = |args: &[String]| discovery_reply(&environment, args);
    let catalog = invoke(&["schema".into()])?;
    let rows = catalog
        .pointer("/result/schemas")
        .and_then(Value::as_array)
        .ok_or("missing schema catalog")?;
    let ids: std::collections::BTreeSet<_> = rows
        .iter()
        .map(|row| {
            let id = row["id"].as_str().ok_or("missing schema ID")?;
            if id.starts_with("biomcp.") && id.contains('_')
                || row["description"].as_str().is_none_or(str::is_empty)
            {
                return Err("schema ID or description is invalid");
            }
            Ok(id)
        })
        .collect::<Result<_, _>>()?;
    if ids != super::schema::ARTIFACTS.iter().map(|(id, _)| *id).collect() {
        return Err("catalog differs from shipped schema inventory".into());
    }
    let help = invoke(&["help".into()])?;
    let commands = help
        .pointer("/result/commands")
        .and_then(Value::as_array)
        .ok_or("missing commands")?;
    parent_replies(commands)?;
    let mut replies = std::collections::BTreeSet::new();
    for command in commands {
        let schemas = command["reply_schemas"]
            .as_array()
            .ok_or("missing reply schemas")?;
        for (entity, reply) in [
            ("drug", "biomcp.drug-who-card/1"),
            ("pgx", "biomcp.pgx-annotation/1"),
        ] {
            if command["command"] == json!(["get", entity])
                && !schemas.iter().any(|id| id.as_str() == Some(reply))
            {
                return Err(format!("missing union reply {reply} for get {entity}"));
            }
        }
        if matches!(
            command.pointer("/command/0").and_then(Value::as_str),
            Some(
                "get"
                    | "search"
                    | "article"
                    | "trial"
                    | "variant"
                    | "protein"
                    | "pathway"
                    | "author"
                    | "cell-line"
            )
        ) && schemas.is_empty()
        {
            return Err(format!("missing reply schemas for {}", command["command"]));
        }
        for id in schemas {
            let id = id.as_str().ok_or("invalid reply ID")?;
            if !ids.contains(id) {
                return Err(format!("unresolvable help reply {id}"));
            }
            replies.insert(id);
        }
    }
    let advertised = tool_definitions(&environment)?;
    for tool in &advertised {
        replies.extend(reply_ids(
            tool.get("outputSchema").ok_or("missing output schema")?,
        ));
    }
    for id in replies {
        let artifact = invoke(&["schema".into(), id.into()])?;
        if artifact.get("$id").and_then(Value::as_str) != Some(id) {
            return Err(format!("reply schema does not resolve: {id}"));
        }
    }
    Ok(())
}

fn parent_replies(commands: &[Value]) -> Result<(), String> {
    let parent = commands
        .iter()
        .find(|command| command["command"] == json!(["get"]))
        .ok_or("missing get parent command")?;
    let replies = parent["reply_schemas"]
        .as_array()
        .ok_or("missing parent reply schemas")?;
    for id in ["biomcp.drug-who-card/1", "biomcp.pgx-annotation/1"] {
        if !replies.iter().any(|reply| reply.as_str() == Some(id)) {
            return Err(format!("missing child reply {id} for get parent"));
        }
    }
    if replies
        .iter()
        .zip(replies.iter().skip(1))
        .any(|(left, right)| left.as_str() >= right.as_str())
    {
        return Err("parent reply schemas are not sorted and unique".into());
    }
    Ok(())
}

pub(super) fn reply_ids(schema: &Value) -> std::collections::BTreeSet<&str> {
    let mut ids = std::collections::BTreeSet::new();
    if let Some(property) = schema.pointer("/properties/schema") {
        ids.extend(property.get("const").and_then(Value::as_str));
        ids.extend(
            property
                .get("enum")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str),
        );
    }
    for branch in schema
        .get("anyOf")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        ids.extend(reply_ids(branch));
    }
    ids
}

fn tool_definitions(environment: &super::launch::Environment) -> Result<Vec<Value>, String> {
    let mut reply = super::mcp::exchange(
        environment,
        &[super::mcp::request(json!(1), "tools/list", json!({}))],
    )?
    .into_iter()
    .next()
    .ok_or("missing tool reply")?;
    let tools = reply
        .pointer_mut("/result/tools")
        .map(Value::take)
        .ok_or("missing tool list")?;
    serde_json::from_value(tools).map_err(|e| e.to_string())
}
