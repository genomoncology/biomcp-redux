//! Feature source-controls: health measures owned files without attributing shared pages.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use std::{fs, path::Path};
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "sources/health-measures-owned-store-files",
        || measured().map_err(Into::into),
    )]
}
fn length(path: &Path) -> Result<u64, String> {
    fs::metadata(path)
        .map(|m| m.len())
        .map_err(|e| e.to_string())
}
fn database_bytes(path: &Path) -> Result<u64, String> {
    let mut bytes = length(path)?;
    for suffix in ["-wal", "-shm"] {
        let sidecar = Path::new(&format!("{}{suffix}", path.display())).to_owned();
        if sidecar.exists() {
            bytes += length(&sidecar)?;
        }
    }
    Ok(bytes)
}
fn measured() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = vec!["health".into(), "--offline".into()];
    invoke(&env, args.clone(), 0)?;
    let folder = env.root().join("cache/biomcp");
    let active = folder.join("cache-1-5.sqlite");
    let retained = folder.join("cache-1-4.sqlite");
    let legacy = folder.join("cache-1-3.sqlite");
    fs::copy(&active, &legacy).map_err(|e| e.to_string())?;
    let legacy_db = rusqlite::Connection::open(&legacy).map_err(|e| e.to_string())?;
    legacy_db
        .execute("UPDATE meta SET version=1 WHERE owner='biomcp-fetch'", [])
        .map_err(|e| e.to_string())?;
    fs::copy(&active, &retained).map_err(|e| e.to_string())?;
    let db = rusqlite::Connection::open(&active).map_err(|e| e.to_string())?;
    db.execute("INSERT INTO bundle VALUES('hgnc',10,1,0,NULL,NULL)", [])
        .map_err(|e| e.to_string())?;
    let retained_db = rusqlite::Connection::open(&retained).map_err(|e| e.to_string())?;
    retained_db
        .execute("INSERT INTO bundle VALUES('hgnc',10,1,0,NULL,NULL)", [])
        .map_err(|e| e.to_string())?;
    for (path, bytes) in [
        (active.with_extension("parts"), 7),
        (retained.with_extension("parts"), 11),
        (legacy.with_extension("parts"), 13),
    ] {
        fs::create_dir(&path).map_err(|e| e.to_string())?;
        fs::write(path.join(".tmpABC123"), vec![0; bytes]).map_err(|e| e.to_string())?;
    }
    fs::write(folder.join("unrelated"), vec![0; 99]).map_err(|e| e.to_string())?;
    let active_bytes = database_bytes(&active)?;
    let retained_bytes = database_bytes(&retained)? + database_bytes(&legacy)?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "store file bytes: active={active_bytes} retained={retained_bytes} partial=31"
    )
    .map_err(|e| e.to_string())?;
    let reply = invoke(&env, args, 0)?;
    let store = &reply["result"]["store"];
    if store["active_bytes"] != active_bytes
        || store["retained_bytes"] != retained_bytes
        || store["partial_download_bytes"] != 31
        || store["total_bytes"] != active_bytes + retained_bytes + 31
    {
        return Err(format!("incorrect measured storage: {store}"));
    }
    let hgnc = reply["result"]["sources"]
        .as_array()
        .and_then(|rows| rows.iter().find(|r| r["source"] == "hgnc"))
        .ok_or("missing HGNC")?;
    if hgnc["bundle_size"] != "shared_storage" || hgnc["bundle"] != "missing" {
        return Err(format!(
            "health attributed shared bytes or synchronized: {hgnc}"
        ));
    }
    if reply
        .to_string()
        .contains(&env.root().to_string_lossy().to_string())
    {
        return Err("health disclosed local paths".into());
    }
    Ok(())
}
