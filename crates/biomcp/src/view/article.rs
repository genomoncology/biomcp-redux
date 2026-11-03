//! Repeated outline titles describe a run of parts, so print each run once.
use serde_json::Value;

pub(super) fn parts(value: &Value, depth: usize, out: &mut String) -> bool {
    let Some(items) = value.as_array() else {
        return false;
    };
    if !items.windows(2).any(|pair| {
        let mut values = pair.iter().map(|item| item.get("title"));
        let title = values.next().flatten();
        title.is_some_and(Value::is_string) && title == values.next().flatten()
    }) {
        return false;
    }
    let mut remaining = items.as_slice();
    while let Some(first) = remaining.first() {
        let title = first.get("title");
        let end = remaining
            .iter()
            .position(|item| item.get("title") != title)
            .unwrap_or(remaining.len());
        let (group, rest) = remaining.split_at(end);
        let mut rows = group.to_vec();
        if let Some(title) = title.and_then(Value::as_str) {
            out.push_str(&format!(
                "{} {}\n\n",
                "#".repeat(depth.min(6)),
                super::escape::escape(title)
            ));
            for row in &mut rows {
                if let Some(fields) = row.as_object_mut() {
                    fields.remove("title");
                }
            }
        }
        super::table(&rows, &[], false, out);
        remaining = rest;
    }
    true
}
