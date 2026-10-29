use super as runner;
use std::path::Path;

pub(crate) fn disease_native(path: &Path) -> Result<(), String> {
    runner::disease_ols::native(path)
}
pub(crate) fn mydisease_native(path: &Path) -> Result<(), String> {
    runner::disease_mydisease::native(path)
}
pub(crate) fn seer_native(path: &Path) -> Result<(), String> {
    runner::disease_seer::native(path)
}
pub(crate) fn trial_condition_native(path: &Path) -> Result<(), String> {
    runner::trial_condition::native(path)
}
pub(crate) fn monarch_native(path: &Path) -> Result<(), String> {
    runner::disease_monarch::native(path)
}
pub(crate) fn faers_search_examples(directory: &std::path::Path) -> Result<(), String> {
    runner::faers_search::record_examples(directory)
}
pub(crate) fn phenotype_native(path: &Path) -> Result<(), String> {
    runner::phenotype::native(path)
}
pub(crate) fn protein_native(path: &Path) -> Result<(), String> {
    runner::protein_commands::native(path)
}

/// Native-only ceilings use the same YAML parser and copied fixture root as cases.
pub(crate) fn budget(key: &str) -> Result<usize, String> {
    let limits: std::collections::BTreeMap<String, usize> =
        serde_saphyr::from_str(&runner::launch::fixture_text("native-proof-budgets.yaml")?)
            .map_err(|error| error.to_string())?;
    limits
        .get(key)
        .copied()
        .ok_or_else(|| format!("unmeasured native proof: {key}"))
}
