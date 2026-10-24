use std::{fs, io, path::Path, process::Command};

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
            .env("HOME", self.root.path().join("home"))
            .env("USERPROFILE", self.root.path().join("home"))
            .env("XDG_CONFIG_HOME", self.root.path().join("config"))
            .env("XDG_CACHE_HOME", self.root.path().join("cache"))
            .current_dir(self.root.path());
        command
    }
}
