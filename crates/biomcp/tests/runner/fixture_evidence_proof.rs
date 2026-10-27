//! Feature runner: failed local peers retain diagnostics and synthetic scratch.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use std::{fs, io::{Read, Write}, net::{Shutdown, TcpStream}, path::Path};

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("runner/retains-failed-local-peer-state", || check().map_err(Into::into))]
}
fn check() -> Result<(), String> {
    for finish in [true, false] {
        let stub = Stub::for_source("fixture.invalid", "text/plain", |_| Ok("synthetic".into()))?;
        let env = Environment::new().map_err(|e| e.to_string())?;
        let name = env.root().file_name().ok_or("scratch name")?.to_owned();
        let retained = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/fixture-failures").join(name);
        fs::write(env.root().join("synthetic-marker"), b"retained scratch").map_err(|e| e.to_string())?;
        env.config(&serde_json::json!({"proxy":format!("http://{}",stub.address)}))
            .map_err(|e| e.to_string())?;
        let mut client = TcpStream::connect(stub.address).map_err(|e| e.to_string())?;
        let peer = client.local_addr().map_err(|e| e.to_string())?;
        client.write_all(b"CONNECT fixture.invalid:443 HTTP/1.1\r\n").map_err(|e| e.to_string())?;
        client.shutdown(Shutdown::Write).map_err(|e| e.to_string())?;
        // EOF acknowledges that the worker observed the incomplete CONNECT.
        let mut end = [0];
        if client.read(&mut end).map_err(|e| e.to_string())? != 0 {
            return Err("partial CONNECT unexpectedly received a response".into());
        }
        if finish {
            let error = stub.finish().err().ok_or("forced disconnect was accepted")?;
            if !error.contains("CONNECT header") || !error.contains(&peer.to_string()) {
                return Err(format!("forced disconnect lost its phase or peer: {error}"));
            }
        } else {
            drop(stub);
        }
        drop(env);
        let result = (|| {
            if fs::read(retained.join("scratch/synthetic-marker")).map_err(|e| e.to_string())? != b"retained scratch" {
                return Err("failed fixture lost scratch bytes".into());
            }
            let manifest: serde_json::Value = serde_json::from_slice(&fs::read(retained.join("manifest.json")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
            if manifest["phase"] != "CONNECT header" || manifest["peer"] != peer.to_string()
                || manifest["ordinal"] != 1 || manifest["hosts"] != serde_json::json!(["fixture.invalid"]) {
                return Err(format!("failed fixture lost diagnostic context: {manifest}"));
            }
            Ok(())
        })();
        let _ = fs::remove_dir_all(&retained);
        result?;
    }
    Ok(())
}
