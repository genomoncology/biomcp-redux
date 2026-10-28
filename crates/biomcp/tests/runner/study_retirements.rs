//! Feature study-retirements: legacy calculations and charts fail before source access.
use crate::runner::transcript::trial as transcript_trial;
use crate::runner::{configuration::invoke, launch::Environment, mcp};
use libtest_mimic::Trial;
use serde_json::{Value, json};
const LIST: &str = "use: search study";
const DOWNLOAD: &str = "use: sync cbioportal-studies for metadata; get study ID files for source file references; whole-study analytical installs are retired";
const CALCULATE: &str = "local calculations are dropped by the entity boundary ruling; use: get study ID and the source study page";
const CHART: &str = "charts are retired";
const DIRECTORY: &str = "BIOMCP_STUDY_DIR is retired; use: sync cbioportal-studies for metadata";
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    let mut cases = vec![("study list".into(), LIST), ("list study".into(), LIST)];
    cases.extend(
        [
            "study download",
            "study download --list",
            "study download synthetic_0",
        ]
        .map(|command| (command.into(), DOWNLOAD)),
    );
    for (leaf, flags) in LEAVES {
        cases.push((format!("study {leaf}"), CALCULATE));
        for flag in *flags {
            cases.push((format!("study {leaf} {flag} synthetic"), CALCULATE));
        }
    }
    for (leaf, flag, values) in VALUES {
        for value in *values {
            cases.push((format!("study {leaf} {flag} {value}"), CALCULATE));
        }
    }
    for leaf in ["query", "survival", "compare", "co-occurrence"] {
        for flag in [
            "--chart",
            "--terminal",
            "--output",
            "-o",
            "--title",
            "--theme",
            "--palette",
            "--cols",
            "--rows",
            "--width",
            "--height",
            "--scale",
            "--mcp-inline",
        ] {
            cases.push((format!("study {leaf} {flag} synthetic"), CHART));
        }
    }
    let mut trials: Vec<_> = cases
        .into_iter()
        .enumerate()
        .map(|(index, (args, message))| {
            transcript_trial(
                format!("study-retirements/fixed-guidance-{index}"),
                move || check(&args, message).map_err(Into::into),
            )
        })
        .collect();
    trials.push(transcript_trial(
        "study-retirements/environment-does-not-read-directory",
        || directory().map_err(Into::into),
    ));
    trials.push(transcript_trial("study-retirements/argument-boundaries", || {
        boundaries().map_err(Into::into)
    }));
    trials
}
fn boundaries() -> Check {
    check("--format json study list", LIST)?;
    check("--disable-sources cbioportal study list", LIST)?;
    check("study query --gene=--chart", CALCULATE)?;
    check("study query --gene --chart", CALCULATE)?;
    check("study query -- --chart", CALCULATE)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = super::seed(&env, super::rows(), 200)?;
    super::call(
        &env,
        &replay,
        &["sync", "cbioportal-studies"],
        0,
        "retirements/boundaries/1",
    )?;
    let value = super::call(
        &env,
        &replay,
        &["search", "study", "--", "--chart"],
        0,
        "retirements/boundaries/2",
    )?;
    if value.get("status") != Some(&json!("complete"))
        || value.pointer("/page/total") != Some(&json!(0))
    {
        return Err(format!("literal study text became a chart flag: {value}"));
    }
    Ok(())
}
fn check(command: &str, message: &str) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    env.config(
        &json!({"proxy":format!("http://{}",listener.local_addr().map_err(|e|e.to_string())?)}),
    )
    .map_err(|e| e.to_string())?;
    let args: Vec<_> = command.split_whitespace().map(str::to_owned).collect();
    let value = invoke(&env, args.clone(), 2)?;
    guidance(&value, message)?;
    if mcp::call_with_globals(&env, &args, true, &[])? != value {
        return Err(format!("CLI/MCP migration differs: {command}"));
    }
    if listener.accept().is_ok() {
        return Err("retired study command opened a connection".into());
    }
    Ok(())
}
fn guidance(value: &Value, message: &str) -> Check {
    if value.pointer("/error/code") != Some(&json!("usage.retired"))
        || value.pointer("/error/message") != Some(&json!(message))
        || value.get("sources").is_some()
        || value.get("result").is_some()
    {
        return Err(format!("wrong fixed study guidance: {value}"));
    }
    // Exact fixed responses have no varying payload; fifteen percent allows wire overhead.
    let expected = json!({"schema":"biomcp.error/1","status":"usage","error":{"code":"usage.retired","message":message}});
    let baseline = serde_json::to_vec(&expected)
        .map_err(|e| e.to_string())?
        .len();
    if serde_json::to_vec(value).map_err(|e| e.to_string())?.len() > (baseline * 115).div_ceil(100)
    {
        return Err("study guidance exceeds its fixed envelope ceiling".into());
    }
    Ok(())
}
fn directory() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    // A regular file cannot be scanned as a directory. Its bytes are not study metadata.
    let path = env.root().join("not-a-directory");
    std::fs::write(&path, b"must not read or parse").map_err(|e| e.to_string())?;
    let output = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &["search".into(), "study".into(), "atlas".into()],
        )
        .env("BIOMCP_STUDY_DIR", &path)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(2) || !output.stderr.is_empty() {
        return Err("legacy directory did not return local migration guidance".into());
    }
    let value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    guidance(&value, DIRECTORY)
}
const LEAVES: &[(&str, &[&str])] = &[
    ("query", &["--study", "-s", "--gene", "-g", "--type", "-t"]),
    ("top-mutated", &["--study", "-s", "--limit", "-l"]),
    (
        "filter",
        &[
            "--study",
            "-s",
            "--mutated",
            "--amplified",
            "--deleted",
            "--expression-above",
            "--expression-below",
            "--cancer-type",
        ],
    ),
    ("cohort", &["--study", "-s", "--gene", "-g"]),
    (
        "survival",
        &["--study", "-s", "--gene", "-g", "--endpoint", "-e"],
    ),
    (
        "compare",
        &["--study", "-s", "--gene", "-g", "--type", "-t", "--target"],
    ),
    ("co-occurrence", &["--study", "-s", "--genes", "-g"]),
];
const VALUES: &[(&str, &str, &[&str])] = &[
    (
        "query",
        "--type",
        &[
            "mutation",
            "mutations",
            "cna",
            "copy_number",
            "copy-number",
            "expression",
            "expr",
            "sv",
            "fusion",
            "fusions",
            "structural_variants",
        ],
    ),
    (
        "survival",
        "--endpoint",
        &[
            "os",
            "overall",
            "overall_survival",
            "dfs",
            "disease_free",
            "pfs",
            "progression_free",
            "dss",
            "disease_specific",
        ],
    ),
    (
        "compare",
        "--type",
        &["expression", "expr", "mutations", "mutation"],
    ),
    ("compare", "--target", &["TP53"]),
];
