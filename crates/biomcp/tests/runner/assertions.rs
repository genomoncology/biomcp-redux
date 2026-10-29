use super::Step;
use serde_json::Value;

#[derive(Debug)]
pub(super) enum Assertion<'a> {
    Equals(&'a Value),
    Omits(&'a Value),
    Unique,
    Nonincreasing,
    IncludesAllOf(&'a Value),
    Includes(&'a Value),
    Excludes(&'a Value),
    Count(u64),
    RankOf(&'a str, u64),
    MaxBytes(u64),
    MaxTokens(u64),
}

pub(super) type Expectations<'a> = Vec<(&'a str, &'a Value, Vec<Assertion<'a>>)>;

pub(super) fn prepare(step: &Step) -> Result<Expectations<'_>, String> {
    step.expect
        .iter()
        .map(|(pointer, value)| {
            validate_pointer(pointer)?;
            reject_empty_wildcard(pointer, value)?;
            let Some(options) = value.as_object() else {
                return Ok((pointer.as_str(), value, vec![Assertion::Equals(value)]));
            };
            if options.is_empty() {
                return Err(format!("{pointer}: assertion object requires an operator"));
            }
            let mut assertions = Vec::new();
            for (operator, expected) in options {
                match operator.as_str() {
                    "equals" | "equals_file" => assertions.push(Assertion::Equals(expected)),
                    "includes_all_of" => {
                        array(pointer, operator, expected)?;
                        assertions.push(Assertion::IncludesAllOf(expected));
                    }
                    "nonincreasing" if expected == &Value::Bool(true) => {
                        assertions.push(Assertion::Nonincreasing)
                    }
                    "unique" if expected == &Value::Bool(true) => {
                        assertions.push(Assertion::Unique)
                    }
                    "omits" => {
                        let names = array(pointer, operator, expected)?;
                        if names.is_empty() || names.iter().any(|name| name.as_str().is_none()) {
                            return Err(format!("{pointer}: omits requires nonempty field names"));
                        }
                        assertions.push(Assertion::Omits(expected));
                    }
                    "includes" => assertions.push(Assertion::Includes(expected)),
                    "excludes" => assertions.push(Assertion::Excludes(expected)),
                    "count" => {
                        assertions.push(Assertion::Count(integer(pointer, operator, expected)?))
                    }
                    "max_bytes" => {
                        assertions.push(Assertion::MaxBytes(integer(pointer, operator, expected)?))
                    }
                    "max_tokens" => {
                        assertions.push(Assertion::MaxTokens(integer(pointer, operator, expected)?))
                    }
                    "rank_of" => {
                        let ranks = expected
                            .as_object()
                            .filter(|ranks| !ranks.is_empty())
                            .ok_or_else(|| {
                                format!("{pointer}: rank_of requires a nonempty object")
                            })?;
                        for (member, rank) in ranks {
                            let rank = rank.as_u64().filter(|rank| *rank > 0).ok_or_else(|| {
                                format!("{pointer}: rank_of requires positive integer ranks")
                            })?;
                            assertions.push(Assertion::RankOf(member, rank));
                        }
                    }
                    _ => return Err(format!("{pointer}: unknown assertion operator {operator}")),
                }
            }
            Ok((pointer.as_str(), value, assertions))
        })
        .collect()
}

fn reject_empty_wildcard(pointer: &str, value: &Value) -> Result<(), String> {
    let empty_array = |value: &Value| value.as_array().is_some_and(Vec::is_empty);
    if pointer.split('/').skip(1).any(|token| token == "*")
        && (empty_array(value)
            || value.get("equals").is_some_and(empty_array)
            || value.get("count").and_then(Value::as_u64) == Some(0))
    {
        return Err(format!(
            "{pointer}: wildcard cannot assert an empty array; assert on the parent array instead, such as {{'/items': {{equals: [], count: 0}}}}"
        ));
    }
    Ok(())
}

fn integer(pointer: &str, operator: &str, value: &Value) -> Result<u64, String> {
    value
        .as_u64()
        .ok_or_else(|| format!("{pointer}: {operator} requires a nonnegative integer"))
}

fn validate_pointer(pointer: &str) -> Result<(), String> {
    if !pointer.is_empty() && !pointer.starts_with('/') {
        return Err(format!("{pointer}: expected a JSON pointer"));
    }
    let mut characters = pointer.chars();
    while let Some(character) = characters.next() {
        if character == '~' && !matches!(characters.next(), Some('0' | '1')) {
            return Err(format!("{pointer}: invalid JSON pointer escape"));
        }
    }
    Ok(())
}

pub(super) fn check(
    stdout: &[u8],
    step: &Step,
    expectations: &Expectations<'_>,
) -> Result<(), String> {
    let parsed = serde_json::from_slice::<Value>(stdout);
    let terms_bytes = if let Ok(value) = &parsed
        && !step.full_output_bytes
    {
        serde_json::to_vec(value).map_err(|e| e.to_string())?.len()
            - super::source_terms::size(value, "")?
    } else {
        0
    };
    ceiling(
        "stdout",
        "max_bytes",
        stdout.len() - terms_bytes,
        step.max_bytes.map(|v| v as u64),
    )?;
    if expectations.is_empty() && step.max_tokens.is_none() {
        return Ok(());
    }
    let value: Value =
        serde_json::from_slice(stdout).map_err(|error| format!("stdout JSON: {error}"))?;
    ceiling(
        "stdout",
        "max_tokens",
        if step.full_output_bytes {
            stdout.len().div_ceil(4)
        } else {
            super::source_terms::size(&value, "")?.div_ceil(4)
        },
        step.max_tokens.map(|v| v as u64),
    )?;
    for (pointer, expected, assertions) in expectations {
        let actual = select(&value, pointer)
            .map_err(|error| format!("{pointer}: expected {expected}, actual {error}"))?;
        for assertion in assertions {
            assertion.check(pointer, &actual)?;
        }
    }
    Ok(())
}

pub(super) fn select(value: &Value, pointer: &str) -> Result<Value, String> {
    let mut nodes = vec![value];
    let mut wildcard = false;
    for token in pointer.split('/').skip(1) {
        let mut next = Vec::new();
        let mut missing = false;
        for node in nodes {
            if token == "*" {
                wildcard = true;
                next.extend(
                    node.as_array()
                        .ok_or_else(|| format!("{node}; wildcard requires an array"))?,
                );
            } else if let Some(child) = node.pointer(&format!("/{token}")) {
                next.push(child);
            } else if !wildcard {
                return Err("<missing>".into());
            } else {
                missing = true;
            }
        }
        if missing && (next.is_empty() || next.iter().any(|value| !value.is_array())) {
            return Err("<missing>".into());
        }
        if next.is_empty() {
            return Err("<empty wildcard selection>".into());
        }
        nodes = next;
    }
    if wildcard {
        Ok(Value::Array(nodes.into_iter().cloned().collect()))
    } else {
        nodes
            .first()
            .map(|node| (*node).clone())
            .ok_or_else(|| "<missing>".into())
    }
}

/// Project source snapshots into the product's empty-collection wire convention.
pub(super) fn omit_empty_lists(value: &mut Value) {
    match value {
        Value::Object(fields) => {
            for child in fields.values_mut() {
                omit_empty_lists(child);
            }
            fields.retain(|_, child| !child.as_array().is_some_and(Vec::is_empty));
        }
        Value::Array(items) => {
            for item in items {
                omit_empty_lists(item);
            }
        }
        _ => {}
    }
}

/// Apply optional-field omission to expected native record projections only.
/// Array values are retained, including explicit null array members.
pub(super) fn omit_null_fields(value: &mut Value, optional: &[&str]) {
    match value {
        Value::Object(fields) => {
            fields.retain(|name, child| !optional.contains(&name.as_str()) || !child.is_null());
            for child in fields.values_mut() {
                omit_null_fields(child, optional);
            }
        }
        Value::Array(items) => {
            for item in items {
                omit_null_fields(item, optional);
            }
        }
        _ => {}
    }
}

fn omits(pointer: &str, expected: &Value, actual: &Value) -> Result<(), String> {
    let fields = actual.as_object().ok_or("omits requires an object")?;
    for name in array(pointer, "omits", expected)? {
        let name = name.as_str().ok_or("omits requires field names")?;
        if fields.contains_key(name) {
            return Err(format!("{pointer} omits: unexpected field {name}"));
        }
    }
    Ok(())
}

impl Assertion<'_> {
    pub(super) fn check(&self, pointer: &str, actual: &Value) -> Result<(), String> {
        match self {
            Self::Equals(expected) => equal(pointer, expected, actual),
            Self::Omits(expected) => omits(pointer, expected, actual),
            Self::Nonincreasing => nonincreasing(pointer, actual),
            Self::Unique => {
                let values = array(pointer, "unique", actual)?;
                for (index, value) in values.iter().enumerate() {
                    if values.iter().take(index).any(|v| v == value) {
                        return Err(format!("{pointer} unique: duplicate member {value}"));
                    }
                }
                Ok(())
            }
            Self::IncludesAllOf(expected) => {
                let values = array(pointer, "includes_all_of", actual)?;
                for value in array(pointer, "includes_all_of", expected)? {
                    if !values.contains(value) {
                        return Err(format!("{pointer} includes_all_of: missing member {value}"));
                    }
                }
                Ok(())
            }
            Self::Excludes(expected) => {
                if !array(pointer, "excludes", actual)?.contains(expected) {
                    Ok(())
                } else {
                    Err(format!(
                        "{pointer} excludes: unexpected member {expected}, actual {actual}"
                    ))
                }
            }
            Self::Includes(expected) => {
                if array(pointer, "includes", actual)?.contains(expected) {
                    Ok(())
                } else {
                    Err(format!(
                        "{pointer} includes: expected member {expected}, actual {actual}"
                    ))
                }
            }
            Self::Count(expected) => equal(
                &format!("{pointer} count"),
                &Value::from(*expected),
                &Value::from(array(pointer, "count", actual)?.len()),
            ),
            Self::RankOf(member, expected) => {
                let rank = array(pointer, "rank_of", actual)?
                    .iter()
                    .position(|value| value.as_str() == Some(member))
                    .map(|index| index + 1);
                match rank {
                    Some(rank) => equal(
                        &format!("{pointer} rank_of {member}"),
                        &Value::from(*expected),
                        &Value::from(rank),
                    ),
                    None => Err(format!(
                        "{pointer} rank_of {member}: expected {expected}, actual <missing> in {actual}"
                    )),
                }
            }
            Self::MaxBytes(limit) => ceiling(
                pointer,
                "max_bytes",
                super::source_terms::size(actual, pointer)?,
                Some(*limit),
            ),
            Self::MaxTokens(limit) => ceiling(
                pointer,
                "max_tokens",
                super::source_terms::size(actual, pointer)?.div_ceil(4),
                Some(*limit),
            ),
        }
    }
}

fn array<'a>(pointer: &str, operator: &str, actual: &'a Value) -> Result<&'a Vec<Value>, String> {
    actual
        .as_array()
        .ok_or_else(|| format!("{pointer} {operator}: expected an array, actual {actual}"))
}

