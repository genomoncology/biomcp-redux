//! Feature gwas-search: source selectors cross the real command boundary.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::json;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("gwas-search/restricted-policy-before-io", || {
        policy().map_err(Into::into)
    })]
}
fn policy() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let mut args = env.globals(None, true);
    args.extend(["search", "gwas", "--pubmed", "17529967", "--disable-sources", "restricted"].map(str::to_owned));
    let value = invoke(&env, args, 4)?;
    if value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
        return Err(format!("restricted source was not refused: {value}"));
    }
    Ok(())
}
