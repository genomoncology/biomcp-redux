//! Measure the corrected protein section help without replaying accepted cases.
use super::*;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "protein-docs-accounting/corrected-help",
        || measure().map_err(Into::into),
    )]
}
fn measure() -> Result<(), String> {
    use std::io::Write;
    let env = launch::Environment::new().map_err(|e| e.to_string())?;
    for args in [vec!["help", "get", "protein"], vec!["help"]] {
        let words = args.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
        let cli = configuration::invoke(&env, words.clone(), 0)?;
        let mcp = mcp::call_with_globals(&env, &words, false, &env.globals(None, false))?;
        for (surface, value) in [("cli", &cli), ("mcp", &mcp)] {
            let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len()
                + usize::from(surface == "cli");
            writeln!(
                std::io::stdout(),
                "PROTEIN_CORRECTED_HELP {} {surface} {bytes}",
                args.join(" ")
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
