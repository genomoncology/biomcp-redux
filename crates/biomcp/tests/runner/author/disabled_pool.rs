//! Feature author-search: a disabled source cannot restore either retained author snapshot.
use super::*;
pub(super) fn check() -> Result<(), String> {
    for name in ["search", "papers"] {
        let root = super::super::launch::fixtures();
        let path = root.join(format!("cases/author/{name}-disabled-pool.yaml"));
        let case: super::super::Case =
            serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = root.join("replies").join(case.replay.ok_or("pool replay")?);
        let registry = super::super::schema::Registry::embedded()?;
        let mut previous = None;
        let mut pool = None;
        for mut step in case.steps {
            if let Some(kind) = step.follow.take() {
                let args =
                    super::super::action(previous.as_ref().ok_or("pool predecessor")?, &kind)?;
                step.run = Some(args.clone());
                step.mcp = Some(args);
            }
            for args in [&mut step.run, &mut step.mcp].into_iter().flatten() {
                for word in args {
                    if word == "<pool>" {
                        *word = pool.clone().ok_or("pool identity")?;
                    }
                }
            }
            let value = super::super::run_step(
                &step,
                &env,
                &super::super::assertions::prepare(&step)?,
                &registry,
                Some(&replay),
            )?;
            if value.get("status") == Some(&json!("complete")) && pool.is_none() {
                let args = super::super::action(&value, "page")?;
                pool = args
                    .windows(2)
                    .find(|pair| pair.first().is_some_and(|v| v == "--pool"))
                    .and_then(|pair| pair.get(1))
                    .cloned();
            }
            if value.pointer("/sources/0/reason") == Some(&json!("disabled"))
                && value.get("result").is_some_and(|v| !v.is_null())
            {
                return Err("disabled source restored author rows".into());
            }
            previous = Some(value);
        }
    }
    Ok(())
}
