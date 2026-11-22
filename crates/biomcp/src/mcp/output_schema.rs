//! Compact, self-contained tool reply schemas derived from canonical command contracts.
use serde_json::{Map, Value, json};
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    io,
};

/// One reply shape for an entity tool: the union of its actions' command replies.
pub(super) fn for_commands(paths: &[(&str, &str, &str)]) -> io::Result<Value> {
    for_replies(command_replies(paths.iter().copied()))
}

fn command_replies<'a>(
    paths: impl Iterator<Item = (&'a str, &'a str, &'a str)>,
) -> BTreeSet<String> {
    paths
        .flat_map(|(verb, entity, id)| crate::schema::replies(id, verb, entity))
        .collect()
}

/// The `saved` tool's reply: saved work, or the reply of a search it ran that failed.
pub(super) fn for_saved() -> io::Result<Value> {
    let mut ids = command_replies(
        crate::grammar::command_paths()
            .into_iter()
            .filter(|(verb, ..)| *verb == "search"),
    );
    ids.insert(<biomcp_types::Saved as biomcp_types::ResponseBody>::SCHEMA.into());
    for_replies(ids)
}

pub(super) fn for_help() -> io::Result<Value> {
    for_replies(
        [vec!["help".into()], vec!["help".into(), "key".into()]]
            .iter()
            .flat_map(|path| crate::schema::local_replies(path))
            .collect(),
    )
}

fn for_replies(ids: BTreeSet<String>) -> io::Result<Value> {
    let mut shapes = Shapes::default();
    let reply = envelope(&mut shapes, &ids)?;
    let failure = error(&mut shapes)?;
    Ok(shapes.finish(vec![reply, failure]))
}

fn envelope(shapes: &mut Shapes, ids: &BTreeSet<String>) -> io::Result<Value> {
    let mut health = canonical(<biomcp_types::Health as biomcp_types::ResponseBody>::SCHEMA)?;
    fields(&mut health)?.shift_remove("result");
    let mut envelope = shapes.record(&health)?;
    let body = fields(&mut envelope)?;
    body.insert("schema".into(), json!({"enum":ids}));
    body.insert("query".into(), json!({"type":["object","array","null"]}));
    let replies = ids
        .iter()
        .map(|id| {
            let root = canonical(id)?;
            let result = root
                .pointer("/properties/result")
                .ok_or_else(|| io::Error::other(format!("missing result in reply schema {id}")))?;
            Ok(json!({"properties":{"schema":{"const":id},"result":shapes.copy(&root, result)?}}))
        })
        .collect::<io::Result<Vec<_>>>()?;
    object(&mut envelope)?.insert("anyOf".into(), json!(replies));
    Ok(envelope)
}

pub(super) fn for_tool(root: &crate::grammar::Root, name: &str) -> io::Result<Value> {
    let mut ids = crate::help::commands(root)?
        .into_iter()
        .filter(|command| match name {
            "search" => command.command.first().is_some_and(|word| word == "search"),
            "fetch" => command.command.first().is_some_and(|word| word == "get"),
            _ => true,
        })
        .flat_map(|command| command.reply_schemas)
        .collect::<BTreeSet<_>>();
    if name == "fetch" {
        ids.extend(crate::schema::local_replies(&["help".into()]));
    }
    // A single-mode server has no typed-only next steps, while the common
    // tools also run in typed mode. The schema accepts either spelling.
    let mut shapes = Shapes::default();
    let reply = envelope(&mut shapes, &ids)?;
    let failure = error(&mut shapes)?;
    let extra = match name {
        "biomcp" => json!({"properties":{
            "$id":{"type":"string","pattern":"^(biomcp|biodata)\\.[a-z0-9_-]+/[1-9][0-9]*$"},
            "$schema":{"const":"https://json-schema.org/draft/2020-12/schema"}},
            "required":["$id","$schema"]}),
        "search" => {
            let id = biomcp_types::ResearchSearch::<biomcp_types::ResearchEnvelope>::SCHEMA;
            let mut search = shapes.record(&canonical(id)?)?;
            let metadata = fields(&mut search)?
                .get_mut("metadata")
                .ok_or_else(|| io::Error::other("missing research search metadata"))?;
            let mut definition = shapes.definition(metadata)?.clone();
            fields(&mut definition)?.insert("envelope".into(), json!({"$ref":"#/anyOf/0"}));
            *metadata = definition;
            search
        }
        "fetch" => {
            let id = biomcp_types::ResearchFetch::<biomcp_types::ResearchEnvelope>::SCHEMA;
            let mut fetch = shapes.record(&canonical(id)?)?;
            let mut metadata = reply.clone();
            object(&mut metadata)?.shift_remove("anyOf");
            fields(&mut fetch)?.insert("metadata".into(), metadata);
            fetch
        }
        _ => return Err(io::Error::other("unknown research tool")),
    };
    Ok(shapes.finish(vec![reply, failure, extra]))
}

fn error(shapes: &mut Shapes) -> io::Result<Value> {
    let root = canonical("biomcp.error/4")?;
    let mut error = shapes.record(&root)?;
    for key in ["schema", "status"] {
        let original = root
            .get("properties")
            .and_then(|fields| fields.get(key))
            .ok_or_else(|| io::Error::other("missing error reply property"))?;
        let node = root
            .pointer(local_pointer(original)?)
            .ok_or_else(|| io::Error::other("missing error reply discriminator"))?;
        let mut constraint = shapes.copy(&root, node)?;
        for keyword in ["const", "enum"] {
            if let Some(value) = node.get(keyword) {
                object(&mut constraint)?.insert(keyword.into(), value.clone());
            }
        }
        fields(&mut error)?.insert(key.into(), constraint);
    }
    Ok(error)
}

