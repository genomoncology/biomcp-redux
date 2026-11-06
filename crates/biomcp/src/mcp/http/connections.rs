//! Recover individual socket and worker failures while retaining admission bounds.
use super::{Service, ACCEPT_POLL, admission, record, wire};
use std::{io, net::{TcpListener, TcpStream}, sync::Arc, thread};
#[expect(
    clippy::disallowed_methods,
    reason = "Own bounded framing and active request workers through completion"
)]
pub(super) fn run(
    listener: TcpListener,
    service: Arc<Service>,
    stopped: std::sync::mpsc::Receiver<()>,
) -> io::Result<()> {
    run_with(listener, service, stopped, |listener| listener.accept().map(|(stream, _)| stream), |work| {
        thread::Builder::new().name("mcp-http".into()).spawn(work)
    })
}
#[expect(clippy::disallowed_methods, reason = "Own bounded HTTP worker handles until shutdown")]
pub(super) fn run_with(
    listener: TcpListener,
    service: Arc<Service>,
    stopped: std::sync::mpsc::Receiver<()>,
    mut accept: impl FnMut(&TcpListener) -> io::Result<TcpStream>,
    mut spawn: impl FnMut(Box<dyn FnOnce() + Send>) -> io::Result<thread::JoinHandle<()>>,
) -> io::Result<()> {
    listener.set_nonblocking(true)?;
    let mut workers: Vec<thread::JoinHandle<()>> = Vec::new();
    let max_workers = service.max_connections * 2 + 1;
    let readers = admission::Slots::new(max_workers);
    loop {
        if stopped.try_recv().is_ok() {
            break;
        }
        let mut stream = match accept(&listener) {
            Ok(stream) => {
                if let Err(error) = stream.set_nonblocking(false) {
                    record(Err(error));
                    continue;
                }
                stream
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                match stopped.recv_timeout(ACCEPT_POLL) {
                    Ok(()) | Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
                }
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => {
                record(Err(error));
                match stopped.recv_timeout(ACCEPT_POLL) {
                    Ok(()) | Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
                }
            },
        };
        let mut retained = Vec::with_capacity(workers.len());
        for worker in workers {
            if worker.is_finished() {
                let _ = worker.join();
            } else {
                retained.push(worker);
            }
        }
        workers = retained;
        // Retain a whole-worker bound while phase permits change hands or diagnostics run.
        let reader = if workers.len() < max_workers {
            readers.take()
        } else {
            None
        };
        let Some(reader) = reader else {
            record(wire::unavailable(&mut stream));
            continue;
        };
        let connection = service.connections.take();
        let service = service.clone();
        match spawn(Box::new(move || {
            record(service.handle(stream, reader, connection));
        })) {
            Ok(worker) => workers.push(worker),
            Err(error) => record(Err(error)),
        }

    }
    drop(listener);
    service.calls.cancel();
    for worker in workers {
        worker
            .join()
            .map_err(|_| io::Error::other("HTTP worker stopped"))?;
    }
    Ok(())
}
