//! Agent guidance through the isolated executable and filesystem boundary.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};
type Check<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("skill/catalog-and-managed-install", || {
        check().map_err(|error| error.to_string().into())
    })]
}
fn run(env: &Environment, args: &[&str]) -> Check<Vec<u8>> {
    let args = args.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &args).output()?;
    if !output.status.success() {
        return Err(format!("skill command failed: {output:?}").into());
    }
    println!("skill-output {args:?}: {} bytes", output.stdout.len());
    Ok(output.stdout)
}
fn state(env: &Environment, directory: &str, expected: &str) -> Check {
    let bytes = run(env, &["skill", "status", directory])?;
    let value: Value = serde_json::from_slice(&bytes)?;
    if value.get("state") != Some(&json!(expected)) {
        return Err(format!("expected {expected}: {value}").into());
    }
    Ok(())
}
fn check() -> Check {
    let env = Environment::new()?;
    let overview = run(&env, &["skill"])?;
    if overview != run(&env, &["skill", "render"])?
        || !String::from_utf8(overview)?.contains("Report counts are not risk")
    {
        return Err("canonical guide or report-count boundary differs".into());
    }
    let list = String::from_utf8(run(&env, &["skill", "list"])?)?;
    if !list.contains("exact-variant-literature") || list.contains("_TEMPLATE") {
        return Err("catalog omitted released example or exposed template".into());
    }
    if run(&env, &["skill", "1"])? != run(&env, &["skill", "treatment-lookup"])? {
        return Err("released example alias differs".into());
    }
    state(&env, "agent", "missing")?;
    run(&env, &["skill", "install", "agent"])?;
    state(&env, "agent", "current")?;
    let target = env.root().join("agent/skills/biomcp");
    let guide = fs::read(target.join("SKILL.md"))?;
    fs::write(target.join("SKILL.md"), "local change")?;
    state(&env, "agent", "locally_modified")?;
    let unchanged: Value = serde_json::from_slice(&run(&env, &["skill", "install", "agent"])?)?;
    if unchanged.get("changed") != Some(&json!(false)) {
        return Err("unforced install changed an existing tree".into());
    }
    fs::write(target.join("personal.txt"), "keep")?;
    run(&env, &["skill", "install", "agent", "--force"])?;
    if fs::read(target.join("SKILL.md"))? != guide
        || fs::read_to_string(target.join("personal.txt"))? != "keep"
    {
        return Err("repair lost guide or unrelated file".into());
    }
    let stamp = target.join(".biomcp-skill.json");
    let mut manifest: Value = serde_json::from_slice(&fs::read(&stamp)?)?;
    *manifest
        .get_mut("biomcp_version")
        .ok_or("version missing")? = json!("0.9.3");
    fs::write(&stamp, serde_json::to_vec(&manifest)?)?;
    state(&env, "agent", "stale")?;
    fs::remove_file(&stamp)?;
    state(&env, "agent", "unmanaged")?;
    boundary(&env, &target)?;
    Ok(())
}
fn refused(env: &Environment, args: &[&str]) -> Check {
    let args = args.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
    if env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()?
        .status
        .success()
    {
        return Err(format!("unsafe install accepted: {args:?}").into());
    }
    Ok(())
}
fn boundary(env: &Environment, target: &Path) -> Check {
    refused(env, &["skill", "install", "agent/../escape"])?;
    refused(env, &["skill", "install"])?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let outside = env.root().join("outside");
        fs::create_dir(&outside)?;
        fs::write(outside.join("sentinel"), "unchanged")?;
        symlink(&outside, env.root().join("linked"))?;
        refused(env, &["skill", "install", "linked"])?;
        let path = target.join("SKILL.md");
        fs::remove_file(&path)?;
        symlink(outside.join("sentinel"), &path)?;
        refused(env, &["skill", "install", "agent", "--force"])?;
        if fs::read_to_string(outside.join("sentinel"))? != "unchanged"
            || fs::read_dir(&outside)?.count() != 1
        {
            return Err("install escaped the named directory".into());
        }
    }
    Ok(())
}
