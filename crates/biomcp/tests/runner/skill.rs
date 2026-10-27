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
        return Err(format!("skill command failed: {args:?}: {output:?}").into());
    }
    // Embedded text measured from the canonical UTF-8 payload; summaries are
    // measured at the executable boundary. Keep fifteen percent headroom.
    let measured: usize = match args.get(1).map(String::as_str) {
        None | Some("render") => 1824,
        Some("list") => 955,
        Some("install") => 128,
        Some("status") => 64,
        _ => 350,
    };
    if output.stdout.len() > (measured * 115).div_ceil(100) {
        return Err(format!("skill output exceeds measured ceiling: {args:?}").into());
    }
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
    env.config(&json!({"keys":{"unknown":"never-read"}}))?;
    fs::write(
        env.root().join("config/biomcp/config.json"),
        "invalid config",
    )?;
    let overview = run(&env, &["skill"])?;
    if overview != run(&env, &["skill", "render"])?
        || !String::from_utf8(overview)?
            .contains("Adverse-event report counts do not establish risk")
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
    aliases(&env)?;
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
    manifest_boundary(&env, &target)?;
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

fn manifest_boundary(env: &Environment, target: &Path) -> Check {
    for path in [
        "../sentinel",
        "/sentinel",
        "use-cases/../../sentinel",
        "use-cases\\sentinel",
    ] {
        let stamp = json!({"schema_version":1,"biomcp_version":"1.0.0","managed_files":{path:"a".repeat(64)}});
        fs::write(
            target.join(".biomcp-skill.json"),
            serde_json::to_vec(&stamp)?,
        )?;
        refused(env, &["skill", "status", "agent"])?;
    }
    fs::remove_file(target.join(".biomcp-skill.json"))?;
    Ok(())
}

fn aliases(env: &Environment) -> Check {
    for (index, slug) in [
        "treatment-lookup",
        "symptom-phenotype",
        "gene-disease-orientation",
        "article-follow-up",
        "variant-pathogenicity",
        "drug-regulatory",
        "gene-function-localization",
        "mechanism-pathway",
        "trial-recruitment",
        "pharmacogene-cumulative",
        "disease-locus-mapping",
        "cellular-process-regulation",
        "mutation-catalog",
        "syndrome-disambiguation",
        "negative-evidence",
        "normalize-to-codes",
        "exact-variant-literature",
    ]
    .iter()
    .enumerate()
    {
        if run(env, &["skill", &(index + 1).to_string()])? != run(env, &["skill", slug])? {
            return Err(format!("released example alias differs: {slug}").into());
        }
    }
    Ok(())
}
