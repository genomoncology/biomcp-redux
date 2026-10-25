//! Generate synthetic local recordings through the real transport recorder.
use super::{Check, body, environment};
use std::path::Path;
pub(super) fn record(directory: &Path) -> Check {
    std::fs::create_dir_all(directory.join("faers")).map_err(|e| e.to_string())?;
    let stub = super::Stub::for_sources(
        &["api.fda.gov"],
        |_| "application/json",
        |request| {
            if !request.starts_with("GET /drug/event.json?search=")
                || !request.contains("&limit=5&skip=0&sort=receivedate%3Adesc ")
            {
                return Err("unexpected synthetic example request".into());
            }
            Ok(body(5, 0).to_string())
        },
    )?;
    let env = environment(&stub)?;
    for (index, flags) in [
        vec![],
        vec![
            "--received-from",
            "2024-01-01",
            "--received-to",
            "2024-12-31",
        ],
    ]
    .into_iter()
    .enumerate()
    {
        let words = ["search", "adverse-event", "pembrolizumab"]
            .into_iter()
            .chain(flags)
            .map(str::to_owned);
        let recording = env.root().join(format!("record-{index}"));
        let mut args = vec!["--record".into(), recording.to_string_lossy().into()];
        args.extend(words);
        super::invoke(&env, args, 0)?;
        for entry in std::fs::read_dir(recording.join("faers")).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.path().extension().is_some_and(|s| s == "json") {
                std::fs::copy(
                    entry.path(),
                    directory.join("faers").join(entry.file_name()),
                )
                .map_err(|e| e.to_string())?;
            }
        }
    }
    if std::fs::read_dir(directory.join("faers"))
        .map_err(|e| e.to_string())?
        .count()
        != 2
    {
        return Err("synthetic recordings were not retained".into());
    }
    if stub.finish()?.len() != 2 {
        return Err("synthetic examples did not record two actual requests".into());
    }
    Ok(())
}
