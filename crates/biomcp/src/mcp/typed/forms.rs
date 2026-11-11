//! One advertised name for search text, record IDs and sections; guessed names map onto it.
//!
//! The command registry names its search text `query`, `text` or `name`, its record ID
//! `input` or `id`, and its sections `section`. Callers that see only a tool's schema
//! guess across those, so every action advertises `query`, `id` and `section` and takes one
//! string or a list. The registry's own name and the common guesses are accepted without
//! being advertised: an action's schema names its arguments and does not forbid others, so
//! a client that checks arguments lets a guess through and the server maps or refuses it.
use serde_json::{Map, Value, json};

/// One advertised argument: the registry field it stands for and the other names accepted.
pub(super) struct Form {
    pub(super) advertised: &'static str,
    pub(super) field: String,
    aliases: Vec<&'static str>,
    /// Several search words are one phrase; several IDs or sections stay several values.
    phrase: bool,
}

const SEARCH_TEXT: [&str; 3] = ["query", "text", "name"];
const SEARCH_GUESSES: [&str; 2] = ["keyword", "keywords"];
const RECORD_ID: [&str; 2] = ["id", "input"];
const SECTIONS: [&str; 2] = ["section", "sections"];

/// The advertised arguments of one registry command, read from its own schema.
pub(super) fn forms(verb: &str, schema: &Value) -> Vec<Form> {
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return Vec::new();
    };
    let has = |name: &str| properties.contains_key(name);
    let first = |name: &str| {
        properties
            .get(name)
            .and_then(|node| node.get("x-positional"))
            .and_then(Value::as_u64)
            == Some(0)
    };
    let mut forms = Vec::new();
    if verb == "search"
        && let Some(field) = SEARCH_TEXT.into_iter().find(|name| has(name))
    {
        // The other spellings of search text, unless the command gives one its own meaning.
        let mut aliases = SEARCH_TEXT
            .into_iter()
            .filter(|name| *name != "query" && (*name == field || !has(name)))
            .collect::<Vec<_>>();
        if !SEARCH_GUESSES.into_iter().any(has) {
            aliases.extend(SEARCH_GUESSES);
        }
        forms.push(Form {
            advertised: "query",
            field: field.into(),
            aliases,
            phrase: true,
        });
    }
    if let Some(field) = RECORD_ID.into_iter().find(|name| first(name))
        && RECORD_ID.into_iter().filter(|name| has(name)).count() == 1
    {
        let aliases = RECORD_ID.into_iter().filter(|name| *name != "id").collect();
        forms.push(Form {
            advertised: "id",
            field: field.into(),
            aliases,
            phrase: false,
        });
    }
    if let Some(field) = SECTIONS.into_iter().find(|name| has(name))
        && SECTIONS.into_iter().filter(|name| has(name)).count() == 1
    {
        let aliases = SECTIONS
            .into_iter()
            .filter(|name| *name != "section")
            .collect();
        forms.push(Form {
            advertised: "section",
            field: field.into(),
            aliases,
            phrase: false,
        });
    }
    forms
}

/// The advertised name of a registry field.
pub(super) fn advertised<'a>(forms: &[Form], field: &'a str) -> &'a str {
    forms
        .iter()
        .find(|form| form.field == field)
        .map_or(field, |form| form.advertised)
}

/// The registry field an advertised or guessed name stands for.
pub(super) fn field<'a>(forms: &'a [Form], name: &'a str) -> &'a str {
    forms
        .iter()
        .find(|form| form.advertised == name || form.aliases.contains(&name))
        .map_or(name, |form| form.field.as_str())
}

/// A list argument also takes one bare string, which the command spells the same way.
fn widen(node: &mut Value) {
    if let Some(fields) = node.as_object_mut()
        && fields.get("type") == Some(&json!("array"))
    {
        fields.insert("type".into(), json!(["string", "array"]));
    }
}