fn equal(pointer: &str, expected: &Value, actual: &Value) -> Result<(), String> {
    if expected == actual {
        Ok(())
    } else {
        Err(format!("{pointer}: expected {expected}, actual {actual}"))
    }
}

fn ceiling(pointer: &str, key: &str, actual: usize, limit: Option<u64>) -> Result<(), String> {
    if let Some(limit) = limit.filter(|limit| actual as u64 > *limit) {
        return Err(format!(
            "{pointer} {key}: expected <= {limit}, actual {actual}"
        ));
    }
    Ok(())
}

fn nonincreasing(pointer: &str, actual: &Value) -> Result<(), String> {
    let numbers: Vec<_> = array(pointer, "nonincreasing", actual)?
        .iter()
        .map(|v| {
            v.as_u64()
                .ok_or_else(|| format!("{pointer} nonincreasing: expected unsigned counts"))
        })
        .collect::<Result<_, _>>()?;
    if numbers.windows(2).any(|pair| pair.first() < pair.get(1)) {
        return Err(format!("{pointer} nonincreasing: count increased"));
    }
    Ok(())
}

// A bounded first-field diagnostic preserves the exact parity failure.
pub(super) fn difference(left: &Value, right: &Value, path: &str) -> Option<String> {
    if left == right {
        return None;
    }
    match (left, right) {
        (Value::Object(a), Value::Object(b)) => {
            for key in a.keys().chain(b.keys()) {
                let key_path = format!("{path}/{}", key.replace('~', "~0").replace('/', "~1"));
                if let Some(diff) = difference(
                    a.get(key).unwrap_or(&Value::Null),
                    b.get(key).unwrap_or(&Value::Null),
                    &key_path,
                ) {
                    return Some(diff);
                }
            }
        }
        (Value::Array(a), Value::Array(b)) if a.len() == b.len() => {
            for (index, (a, b)) in a.iter().zip(b).enumerate() {
                if let Some(diff) = difference(a, b, &format!("{path}/{index}")) {
                    return Some(diff);
                }
            }
        }
        _ => (),
    }
    let brief = |value: &Value| value.to_string().chars().take(120).collect::<String>();
    Some(format!("{path}: CLI={} MCP={}", brief(left), brief(right)))
}
