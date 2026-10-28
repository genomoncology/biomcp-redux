//! Feature local-file-persistence: real CLI files in isolated homes contain only synthetic data.
use super::{key_registration::invoke, launch::Environment};
use std::fs;
type Check = Result<(), String>;

pub(super) fn ordered(linked: bool) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let target = env.root().join("target.json");
    let link = env.root().join("config/biomcp/config.json");
    let original = concat!(
        r#"{"ncbi_email":"synthetic@example.invalid","format":"json","keys":{"#,
        r#""semantic_scholar_api_key":"synthetic-registration-first","#,
        r#""ncbi_api_key":"synthetic-registration-first"},"disable_sources":["reactome"]}"#,
    );
    fs::write(&target, original).map_err(|e| e.to_string())?;
    fs::create_dir_all(link.parent().ok_or("parent")?).map_err(|e| e.to_string())?;
    if linked {
        #[cfg(unix)]
        std::os::unix::fs::symlink("../../target.json", &link).map_err(|e| e.to_string())?;
    } else {
        fs::copy(&target, &link).map_err(|e| e.to_string())?;
    }
    invoke(
        &env,
        &["key", "set", "ncbi", "--replace"],
        "synthetic-registration-second\n",
        0,
    )?;
    if linked
        && !fs::symlink_metadata(&link)
            .map_err(|e| e.to_string())?
            .file_type()
            .is_symlink()
    {
        return Err("key set replaced the config symlink".into());
    }
    let updated =
        fs::read_to_string(if linked { &target } else { &link }).map_err(|e| e.to_string())?;
    if !updated.contains("synthetic-registration-second") {
        return Err("key set did not update the selected target".into());
    }
    let fields = [
        "ncbi_email",
        "format",
        "keys",
        "semantic_scholar_api_key",
        "ncbi_api_key",
        "disable_sources",
    ];
    let positions = fields
        .iter()
        .map(|field| updated.find(&format!("\"{field}\"")))
        .collect::<Option<Vec<_>>>()
        .ok_or("lost config field")?;
    if !positions
        .iter()
        .zip(positions.iter().skip(1))
        .all(|(left, right)| left < right)
    {
        return Err("key set reordered config fields".into());
    }
    Ok(())
}

pub(super) fn modes() -> Check {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut failures = Vec::new();
        for existing in [false, true] {
            let env = Environment::new().map_err(|e| e.to_string())?;
            let folder = env.root().join("cache/biomcp");
            if existing {
                fs::create_dir(&folder).map_err(|e| e.to_string())?;
                fs::set_permissions(&folder, fs::Permissions::from_mode(0o755))
                    .map_err(|e| e.to_string())?;
            }
            let args = ["store", "clean", "--offline"].map(String::from);
            let output = env
                .command(env!("CARGO_BIN_EXE_biomcp"), &args)
                .output()
                .map_err(|e| e.to_string())?;
            if !output.status.success() {
                return Err("offline store clean failed".into());
            }
            let mode = |path: &std::path::Path| -> Result<u32, String> {
                Ok(fs::metadata(path)
                    .map_err(|e| e.to_string())?
                    .permissions()
                    .mode()
                    & 0o777)
            };
            if mode(&folder)? != if existing { 0o755 } else { 0o700 } {
                failures.push("cache folder mode changed or is not owner-only");
            }
            let database = fs::read_dir(&folder)
                .map_err(|e| e.to_string())?
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .find(|path| path.extension().is_some_and(|ext| ext == "sqlite"))
                .ok_or("cache database missing")?;
            if mode(&database)? != 0o600 {
                failures.push("cache database is not owner-only");
            }
        }
        if !failures.is_empty() {
            return Err(failures.join("; "));
        }
    }
    Ok(())
}
