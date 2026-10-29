//! Record ordinary command outputs without replacing child processes or pipes.
use super::transcript::Context;
use serde_json::json;
use std::{
    ffi::OsStr,
    io,
    ops::{Deref, DerefMut},
    path::Path,
    process::{Child, Command, ExitStatus, Output, Stdio},
    sync::Arc,
};

pub(super) struct Capture {
    command: Command,
    context: Option<Arc<Context>>,
    key: Option<String>,
    args: Vec<String>,
    stable_args: Vec<String>,
    spawned: bool,
    executable: String,
    root: String,
    finished: bool,
}
impl Capture {
    pub(super) fn new(
        command: Command,
        context: Option<Arc<Context>>,
        args: &[String],
        root: &Path,
    ) -> Self {
        let stable = args
            .iter()
            .map(|arg| {
                arg.replace(&root.to_string_lossy().to_string(), "<root>")
                    .replace(
                        crate::runner::launch::checkout().to_string_lossy().as_ref(),
                        "<checkout>/crates/biomcp",
                    )
            })
            .collect::<Vec<_>>();
        Self {
            executable: command.get_program().to_string_lossy().into(),
            command,
            context,
            key: None,
            stable_args: stable,
            spawned: false,
            args: args.to_vec(),
            root: root.to_string_lossy().into(),
            finished: false,
        }
    }
    fn prepare(&mut self) {
        self.key = self.context.as_ref().map(|c| c.allocate(&self.stable_args));
        self.spawned = true;
        self.finished = false;
    }
    pub(super) fn spawn(&mut self) -> io::Result<Child> {
        self.prepare();
        self.command.spawn()
    }
    pub(super) fn output(&mut self) -> io::Result<Output> {
        self.prepare();
        let output = self.command.output()?;
        self.record(output.status, &output.stdout, "cli")?;
        Ok(output)
    }
    pub(super) fn wait_with_output(&mut self, child: Child) -> io::Result<Output> {
        let output = child.wait_with_output()?;
        self.record(output.status, &output.stdout, "cli")?;
        Ok(output)
    }
    pub(super) fn record(
        &mut self,
        status: ExitStatus,
        stdout: &[u8],
        surface: &str,
    ) -> io::Result<()> {
        if let (Some(context), Some(key)) = (&self.context, &self.key) {
            context.record(key, json!({"args":self.args,"surface":surface,
                "exit":status.code(),"stdout":stdout,
                "executable":self.executable,"root":self.root,"checkout":crate::runner::launch::checkout().to_string_lossy().as_ref(),"version":env!("CARGO_PKG_VERSION")}))
                .map_err(io::Error::other)?;
        }
        self.finished = true;
        Ok(())
    }
    pub(super) fn stdin(&mut self, value: impl Into<Stdio>) -> &mut Self {
        self.command.stdin(value);
        self
    }
    pub(super) fn stdout(&mut self, value: impl Into<Stdio>) -> &mut Self {
        self.command.stdout(value);
        self
    }
    pub(super) fn stderr(&mut self, value: impl Into<Stdio>) -> &mut Self {
        self.command.stderr(value);
        self
    }
    pub(super) fn env(&mut self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> &mut Self {
        self.command.env(key, value);
        self
    }
}
impl Deref for Capture {
    type Target = Command;
    fn deref(&self) -> &Command {
        &self.command
    }
}
impl DerefMut for Capture {
    fn deref_mut(&mut self) -> &mut Command {
        &mut self.command
    }
}
impl Drop for Capture {
    fn drop(&mut self) {
        if self.spawned
            && !self.finished
            && let Some(context) = &self.context
        {
            context.uncaptured();
        }
    }
}
