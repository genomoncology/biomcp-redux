//! Owning synthetic protein case phases; CLI counts its trailing newline.
use serde_json::Value;
pub(super) fn check(value: &Value, case: &str, surface: &str) -> Result<(), String> {
    let measurements: std::collections::BTreeMap<String, usize> = serde_saphyr::from_str(
        &crate::runner::launch::fixture_text("protein-synthetic-budgets.yaml")?,
    )
    .map_err(|e| e.to_string())?;
    let key = format!("{case} [{surface}]");
    let measured = measurements
        .get(&key)
        .ok_or_else(|| format!("unmeasured protein phase: {key}"))?;
    let bytes =
        serde_json::to_vec(value).map_err(|e| e.to_string())?.len() + usize::from(surface == "cli");
    let max_bytes = (measured * 115).div_ceil(100);
    if bytes > max_bytes {
        return Err(format!(
            "protein {key}: {bytes} bytes exceeds max_bytes={max_bytes}"
        ));
    }
    Ok(())
}

/// Keep one mutation fixture and environment across its checked command phases.
pub(super) fn steps(
    variant: &str,
    capture: &str,
    change: impl FnMut(&str, &mut Value),
    steps: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    let replay = root.path().join("replay");
    super::response_cases::copy_capture(capture, &replay, change)?;
    let env = super::launch::Environment::new()?;
    let registry = super::schema::Registry::embedded()?;
    let steps: Vec<super::Step> = serde_json::from_value(steps)?;
    let mut previous = None;
    for (index, step) in steps.iter().enumerate() {
        let value = super::run_sequence(
            step,
            &env,
            &super::assertions::prepare(step)?,
            &registry,
            Some(&replay),
            previous.as_ref(),
        )?;
        check(&value, &format!("{variant}/step-{index}"), "cli")?;
        previous = Some(value);
    }
    previous.ok_or_else(|| "protein case requires command phases".into())
}
