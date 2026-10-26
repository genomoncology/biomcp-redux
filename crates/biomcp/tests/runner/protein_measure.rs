//! Temporary bounded inventory runner; removed after its one measurement job.
use super::*;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("protein-docs-accounting/measure-yaml", || {
        measure().map_err(Into::into)
    })]
}
fn measure() -> Result<(), String> {
    let root = launch::fixtures().join("cases/protein");
    let registry = schema::Registry::embedded()?;
    let mut files = Vec::new();
    collect(&root, &mut files)?;
    files.sort();
    for path in files {
        let case: Case =
            serde_saphyr::from_str(&fs::read_to_string(&path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let env = launch::Environment::new().map_err(|e| e.to_string())?;
        if let Some(config) = &case.config {
            env.config(config).map_err(|e| e.to_string())?;
        }
        let replay = case
            .replay
            .as_ref()
            .map(|p| launch::fixtures().join("replies").join(p));
        for (index, step) in case.steps.iter().enumerate() {
            let name = path
                .strip_prefix(&root)
                .map_err(|e| e.to_string())?
                .to_string_lossy();
            measure_step(
                &env,
                &registry,
                replay.as_deref(),
                step,
                &format!("yaml/{name}/step-{index}"),
            )?;
        }
    }
    Ok(())
}
fn measure_step(
    env: &launch::Environment,
    registry: &schema::Registry,
    replay: Option<&Path>,
    step: &Step,
    key: &str,
) -> Result<(), String> {
    let mut current = step.clone();
    current.walk = None;
    let mut page = 0;
    loop {
        let mut last = None;
        for surface in ["cli", "mcp"] {
            let mut one = current.clone();
            if surface == "cli" {
                one.mcp = None;
            } else {
                one.run = None;
            }
            if one.run.is_none() && one.mcp.is_none() {
                continue;
            }
            let value = run_step(&one, env, &Vec::new(), registry, replay)?;
            super::protein_budgets::check(&value, &format!("{key}/page-{page}"), surface)?;
            last = Some(value);
        }
        let value = last.ok_or("no output")?;
        if step.walk.is_none() {
            return Ok(());
        }
        let Ok(next) = action(&value, "page") else {
            return Ok(());
        };
        current.run = Some(next);
        current.mcp = None;
        page += 1;
        if page >= step.walk.as_ref().ok_or("walk")?.max_pages {
            return Err("measurement walk limit".into());
        }
    }
}
fn collect(root: &Path, files: &mut Vec<std::path::PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.is_dir() {
            collect(&path, files)?;
        } else if path.extension().is_some_and(|x| x == "yaml") {
            files.push(path);
        }
    }
    Ok(())
}
