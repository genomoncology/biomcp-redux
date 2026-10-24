use super::launch::Environment;
use libtest_mimic::Trial;
use std::{env, fs, path::Path};

type Check = Result<(), String>;

pub(crate) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("runner/rejects-bad-cases", || {
            negatives().map_err(Into::into)
        }),
        Trial::test("runner/selects-and-lists-cases", || {
            selection().map_err(Into::into)
        }),
        Trial::test("runner/isolates-child-environments", || {
            isolation().map_err(Into::into)
        }),
        Trial::test("schema/output-failure", || {
            output_failure().map_err(Into::into)
        }),
    ]
}

fn put(root: &Path, relative: &str, step: &str) -> Check {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(
        path,
        format!(
            "story: Runner contract probe\nsteps:\n  - run: [schema, biodata.source/1]\n{step}\n"
        ),
    )
    .map_err(|error| error.to_string())
}

fn invoke(root: &Path, args: &[&str]) -> Result<std::process::Output, String> {
    let mut arguments = vec![
        "--cases".to_owned(),
        root.display().to_string(),
        "--color".into(),
        "never".into(),
    ];
    arguments.extend(args.iter().map(|value| (*value).to_owned()));
    Environment::new()
        .map_err(|error| error.to_string())?
        .command(
            env::current_exe().map_err(|error| error.to_string())?,
            &arguments,
        )
        .output()
        .map_err(|error| error.to_string())
}

fn negatives() -> Check {
    let directory = tempfile::tempdir().map_err(|error| error.to_string())?;
    let cases = [
        (
            "wrong-value",
            "    expect: {'/type': integer}",
            "/type: expected \"integer\", actual \"string\"",
        ),
        (
            "missing-null",
            "    expect: {'/absent': null}",
            "/absent: expected null, actual <missing>",
        ),
        (
            "missing-ceiling",
            "    expect: {'/absent': {max_bytes: 10}}",
            "actual <missing>",
        ),
        (
            "wrong-exit",
            "    exit: 2",
            "exit: expected 2, actual Some(0)",
        ),
        (
            "whole-bytes",
            "    max_bytes: 0",
            "stdout max_bytes: expected <= 0, actual",
        ),
        (
            "whole-tokens",
            "    max_tokens: 0",
            "stdout max_tokens: expected <= 0, actual",
        ),
        (
            "pointer-bytes",
            "    expect: {'/type': {max_bytes: 0}}",
            "/type max_bytes: expected <= 0, actual 8",
        ),
        (
            "pointer-tokens",
            "    expect: {'/type': {max_tokens: 1}}",
            "/type max_tokens: expected <= 1, actual 2",
        ),
    ];
    for (name, step, diagnostic) in cases {
        let filename = format!("{name}.yaml");
        put(directory.path(), &filename, step)?;
        let output = invoke(directory.path(), &[&filename, "--exact"])?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        if output.status.success()
            || !stdout.contains(&format!("test {filename} ... FAILED"))
            || !stdout.contains(diagnostic)
            || !stdout.contains("1 failed")
        {
            return Err(format!(
                "{name}: expected a selected failing case with {diagnostic}; actual {stdout}; stderr {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }
    Ok(())
}

fn selection() -> Check {
    let directory = tempfile::tempdir().map_err(|error| error.to_string())?;
    put(
        directory.path(),
        "left/same.yaml",
        "    expect: {'/type': string}",
    )?;
    put(
        directory.path(),
        "right/same.yaml",
        "    expect: {'/type': integer}",
    )?;
    let listed = invoke(directory.path(), &["same.yaml", "--list"])?;
    let listing = String::from_utf8_lossy(&listed.stdout);
    if !listed.status.success()
        || !listing.contains("left/same.yaml: test")
        || !listing.contains("right/same.yaml: test")
    {
        return Err(format!(
            "distinct path names missing from --list: {listing}"
        ));
    }
    let output = invoke(directory.path(), &["left/same.yaml", "--exact"])?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !output.status.success()
        || !stdout.contains("left/same.yaml ... ok")
        || !stdout.contains("1 passed")
        || !stdout.contains("1 filtered out")
    {
        return Err(format!(
            "--cases with Cargo filter did not select the passing case: {stdout}"
        ));
    }
    let filtered = invoke(directory.path(), &["left/", "--list"])?;
    let stdout = String::from_utf8_lossy(&filtered.stdout);
    if !filtered.status.success()
        || !stdout.contains("left/same.yaml")
        || stdout.contains("right/same.yaml")
    {
        return Err(format!("--cases with filtered --list: {stdout}"));
    }
    Ok(())
}

fn isolation() -> Check {
    let before: Vec<_> = env::vars_os().collect();
    let first = Environment::new().map_err(|error| error.to_string())?;
    let second = Environment::new().map_err(|error| error.to_string())?;
    let read = |environment: &Environment| -> Result<String, String> {
        let output = environment
            .command("/usr/bin/env", &[])
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() {
            return Err("environment child failed".into());
        }
        String::from_utf8(output.stdout).map_err(|error| error.to_string())
    };
    let first_step = read(&first)?;
    let next_step = read(&first)?;
    let other_case = read(&second)?;
    for key in ["HOME", "USERPROFILE", "XDG_CONFIG_HOME", "XDG_CACHE_HOME"] {
        let field = |text: &str| {
            text.lines()
                .find(|line| line.starts_with(&format!("{key}=")))
                .map(str::to_owned)
        };
        let first = field(&first_step).ok_or_else(|| format!("child lacks {key}"))?;
        if Some(first.clone()) != field(&next_step) || Some(first) == field(&other_case) {
            return Err(format!(
                "{key}: case steps must share environment; cases must differ"
            ));
        }
    }
    if before != env::vars_os().collect::<Vec<_>>() {
        return Err("runner changed process environment".into());
    }
    Ok(())
}

fn output_failure() -> Check {
    // A read-only stdout handle fails every write without a pipe scheduling race.
    let file = tempfile::NamedTempFile::new().map_err(|error| error.to_string())?;
    let read_only = fs::File::open(file.path()).map_err(|error| error.to_string())?;
    let output = Environment::new()
        .map_err(|error| error.to_string())?
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &["schema".into(), "biodata.source/1".into()],
        )
        .stdout(read_only)
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.code() != Some(5) || output.stderr != b"biomcp: output failed\n" {
        return Err(format!(
            "output failure: expected exit 5 and fixed diagnostic, actual {:?} {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}
