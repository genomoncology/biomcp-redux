use super::{Environment, Path, Value, fs};
pub(super) fn check(name: &str) -> Result<(), String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../../tests/native/author-controls/{name}.yaml"));
    let case: super::super::Case =
        serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = if name == "stale" {
        let target = env.root().join("known-recordings");
        super::super::response_cases::cache_capture(
            "author/synthetic",
            &target,
            super::super::response_cases::CACHE_ACQUIRED,
        )?;
        target
    } else {
        super::super::launch::fixtures().join("replies/author/synthetic")
    };
    let registry = super::super::schema::Registry::embedded()?;
    let mut pool: Option<String> = None;
    let mut published = None;
    for (index, mut step) in case.steps.into_iter().enumerate() {
        if (name == "pool" && index == 6) || (name == "stale" && index == 1) {
            let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
                .map_err(|e| e.to_string())?;
            db.execute(
                "UPDATE pool SET expires=0 WHERE id=?1",
                [pool.as_deref().ok_or("pool")?],
            )
            .map_err(|e| e.to_string())?;
            if name == "stale" {
                db.execute("UPDATE reply SET fresh_until=0", [])
                    .map_err(|e| e.to_string())?;
            }
        }
        if let Some(pool) = &pool {
            for args in [&mut step.run, &mut step.mcp].into_iter().flatten() {
                for word in args {
                    if word == "<pool>" {
                        *word = pool.clone();
                    }
                }
            }
        }
        let expect = super::super::assertions::prepare(&step)?;
        let value = super::super::run_step(&step, &env, &expect, &registry, Some(&replay))?;
        if index == 0 || (name == "stale" && index == 1) {
            pool = value
                .pointer("/page/pool/id")
                .and_then(Value::as_str)
                .map(String::from);
            published = value.pointer("/page/pool").cloned();
        }
        if (1..=3).contains(&index) {
            for field in ["id", "created", "expires"] {
                if value.pointer(&format!("/page/pool/{field}"))
                    != published.as_ref().and_then(|p| p.get(field))
                {
                    return Err(format!("retained author pool changed {field}"));
                }
            }
        }
    }
    Ok(())
}
