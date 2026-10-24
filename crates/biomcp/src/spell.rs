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
        Action::Sync { bundle } => Ok(vec![
            "sync".into(),
            match bundle {
                Bundle::Hgnc => "hgnc",
            }
            .into(),
        ]),
        Action::Run { request, fresh } => {
            let mut args = request_args(request)?;
            if *fresh {
                args.push("--fresh".into());
            }
            Ok(args)
        }
    }
}
fn request_args(request: &Request) -> Result<Vec<String>, serde_json::Error> {
    let schema: Value = serde_json::from_str(
        crate::schema::find(crate::grammar::query_schema(request)).unwrap_or("{}"),
    )?;
    let value = match request {
        Request::GetGene(query) => serde_json::to_value(query)?,
        Request::SearchGene(query) => serde_json::to_value(query)?,
    };
    let mut args = vec![
        match request {
            Request::GetGene(_) => "get",
            Request::SearchGene(_) => "search",
        }
        .into(),
        "gene".into(),
    ];
    if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
        let mut fields: Vec<_> = properties.iter().collect();
        fields.sort_by_key(|(_, property)| {
            property
                .get("x-positional")
                .and_then(Value::as_u64)
                .unwrap_or(u64::MAX)
        });
        for (name, property) in fields {
            let Some(value) = value
                .get(name)
                .filter(|value| !value.is_null() && property.get("default") != Some(*value))
            else {
                continue;
            };
            if property.get("x-positional").is_none() {
                args.push(format!("--{name}"));
            }
            for value in value
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or(std::slice::from_ref(value))
            {
                args.push(
                    value
                        .as_str()
                        .map_or_else(|| value.to_string(), str::to_owned),
                );
            }
        }
    }
    Ok(args)
}