/// An ID or a section also takes a list; its allowed values move onto the list's items.
fn listed(fields: &mut Map<String, Value>) {
    if fields.get("type") != Some(&json!("string")) {
        return;
    }
    let mut item = json!({"type":"string"});
    if let (Some(choices), Some(item)) = (fields.remove("enum"), item.as_object_mut()) {
        item.insert("enum".into(), choices);
    }
    fields.insert("type".into(), json!(["string", "array"]));
    fields.insert("items".into(), item);
}

/// Rewrite one action's compact schema to its advertised names. The schema stops forbidding
/// other names, so a guessed one reaches the server.
pub(super) fn advertise(input: &mut Value, forms: &[Form]) {
    let required = input
        .get("required")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    if let Some(fields) = input.as_object_mut() {
        fields.remove("additionalProperties");
    }
    let Some(properties) = input.get_mut("properties").and_then(Value::as_object_mut) else {
        return;
    };
    properties.values_mut().for_each(widen);
    let mut optional = Vec::new();
    for form in forms {
        let Some(mut node) = properties.remove(&form.field) else {
            continue;
        };
        if let Some(fields) = node.as_object_mut() {
            if !form.phrase {
                listed(fields);
            }
            if required.contains(&json!(form.field)) && !form.aliases.is_empty() {
                let text = fields
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                fields.insert("description".into(), json!(format!("Required. {text}")));
                optional.push(json!(form.field));
            }
        }
        properties.insert(form.advertised.into(), node);
    }
    // A client checks `required` before the server can read a guessed name, so an argument
    // with accepted guesses is required by the server and by its description instead.
    if let Some(Value::Array(names)) = input.get_mut("required") {
        names.retain(|name| !optional.contains(name));
        if names.is_empty()
            && let Some(fields) = input.as_object_mut()
        {
            fields.remove("required");
        }
    }
}

/// The argument a call got wrong and what it may be; no values means it was left out.
pub(super) struct Refused {
    pub(super) argument: String,
    pub(super) valid: Vec<String>,
}

/// Move every accepted name onto its registry field and make a one-item list one value.
/// `input` is the command's compact schema under its registry names.
pub(super) fn normalize(values: &mut Value, forms: &[Form], input: &Value) -> Result<(), Refused> {
    let required = input.get("required").and_then(Value::as_array);
    let Some(fields) = values.as_object_mut() else {
        return Ok(());
    };
    for form in forms {
        let mut names = vec![form.advertised];
        for name in std::iter::once(form.field.as_str()).chain(form.aliases.iter().copied()) {
            if !names.contains(&name) {
                names.push(name);
            }
        }
        let mut sent = names.into_iter().filter(|name| fields.contains_key(*name));
        let Some(name) = sent.next() else {
            if required.is_some_and(|names| names.contains(&json!(form.field))) {
                return Err(Refused {
                    argument: form.advertised.into(),
                    valid: Vec::new(),
                });
            }
            continue;
        };
        if let Some(second) = sent.next() {
            return Err(Refused {
                argument: second.into(),
                valid: vec![form.advertised.into()],
            });
        }
        let Some(mut value) = fields.remove(name) else {
            continue;
        };
        let kind = input.pointer(&format!("/properties/{}/type", form.field));
        let list = kind == Some(&json!("array"));
        if let Value::Array(items) = &value
            && !list
        {
            if form.phrase && items.iter().all(Value::is_string) {
                let words = items.iter().filter_map(Value::as_str).collect::<Vec<_>>();
                value = json!(words.join(" "));
            } else if let [one] = items.as_slice() {
                value = one.clone();
            } else if form.advertised == "id" {
                // The command would read a second ID as a section name and refuse that.
                return Err(Refused {
                    argument: form.advertised.into(),
                    valid: vec!["one value per call".into()],
                });
            }
        }
        fields.insert(form.field.clone(), value);
    }
    Ok(())
}

/// Spell a registry-named arguments object with its advertised names, for a next call.
pub(super) fn advertised_arguments(arguments: &Value, forms: &[Form]) -> Value {
    match arguments.as_object() {
        Some(fields) => Value::Object(
            fields
                .iter()
                .map(|(key, value)| (advertised(forms, key).to_owned(), value.clone()))
                .collect(),
        ),
        None => arguments.clone(),
    }
}
