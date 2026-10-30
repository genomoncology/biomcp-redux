//! Feature schema-discovery: native proof output survives job scratch cleanup.
use super::super::launch::Environment;
use std::{fs, path::PathBuf};

#[expect(
    clippy::disallowed_methods,
    reason = "The retention proof invokes its runner with isolated child settings"
)]
pub(super) fn check() -> Result<(), Box<dyn std::error::Error>> {
    for explicit in [false, true] {
        let env = Environment::new()?;
        let scratch = env.root().join("job-tmp");
        fs::create_dir(&scratch)?;
        let args = vec![
            "--cases".to_owned(),
            super::super::launch::fixtures().join("cases").display().to_string(),
            "--exact".to_owned(),
            crate::runner::pool_contract::DRUG_CIVIC.0.to_owned(),
        ];
        let mut command = env.command(std::env::current_exe()?, &args);
        command
            .env("TMPDIR", &scratch)
            .env("TMP", &scratch)
            .env("TEMP", &scratch);
        let expected = if explicit {
            let override_path = env.root().join("explicit-evidence");
            command.env("BIOMCP_EVIDENCE_DIR", &override_path);
            env.root()
                .canonicalize()?
                .join("explicit-evidence/fixture-failures")
        } else {
            command.env_remove("BIOMCP_EVIDENCE_DIR");
            super::super::launch::checkout()
                .join("../..")
                .canonicalize()?
                .join("target/fixture-evidence")
        };
        let output = command.output()?;
        if !output.status.success() {
            return Err(format!("native proof child failed: {output:?}").into());
        }
        let stdout = std::str::from_utf8(&output.stdout)?;
        let retained = PathBuf::from(
            stdout
                .lines()
                .find_map(|line| line.strip_prefix("NATIVE_DRUG_NEXT_PROOF="))
                .ok_or("native proof omitted retained evidence")?,
        );
        let mut before = Vec::new();
        for stream in ["evidence", "assertions"] {
            for file in ["cli.stdout", "mcp.stdout"] {
                let path = retained.join(format!("replay-{stream}/{file}"));
                let bytes = fs::read(&path)?;
                if bytes.is_empty() {
                    return Err(format!("native proof omitted output bytes: {path:?}").into());
                }
                before.push((path, bytes));
            }
        }
        fs::remove_dir_all(&scratch)?;
        for (path, bytes) in before {
            if fs::read(&path)? != bytes {
                return Err(format!("native evidence changed after scratch cleanup: {path:?}").into());
            }
        }
        if retained.parent() != Some(expected.as_path()) {
            return Err(format!("native evidence used unexpected path: {retained:?}").into());
        }
        fs::remove_dir_all(retained)?;
    }
    Ok(())
}
