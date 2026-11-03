//! Repeated outline titles describe a run of parts, so print each run once.
use serde_json::Value;

pub(super) fn parts(value: &Value, depth: usize, out: &mut String) -> bool {
    let Some(items) = value.as_array() else {
        return false;
    };
    if !items.windows(2).any(|pair| {
        pair[0].get("title").is_some_and(Value::is_string)
            && pair[0].get("title") == pair[1].get("title")
    }) {
        return false;
    }
    let mut start = 0;
    while start < items.len() {
        let title = items[start].get("title");
        let end = items[start..]
            .iter()
            .position(|item| item.get("title") != title)
            .map_or(items.len(), |offset| start + offset);
        let mut rows = items[start..end].to_vec();
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
        start = end;
    }
    true
}
