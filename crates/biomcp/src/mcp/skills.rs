//! Skills over MCP backed by the embedded research guide and worked examples.
use super::{current, protocol, typed::catalog};
use crate::{command::Context, skill::research};
use biomcp_types::Source;
use serde_json::{Map, Value, json};
use std::{collections::BTreeSet, ffi::OsString, io};

const EXTENSION: &str = "io.modelcontextprotocol/skills";
const ROOT: &str = "skill://biomcp-research/";
const GUIDE: &str = "SKILL.md";
pub(super) const GUIDE_URI: &str = "skill://biomcp-research/SKILL.md";
const MARKDOWN: &str = "text/markdown";

pub(super) fn capabilities(context: Context<'_>, extensions: bool) -> Value {
    let mut capabilities = Map::from_iter([("tools".into(), json!({}))]);
    if crate::config::mcp_skills(context.settings) {
        capabilities.insert("resources".into(), json!({}));
        if extensions {
            capabilities.insert(
                "extensions".into(),
                json!({EXTENSION: {"directoryRead":false}}),
            );
        }
    }
    Value::Object(capabilities)
}

pub(super) fn handles(method: &str, context: Context<'_>, modern: bool) -> bool {
    crate::config::mcp_skills(context.settings)
        && match method {
            "skills/list" | "skills/get" => modern,
            "resources/list" | "resources/read" | "resources/templates/list" => true,
            _ => false,
        }
}

pub(super) fn reply(
    id: Value,
    method: &str,
    params: &Value,
    context: Context<'_>,
    modern: bool,
) -> Value {
    let result = serve(method, params, context);
    let mut result = match result {
        Ok(Some(result)) => result,
        Ok(None) => return protocol::error(id, -32602, "Invalid params"),
        Err(_) => return protocol::error(id, -32603, "Research skill unavailable"),
    };
    if modern {
        current::add_catalog_cache_fields(&mut result, context.globals.hosted);
    }
    protocol::result(id, result, modern)
}

fn serve(method: &str, params: &Value, context: Context<'_>) -> io::Result<Option<Value>> {
    let uri = params.get("uri").and_then(Value::as_str);
    let result = match method {
        "skills/get" if uri != Some(GUIDE_URI) => return Ok(None),
        "skills/list" | "resources/list" | "resources/templates/list"
            if params.get("cursor").is_some() =>
        {
            return Ok(None);
        }
        "resources/read" => {
            let Some(path) = uri.and_then(|uri| uri.strip_prefix(ROOT)) else {
                return Ok(None);
            };
            return Ok(files(context, Some(path))?.into_iter().next().map(|file| {
                json!({"contents":[{"uri":file.uri(),"mimeType":MARKDOWN,"text":file.text}]})
            }));
        }
        "resources/templates/list" => json!({"resourceTemplates":[]}),
        "resources/list" => {
            let resources: Vec<_> = files(context, None)?
                .into_iter()
                .map(|file| json!({"uri":file.uri(),"name":file.path,"mimeType":MARKDOWN}))
                .collect();
            json!({"resources":resources})
        }
        "skills/list" => json!({"skills":[entry(context)?]}),
        "skills/get" => json!({"skill":entry(context)?}),
        _ => return Ok(None),
    };
    Ok(Some(result))
}

fn entry(context: Context<'_>) -> io::Result<Value> {
    let resources: Vec<_> = files(context, None)?
        .into_iter()
        .map(|file| {
            json!({"uri":file.uri(),"digest":format!("sha256:{}", crate::skill::digest(file.text.as_bytes())),
                "size":file.text.len()})
        })
        .collect();
    let frontmatter: Value = serde_json::from_str(research::FRONTMATTER)?;
    Ok(json!({"uri":GUIDE_URI,"frontmatter":frontmatter,"resources":resources}))
}

struct File {
    path: String,
    text: &'static str,
}

impl File {
    fn uri(&self) -> String {
        format!("{ROOT}{}", self.path)
    }
}

fn files(context: Context<'_>, path: Option<&str>) -> io::Result<Vec<File>> {
    let excluded = if context.globals.hosted && path != Some(GUIDE) {
        catalog::excluded(context)?
    } else {
        BTreeSet::new()
    };
    let files = std::iter::once(File {
        path: GUIDE.into(),
        text: research::GUIDE,
    })
    .chain(research::references().map(|(name, text)| File {
        path: format!("references/{name}"),
        text,
    }))
    .filter(|file| path.is_none_or(|path| path == file.path))
    .filter(|file| {
        file.path == GUIDE || excluded.is_empty() || permitted(file.text, context, &excluded)
    })
    .collect();
    Ok(files)
}

fn permitted(text: &str, context: Context<'_>, excluded: &BTreeSet<Source>) -> bool {
    // Check every example command, including follow-ups that have no typed example.
    text.lines()
        .filter_map(|line| line.strip_prefix("biomcp "))
        .all(|command| {
            let Some(words) = shlex::split(command) else {
                return false;
            };
            let args: Vec<OsString> = words.into_iter().map(OsString::from).collect();
            matches!(
                crate::grammar::parse(context.root, &args)
                    .ok()
                    .and_then(|parsed| parsed.action),
                Some(biomcp_query::Action::Run { request, .. })
                    if catalog::request_allowed(&request, excluded).unwrap_or(false)
            )
        })
}
