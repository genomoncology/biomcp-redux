//! Assertions over a selected array or text that look across its members.
use super::array;
use serde_json::Value;

pub(super) fn nonincreasing(pointer: &str, actual: &Value) -> Result<(), String> {
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

/// Every sentence of two or more words in the selected text appears once.
pub(super) fn sentences_once(pointer: &str, actual: &Value) -> Result<(), String> {
    fn texts<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
        match value {
            Value::String(text) => out.push(text),
            Value::Array(items) => items.iter().for_each(|item| texts(item, out)),
            Value::Object(fields) => fields.values().for_each(|field| texts(field, out)),
            _ => {}
        }
    }
    let mut all = Vec::new();
    texts(actual, &mut all);
    let mut seen = std::collections::BTreeSet::new();
    let repeats = all
        .into_iter()
        .flat_map(str::lines)
        .flat_map(|line| line.split_inclusive(". "))
        .map(str::trim)
        .filter(|sentence| sentence.contains(' ') && !seen.insert(*sentence))
        .collect::<std::collections::BTreeSet<_>>();
    if repeats.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{pointer} sentences_once: repeated {}",
            repeats.into_iter().collect::<Vec<_>>().join(" | ")
        ))
    }
}
