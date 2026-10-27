//! One local peer owns only the synthetic environments explicitly attached to it.
#![expect(
    clippy::disallowed_methods,
    reason = "The fixture owner selects durable test evidence outside scheduler scratch"
)]
#![expect(
    clippy::disallowed_types,
    reason = "The local fixture shares I/O diagnostics and owned scratch with its worker thread"
)]
use super::launch::Environment;
use serde::Serialize;
use std::{
    fs, io,
    net::{SocketAddr, TcpStream},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

#[derive(Serialize)]
struct Context {
    local: SocketAddr,
    peer: Option<SocketAddr>,
    hosts: Vec<String>,
    ordinal: usize,
    phase: &'static str,
    io_error: Option<String>,
}
pub(super) struct Evidence {
    context: Mutex<Context>,
    roots: Mutex<Vec<Arc<tempfile::TempDir>>>,
}
impl Evidence {
    pub(super) fn new(local: SocketAddr, hosts: &[&str]) -> Arc<Self> {
        Arc::new(Self {
            context: Mutex::new(Context {
                local,
                peer: None,
                hosts: hosts.iter().map(|s| (*s).into()).collect(),
                ordinal: 0,
                phase: "accept",
                io_error: None,
            }),
            roots: Mutex::new(Vec::new()),
        })
    }
    pub(super) fn attach(&self, env: &Environment) {
        let root = env.shared_root();
        let mut roots = self
            .roots
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if !roots.iter().any(|other| Arc::ptr_eq(other, &root)) {
            roots.push(root);
        }
    }
    pub(super) fn accept(&self, stream: io::Result<TcpStream>) -> Result<TcpStream, String> {
        let socket = self.check("accept", stream)?;
        {
            let mut context = self
                .context
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            context.ordinal += 1;
            context.peer = socket.peer_addr().ok();
        }
        self.check(
            "read timeout setup",
            socket.set_read_timeout(Some(std::time::Duration::from_secs(10))),
        )?;
        self.check(
            "write timeout setup",
            socket.set_write_timeout(Some(std::time::Duration::from_secs(10))),
        )?;
        Ok(socket)
    }
    pub(super) fn check<T, E: std::fmt::Display>(
        &self,
        phase: &'static str,
        result: Result<T, E>,
    ) -> Result<T, String> {
        let mut context = self
            .context
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        context.phase = phase;
        context.io_error = result.as_ref().err().map(ToString::to_string);
        result.map_err(|error| {
            format!(
                "source stub {phase}: {error}; local={} peer={:?} hosts={:?} ordinal={}",
                context.local, context.peer, context.hosts, context.ordinal
            )
        })
    }
    pub(super) fn preserve(&self, reason: &'static str) -> String {
        let context = self
            .context
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let roots = self
            .roots
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut locations = Vec::new();
        for root in roots.iter() {
            match retain(root.path(), &context, reason) {
                Ok(path) => locations.push(path.display().to_string()),
                Err(error) => locations.push(format!("retention failed: {error}")),
            }
        }
        let line = format!(
            "fixture {reason}: phase={} local={} peer={:?} hosts={:?} ordinal={}; evidence={locations:?}",
            context.phase, context.local, context.peer, context.hosts, context.ordinal
        );
        use std::io::Write;
        let _ = writeln!(io::stderr(), "{line}");
        line
    }
}
fn retain(root: &Path, context: &Context, reason: &str) -> io::Result<PathBuf> {
    let base = directory()?;
    let path = base.join(
        root.file_name()
            .ok_or_else(|| io::Error::other("missing fixture directory name"))?,
    );
    fs::create_dir(&path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700))?;
    }
    copy(root, &path.join("scratch"))?;
    let mut manifest = serde_json::to_value(context)?;
    manifest
        .as_object_mut()
        .ok_or_else(|| io::Error::other("fixture context is not an object"))?
        .insert("reason".into(), reason.into());
    fs::write(
        path.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest)?,
    )?;
    Ok(path)
}
fn directory() -> io::Result<PathBuf> {
    let fallback = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/fixture-failures");
    let directory = std::env::var_os("BIOMCP_EVIDENCE_DIR").map_or_else(
        || fallback.clone(),
        |p| PathBuf::from(p).join("fixture-failures"),
    );
    fs::create_dir_all(&directory)?;
    let directory = directory.canonicalize()?;
    // A configured temporary directory cannot defeat native-gate retention.
    if std::env::var_os("TMPDIR")
        .map(PathBuf::from)
        .and_then(|p| p.canonicalize().ok())
        .is_some_and(|p| directory.starts_with(p))
    {
        fs::create_dir_all(&fallback)?;
        return fallback.canonicalize();
    }
    Ok(directory)
}
fn copy(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_dir() {
            copy(&entry.path(), &destination.join(entry.file_name()))?;
        } else if kind.is_file() {
            fs::copy(entry.path(), destination.join(entry.file_name()))?;
        }
        // Never follow a link out of an isolated fixture environment.
    }
    Ok(())
}
