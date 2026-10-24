use serde_json::Value;

pub(super) fn validate(_value: &Value, _args: &[String], _success: bool) -> Result<(), String> {
    Ok(())
}
