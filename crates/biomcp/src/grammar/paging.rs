//! One way to page a list (1061): `limit` and `offset` page every list the reply returns.
//!
//! A section's `section_limit` and `section_offset` are other names for the same option, so a
//! command that has only one pair takes the other pair's names too. `get variant` has both: with
//! a named section, `limit` and `offset` page that section; without one they page candidates.
use serde_json::{Map, Value};

/// The two names of each paging option: the list name and the section name.
pub(crate) const PAIRS: [(&str, &str); 2] =
    [("limit", "section_limit"), ("offset", "section_offset")];

/// The schema field a paging name stands for in this command, when it is not its own field.
pub(crate) fn other_name(schema: &Value, name: &str) -> Option<&'static str> {
    let has = |field: &str| schema.pointer(&format!("/properties/{field}")).is_some();
    PAIRS.iter().find_map(|&(list, section)| {
        if name == list && has(section) && !has(list) {
            Some(section)
        } else if name == section && has(list) && !has(section) && has("section") {
            Some(list)
        } else {
            None
        }
    })
}

/// The one help line that names the options paging this command's lists.
pub(crate) fn help_line(schema: &Value) -> Option<String> {
    let has = |field: &str| schema.pointer(&format!("/properties/{field}")).is_some();
    let flag = |field: &str| format!("--{}", field.replace('_', "-"));
    let mut names: Vec<_> = PAIRS
        .iter()
        .filter(|(list, section)| has(list) || has(section))
        .map(|(list, _)| flag(list))
        .collect();
    if has("page") {
        names.push(flag("page"));
    }
    if names.is_empty() {
        return None;
    }
    let names = names.join(" and ");
    let others: Vec<_> = PAIRS
        .iter()
        .filter(|(_, section)| has(section) || other_name(schema, section).is_some())
        .map(|(_, section)| flag(section))
        .collect();
    let same = if others.len() == 1 {
        "is the same option"
    } else {
        "are the same options"
    };
    let others = others.join(" and ");
    Some(if has("limit") && has("section_limit") {
        format!(
            "Paging: {names} page the named section, else the candidates; {others} page the section."
        )
    } else if others.is_empty() {
        format!("Paging: {names}.")
    } else {
        format!("Paging: {names}; {others} {same}.")
    })
}

/// Move each paging value onto the field this command reads it from.
pub(crate) fn rename(schema: &Value, fields: &mut Map<String, Value>) {
    let has = |field: &str| schema.pointer(&format!("/properties/{field}")).is_some();
    let sectioned = fields.get("section").is_some_and(named);
    for (list, section) in PAIRS {
        let (from, to) = if has(list) && has(section) {
            if !sectioned {
                continue;
            }
            (list, section)
        } else if let Some(to) = other_name(schema, list) {
            (list, to)
        } else if let Some(to) = other_name(schema, section) {
            (section, to)
        } else {
            continue;
        };
        if !fields.contains_key(to)
            && let Some(value) = fields.remove(from)
        {
            fields.insert(to.into(), value);
        }
    }
}

fn named(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Array(values) => !values.is_empty(),
        _ => true,
    }
}

/// The paging options a command may have, and other words callers try for them.
const OPTIONS: [&str; 5] = ["limit", "offset", "section-limit", "section-offset", "page"];
const GUESSES: [&str; 5] = ["page-size", "per-page", "size", "skip", "rows"];

/// Whether a refusal of `spelling` lists the option `name`. A paging word the command lacks is
/// answered with the paging options it has, so the refusal names the working form; any other
/// refused word, or a command without paging, lists every option.
pub(crate) fn listed(command: &clap::Command, spelling: &str, name: &str) -> bool {
    let word = spelling.trim_start_matches('-');
    let paging = |name: &str| OPTIONS.contains(&name);
    let pages = command
        .get_arguments()
        .filter(|argument| !argument.is_hide_set())
        .filter_map(clap::Arg::get_long)
        .any(paging);
    !(pages && (paging(word) || GUESSES.contains(&word))) || paging(name)
}
