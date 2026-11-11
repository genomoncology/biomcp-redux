//! Worked examples from the skill's use cases, each with the typed call this server derives.
use serde_json::Value;
use std::io;

const EXAMPLES: &str = include_str!(concat!(env!("OUT_DIR"), "/first-read-examples.json"));

/// Each use case's example and the typed call derived from its command through the grammar
/// and the default typed catalog; `None` when no typed tool serves that command.
pub(crate) fn calls(root: &crate::grammar::Root) -> io::Result<Vec<(Value, Option<Value>)>> {
    let examples: Vec<Value> = serde_json::from_str(EXAMPLES).map_err(io::Error::other)?;
    let globals = crate::grammar::Globals::default();
    let settings = Ok(None);
    let fetch = Err(crate::config::StartupError::Local);
    let context = crate::command::Context::new(&globals, &fetch, root, &settings);
    let catalog = super::catalog(context, true)?;
    Ok(examples
        .into_iter()
        .map(|example| {
            let call = example
                .get("args")
                .and_then(Value::as_array)
                .and_then(|args| super::summary::translate(root, args, &catalog));
            (example, call)
        })
        .collect())
}
