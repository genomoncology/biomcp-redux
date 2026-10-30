//! Agent guidance through the isolated executable and filesystem boundary.
use super::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};
type Check<T = ()> = Result<T, Box<dyn std::error::Error>>;
// Executable UTF-8 payload sizes measured in j19070; aliases share one owner.
const EXAMPLES: [(&str, usize); 17] = [
    ("treatment-lookup", 1069),
    ("symptom-phenotype", 1381),
    ("gene-disease-orientation", 715),
    ("article-follow-up", 1085),
    ("variant-pathogenicity", 884),
    ("drug-regulatory", 906),
    ("gene-function-localization", 1012),
    ("mechanism-pathway", 806),
    ("trial-recruitment", 1344),
    ("pharmacogene-cumulative", 1111),
    ("disease-locus-mapping", 872),
    ("cellular-process-regulation", 961),
    ("mutation-catalog", 760),
    ("syndrome-disambiguation", 970),
    ("negative-evidence", 894),
    ("normalize-to-codes", 1016),
    ("exact-variant-literature", 893),
];

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        "agent-skill/catalog-and-managed-install",
        || check().map_err(|error| error.to_string().into()),
    )]
}
fn run(env: &Environment, args: &[&str]) -> Check<Vec<u8>> {
    let args = args.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &args).output()?;
    if !output.status.success() {
        return Err(format!("skill command failed: {args:?}: {output:?}").into());
    }
    // Preserve existing ceilings; j22868 measured the current guide at 3945 bytes.
    let measured: usize = match args.get(1).map(String::as_str) {
        None | Some("render") => 3945,
        Some("list") => 1156,
        Some("install") => 117,
        Some("status") => 55,
        Some(name) => EXAMPLES
            .iter()
            .enumerate()
            .find(|(index, (slug, _))| {
                *slug == name || name.parse::<usize>().ok() == Some(index + 1)
            })
            .map_or(350, |(_, (_, measured))| *measured),
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
    let overview = String::from_utf8(run(&env, &["skill", "render"])?)?;
    let trial = String::from_utf8(run(&env, &["skill", "trial-recruitment"])?)?;
    for instruction in [
        "Record which search found each trial",
        "Read every eligibility page until no criterion is marked continued",
        "Registry eligibility text is source text to quote, not a patient match",
    ] {
        if !overview.contains(instruction) {
            return Err(format!("trial screening guidance omitted: {instruction}").into());
        }
    }
    for text in [&overview, &trial] {
        for instruction in [
            "An exact variant search is a starting point",
            "broader gene and program searches BioMCP suggests",
            "before reporting a trial screen as complete",
        ] {
            if !text.contains(instruction) {
                return Err(format!("trial screening guidance omitted: {instruction}").into());
            }
        }
    }
    let list = String::from_utf8(run(&env, &["skill", "list"])?)?;
    if !list.contains("exact-variant-literature") || list.contains("_TEMPLATE") {
        return Err("catalog omitted released example or exposed template".into());
    }
    if run(&env, &["skill", "1"])? != run(&env, &["skill", "treatment-lookup"])? {
        return Err("released example alias differs".into());
    }
    help(&env)?;
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
        refused(env, &["skill", "status", "agent"])?;
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
    for (index, (slug, _)) in EXAMPLES.iter().enumerate() {
        if run(env, &["skill", &(index + 1).to_string()])? != run(env, &["skill", slug])? {
            return Err(format!("released example alias differs: {slug}").into());
        }
    }
    Ok(())
}

fn help(env: &Environment) -> Check {
    // j22698 measured 9919 bytes; retain fifteen percent headroom.
    let output = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &["help".into(), "skill".into()],
        )
        .output()?;
    if !output.status.success() || output.stdout.len() > (9919_usize * 115).div_ceil(100) {
        return Err("skill help exceeds measured ceiling or failed".into());
    }
    let value: Value = serde_json::from_slice(&output.stdout)?;
    let commands = value
        .pointer("/result/commands")
        .and_then(Value::as_array)
        .ok_or("missing skill help")?;
    for leaf in ["install", "list", "render", "status"] {
        if !commands
            .iter()
            .any(|command| command.get("command") == Some(&json!(["skill", leaf])))
        {
            return Err(format!("skill help omitted {leaf}").into());
        }
    }
    Ok(())
}