fn canonical(id: &str) -> io::Result<Value> {
    serde_json::from_str(
        crate::schema::find(id).ok_or_else(|| io::Error::other("missing reply schema"))?,
    )
    .map_err(io::Error::other)
}

fn object(schema: &mut Value) -> io::Result<&mut Map<String, Value>> {
    schema
        .as_object_mut()
        .ok_or_else(|| io::Error::other("expected output schema object"))
}

fn fields(schema: &mut Value) -> io::Result<&mut Map<String, Value>> {
    schema
        .get_mut("properties")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| io::Error::other("missing output schema properties"))
}

fn local_pointer(node: &Value) -> io::Result<&str> {
    node.get("$ref")
        .and_then(Value::as_str)
        .and_then(|reference| reference.strip_prefix('#'))
        .ok_or_else(|| io::Error::other("output schema reference must be local"))
}

#[derive(Default)]
struct Shapes {
    definitions: Map<String, Value>,
    seen: HashSet<(String, String)>,
    aliases: BTreeMap<String, String>,
}

impl Shapes {
    fn finish(self, branches: Vec<Value>) -> Value {
        json!({"$schema":super::protocol::SCHEMA_DIALECT,"type":"object","anyOf":branches,"$defs":self.definitions})
    }

    fn record(&mut self, root: &Value) -> io::Result<Value> {
        let mut record = self.copy(root, root)?;
        if let Some(required) = root.get("required") {
            object(&mut record)?.insert("required".into(), required.clone());
        }
        Ok(record)
    }

    fn definition(&self, reference: &Value) -> io::Result<&Value> {
        let key = local_pointer(reference)?.trim_start_matches("/$defs/");
        self.definitions
            .get(key)
            .ok_or_else(|| io::Error::other("missing output definition"))
    }

    // Keep names and structural types only. Projected rows and hosted metadata can
    // omit canonical fields. The envelope and research wrappers retain required keys.
    fn copy(&mut self, root: &Value, source: &Value) -> io::Result<Value> {
        let Some(node) = source.as_object() else {
            return Ok(source.clone());
        };
        if node.contains_key("$ref") {
            return self.reference(root, local_pointer(source)?);
        }
        let mut shape = Map::new();
        if let Some(kind) = node.get("type") {
            shape.insert("type".into(), kind.clone());
        }
        for key in ["properties", "patternProperties"] {
            if let Some(properties) = node.get(key).and_then(Value::as_object) {
                let properties = properties
                    .iter()
                    .map(|(name, value)| Ok((name.clone(), self.copy(root, value)?)))
                    .collect::<io::Result<Map<_, _>>>()?;
                shape.insert(key.into(), Value::Object(properties));
            }
        }
        for key in ["items", "additionalProperties"] {
            if let Some(item) = node.get(key) {
                shape.insert(key.into(), self.copy(root, item)?);
            }
        }
        for key in ["anyOf", "oneOf", "allOf", "prefixItems"] {
            if let Some(branches) = node.get(key).and_then(Value::as_array) {
                let mut copied = Vec::new();
                for branch in branches {
                    let branch = self.copy(root, branch)?;
                    if key == "prefixItems" || !copied.contains(&branch) {
                        copied.push(branch);
                    }
                }
                let keyword = if key == "oneOf" { "anyOf" } else { key };
                if key == "anyOf" && shape.is_empty() {
                    if let [only] = copied.as_slice() {
                        return Ok(only.clone());
                    }
                    if let Some(kinds) = copied.iter().map(simple_kind).collect::<Option<Vec<_>>>()
                    {
                        return Ok(json!({"type":kinds}));
                    }
                }
                shape.insert(keyword.into(), Value::Array(copied));
            }
        }
        Ok(Value::Object(shape))
    }

    fn reference(&mut self, root: &Value, pointer: &str) -> io::Result<Value> {
        let name = pointer.trim_start_matches("/$defs/");
        let target = root
            .pointer(pointer)
            .ok_or_else(|| io::Error::other("missing reply definition"))?;
        if name == "Reply_Step" {
            self.definitions.entry("Step").or_insert_with(step);
            return Ok(json!({"$ref":"#/$defs/Step"}));
        }
        if let Some(kind) = target.get("type")
            && !matches!(kind.as_str(), Some("object" | "array") | None)
        {
            return Ok(json!({"type":kind}));
        }
        let name = name.trim_start_matches("Reply_");
        let id = root
            .get("$id")
            .and_then(Value::as_str)
            .ok_or_else(|| io::Error::other("missing reply ID"))?;
        let next = self.aliases.len();
        let alias = self
            .aliases
            .entry(name.into())
            .or_insert_with(|| next.to_string())
            .clone();
        if self.seen.insert((id.into(), name.into())) {
            let shape = self.copy(root, target)?;
            if self
                .definitions
                .get(&alias)
                .is_some_and(|existing| *existing != shape)
            {
                return Err(io::Error::other(format!(
                    "inconsistent reply definition {name}"
                )));
            }
            self.definitions.insert(alias.clone(), shape);
        }
        Ok(json!({"$ref":format!("#/$defs/{alias}")}))
    }
}

fn simple_kind(schema: &Value) -> Option<&str> {
    let node = schema.as_object()?;
    if node.len() == 1 {
        return node.get("type")?.as_str();
    }
    None
}

fn step() -> Value {
    json!({"type":"object","properties":{"kind":{"type":"string"},"name":{"type":"string"},
        "args":{"type":"array","items":{"type":"string"}},"arguments":{"type":"object"},
        "options":{"type":"object"}},"anyOf":[{"required":["args"]},{"required":["name","arguments"]}]})
}
