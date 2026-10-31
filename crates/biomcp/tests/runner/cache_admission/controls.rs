//! Positive controls change only the native response body and its output assertions.
use super::Owner;
use crate::runner::Step;
use serde_json::{Value, json};
pub(super) fn prepare(
    owner: &Owner,
    first: &Step,
    control: &str,
    wire: &str,
) -> Result<(Step, String), String> {
    let mut body: Value = serde_json::from_str(wire).map_err(|e| e.to_string())?;
    if owner.string {
        body = serde_json::from_str(body.as_str().ok_or("native JSON-string wrapper")?)
            .map_err(|e| e.to_string())?;
    }
    let mut step = first.clone();
    if control != "refused" {
        step.expect.clear();
        let count = usize::from(control == "optional");
        if count == 0 && owner.count.ends_with("items") {
            let (parent, field) = owner.count.rsplit_once('/').ok_or("native items pointer")?;
            step.expect.insert(parent.into(), json!({"omits":[field]}));
        } else {
            step.expect.insert(
                owner.count.into(),
                if owner.count.ends_with("items") {
                    json!({"count":count})
                } else {
                    json!(count)
                },
            );
        }
        if !owner.total.is_empty() {
            step.expect.insert(owner.total.into(), json!(count));
        }
        if control == "optional" {
            *body
                .pointer_mut(owner.identity)
                .ok_or("accepted native identity slot")? =
                serde_json::from_str(owner.accepted_identity).map_err(|e| e.to_string())?;
            step.expect.insert(
                "/problems/*/field".into(),
                json!({"includes": owner.optional}),
            );
        } else {
            *body.pointer_mut(owner.rows).ok_or("native record array")? = json!([]);
            if !owner.total_field.is_empty() {
                *body.pointer_mut(owner.total_field).ok_or("native total")? = json!(0);
            }
        }
    }
    // SEER retains its explicit dataset cautions; empty metadata study sections return complete.
    if control == "empty" && owner.feature != "disease-seer" && owner.feature != "study-get" {
        step.exit = 1;
    }
    let wire = if owner.string {
        json!(body.to_string()).to_string()
    } else {
        body.to_string()
    };
    Ok((step, wire))
}
