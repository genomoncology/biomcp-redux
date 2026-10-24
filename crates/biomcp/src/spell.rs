//! Reverse the same embedded query schema used by the parser.
use biodata::{Request, Step, StepKind};
use biomcp_query::{Action, Bundle};
use serde_json::Value;

pub(crate) fn step(kind: StepKind, action: &Action) -> Result<Step, serde_json::Error> {
    Ok(Step {
        kind,
        args: action_args(action)?
            .into_iter()
            .map(String::into_boxed_str)
            .collect(),
    })
}
pub(crate) fn action_args(action: &Action) -> Result<Vec<String>, serde_json::Error> {
    match action {
        Action::KeyHelp => Ok(vec!["help".into(), "key".into(), "oncokb".into()]),
        Action::Sync { bundle } => Ok(vec![
            "sync".into(),
            match bundle {
                Bundle::Hgnc => "hgnc",
                Bundle::ClinGen => "clingen",
            }
            .into(),
        ]),
        Action::Run { request, fresh } => {
            let mut args = request_args(request)?;
            if *fresh {
                let position = args
                    .iter()
                    .position(|arg| arg == "--")
                    .unwrap_or(args.len());
                args.insert(position, "--fresh".into());
            }
            Ok(args)
        }
    }
}
fn request_args(request: &Request) -> Result<Vec<String>, serde_json::Error> {
    let schema: Value = serde_json::from_str(
        crate::schema::find(crate::grammar::query_schema(request)).unwrap_or("{}"),
    )?;
    let query = match request {
        Request::ArticleText(query) => serde_json::to_value(query)?,
        Request::GetArticle(query) => serde_json::to_value(query)?,
        Request::SearchArticle(query) => serde_json::to_value(query)?,
        Request::GetGene(query) => serde_json::to_value(query)?,
        Request::SearchGene(query) => serde_json::to_value(query)?,
    };
    let mut args = vec![
        match request {
            Request::ArticleText(_) => "article",
            Request::GetGene(_) | Request::GetArticle(_) => "get",
            Request::SearchGene(_) | Request::SearchArticle(_) => "search",
        }
        .into(),
        if matches!(request, Request::ArticleText(_)) {
            "text"
        } else if matches!(request, Request::SearchArticle(_) | Request::GetArticle(_)) {
            "article"
        } else {
            "gene"
        }
        .into(),
    ];
    let positional = fields(&schema, &query, &mut args);
    if positional.iter().any(|arg| arg.starts_with('-')) {
        args.push("--".into());
        args.extend(positional);
    } else {
        args.splice(2..2, positional);
    }
    Ok(args)
}

fn trailing_input(schema: &Value, query: &Value, property: &Value) -> bool {
    let item = property.get("items").unwrap_or(&Value::Null);
    let item = item
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|s| schema.pointer(s.trim_start_matches('#')))
        .unwrap_or(item);
    query
        .get("input")
        .and_then(Value::as_array)
        .is_some_and(|inputs| {
            inputs.iter().any(|input| {
                item.get("enum")
                    .and_then(Value::as_array)
                    .is_some_and(|words| words.contains(input))
            })
        })
}

fn fields(schema: &Value, query: &Value, args: &mut Vec<String>) -> Vec<String> {
    let mut positional = Vec::new();
    let mut trailing = Vec::new();
    if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
        let mut fields: Vec<_> = properties.iter().collect();
        fields.sort_by_key(|(_, property)| {
            property
                .get("x-positional")
                .and_then(Value::as_u64)
                .unwrap_or(u64::MAX)
        });
        for (name, property) in fields {
            let Some(value) = query
                .get(name)
                .filter(|value| !value.is_null() && property.get("default") != Some(*value))
            else {
                continue;
            };
            let is_trailing = property.get("x-trailing-section") == Some(&Value::Bool(true));
            let explicit = is_trailing && trailing_input(schema, query, property);
            if property.get("x-positional").is_none() && !is_trailing {
                args.push(format!("--{}", name.replace('_', "-")));
            }
            for value in value
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or(std::slice::from_ref(value))
            {
                let target = if is_trailing && !explicit {
                    &mut trailing
                } else if property.get("x-positional").is_some() {
                    &mut positional
                } else {
                    &mut *args
                };
                if explicit {
                    target.push(format!("--{}", name.replace('_', "-")));
                }
                target.push(
                    value
                        .as_str()
                        .map_or_else(|| value.to_string(), str::to_owned),
                );
            }
        }
    }
    positional.extend(trailing);
    positional
}
