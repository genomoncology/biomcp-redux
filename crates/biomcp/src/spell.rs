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
        Action::Health => Ok(vec!["health".into()]),
        Action::KeyHelp(source) => Ok(vec!["help".into(), "key".into(), source.to_string()]),
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
    let query = query_value(request)?;
    let mut args = command_words(request);
