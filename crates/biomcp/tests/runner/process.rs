use std::{io::{self, Read}, process::{Child, ExitStatus}, thread, time::{Duration, Instant}};

const WATCHDOG: Duration = Duration::from_secs(30);

// Reader threads drain pipes while the parent watches process termination. No
// timing assertion defines success; the watchdog only kills and reaps a hang.
pub(super) struct Process {
    pub(super) child: Child,
    stdout: thread::JoinHandle<io::Result<Vec<u8>>>,
    stderr: thread::JoinHandle<io::Result<Vec<u8>>>,
}

impl Process {
    pub(super) fn new(mut child: Child) -> Result<Self, String> {
        let stdout = child.stdout.take().map_or_else(|| reader(io::empty()), reader);
        let stderr = child.stderr.take().ok_or("stderr missing")?;
        Ok(Self { child, stdout, stderr: reader(stderr) })
    }

    pub(super) fn finish(mut self) -> Result<(ExitStatus, Vec<u8>, Vec<u8>), String> {
        let deadline = Instant::now() + WATCHDOG;
        let status = loop {
            if let Some(status) = self.child.try_wait().map_err(|e| e.to_string())? { break status; }
            if Instant::now() >= deadline {
                let _ = self.child.kill();
                let _ = self.child.wait();
                return Err("MCP watchdog killed and reaped server".into());
            }
            thread::sleep(Duration::from_millis(10));
        };
        Ok((status, joined(self.stdout)?, joined(self.stderr)?))
    }
}

fn reader(mut input: impl Read + Send + 'static) -> thread::JoinHandle<io::Result<Vec<u8>>> {
    thread::spawn(move || { let mut bytes = Vec::new(); input.read_to_end(&mut bytes)?; Ok(bytes) })
}

fn joined(handle: thread::JoinHandle<io::Result<Vec<u8>>>) -> Result<Vec<u8>, String> {
    handle.join().map_err(|_| "pipe reader panicked")?.map_err(|e| e.to_string())
}
