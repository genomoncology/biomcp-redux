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
