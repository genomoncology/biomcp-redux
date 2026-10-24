use std::{fs, io, path::Path, process::Command};

pub(super) const ENV_PATHS: &[(&str, &str)] = &[
    ("HOME", "home"),
    ("USERPROFILE", "home"),
    ("XDG_CONFIG_HOME", "config"),
    ("XDG_CACHE_HOME", "cache"),
];

#[derive(Debug)]
pub(super) struct Environment {
    root: tempfile::TempDir,
}

impl Environment {
    pub(super) fn new() -> io::Result<Self> {
        let root = tempfile::tempdir()?;
        for directory in ["home", "config", "cache"] {
            fs::create_dir(root.path().join(directory))?;
        }
        Ok(Self { root })
    }

    pub(super) fn config(&self, value: &serde_json::Value) -> io::Result<()> {
        let directory = self.root.path().join("config/biomcp");
        fs::create_dir_all(&directory)?;
        fs::write(directory.join("config.json"), serde_json::to_vec(value)?)
    }
    pub(super) fn globals(&self, replay: Option<&Path>, offline: bool) -> Vec<String> {
        if offline {
            vec![
                "--offline".into(),
                "--case-cache".into(),
                self.root.path().join("case-cache").to_string_lossy().into(),
            ]
        } else if let Some(replay) = replay {
            vec![
                "--replay".into(),
                replay.to_string_lossy().into(),
                "--case-cache".into(),
                self.root.path().join("case-cache").to_string_lossy().into(),
            ]
        } else {
            Vec::new()
        }
    }
    pub(super) fn root(&self) -> &Path {
        self.root.path()
    }

    // All case steps and launch-boundary proofs use this child-only boundary.
    pub(super) fn command(&self, program: impl AsRef<Path>, args: &[String]) -> Command {
        let mut command = Command::new(program.as_ref());
        command
            .args(args)
            .env_clear()
            .envs(
                ENV_PATHS
                    .iter()
                    .map(|(key, path)| (key, self.root.path().join(path))),
            )
            .current_dir(self.root.path());
        if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
            command.env("LLVM_PROFILE_FILE", profile);
        }
        command
    }
}

#[expect(
    clippy::disallowed_methods,
    reason = "Installed checks copy public fixtures outside the checkout"
)]
pub(super) fn fixtures() -> std::path::PathBuf {
    std::env::var_os("BIOMCP_TEST_FIXTURES").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests"),
        std::path::PathBuf::from,
    )
}
