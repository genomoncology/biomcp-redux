use super::Step;
use serde_json::Value;

#[derive(Debug)]
pub(super) enum Assertion<'a> {
    Equals(&'a Value),
    Includes(&'a Value),
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
            let Some(options) = value.as_object() else {
                return Ok((pointer.as_str(), value, vec![Assertion::Equals(value)]));
            };
            if options.is_empty() {
                return Err(format!("{pointer}: assertion object requires an operator"));
            }
            let mut assertions = Vec::new();
            for (operator, expected) in options {
                match operator.as_str() {
                    "equals" => assertions.push(Assertion::Equals(expected)),
                    "includes" => assertions.push(Assertion::Includes(expected)),
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
    ceiling(
        "stdout",
        "max_bytes",
        stdout.len(),
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
        size(&value)?.div_ceil(4),
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
        for node in nodes {
            if token == "*" {
                wildcard = true;
                next.extend(
                    node.as_array()
                        .ok_or_else(|| format!("{node}; wildcard requires an array"))?,
                );
            } else {
                next.push(node.pointer(&format!("/{token}")).ok_or("<missing>")?);
            }
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

impl Assertion<'_> {
    pub(super) fn check(&self, pointer: &str, actual: &Value) -> Result<(), String> {
        match self {
            Self::Equals(expected) => equal(pointer, expected, actual),
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
            Self::MaxBytes(limit) => ceiling(pointer, "max_bytes", size(actual)?, Some(*limit)),
            Self::MaxTokens(limit) => ceiling(
                pointer,
                "max_tokens",
                size(actual)?.div_ceil(4),
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

fn size(value: &Value) -> Result<usize, String> {
    serde_json::to_vec(value)
        .map(|bytes| bytes.len())
        .map_err(|error| error.to_string())
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
