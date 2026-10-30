//! Feature runner: failed local peers retain diagnostics and synthetic scratch.
use super::super::{launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use std::{
    fs,
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("schema-discovery/retains-failed-local-peer-state", || {
            check().map_err(Into::into)
        }),
        transcript_trial("schema-discovery/persistent-fixture-evidence", || {
            persistence().map_err(|error| error.to_string().into())
        }),
        transcript_trial("schema-discovery/failed-local-peer-child", || {
            aborted(true).map(|_| ()).map_err(Into::into)
        }).with_ignored_flag(true),
        transcript_trial("schema-discovery/single-source-capturing-callback", || {
            capturing_callback().map_err(|error| error.to_string().into())
        }),
    ]
}
fn capturing_callback() -> Result<(), Box<dyn std::error::Error>> {
    use biomcp_fetch::{Certificates, Fetch, FetchConfig, Network, Secret};
    use std::time::Duration;

    let mut reply = String::from("captured");
    let stub = Stub::for_source("eutils.ncbi.nlm.nih.gov", "text/plain", move |_| {
        reply.push('!');
        Ok(reply.clone())
    })?;
    let env = Environment::new()?;
    stub.retain(&env);
    let fetch = Fetch::open(FetchConfig {
        network: Network::new(
            None,
            Some(Secret::new(format!("http://{}", stub.address))),
            "",
            Certificates::from_pem(stub.certificate.as_bytes())?,
        )?,
        ..FetchConfig::default()
    })?;
    let source = &biomcp_adapters::pubmed::SPEC;
    for (path, expected) in [("/first", "captured!"), ("/second", "captured!!")] {
        let body = fetch
            .leg(source, fetch.deadline(Duration::from_secs(5)))
            .bytes(&source.get(path).bytes(64), &[])?;
        if body != expected.as_bytes() {
            return Err("single-source callback lost its captured mutable state".into());
        }
    }
    if stub.finish()?.len() != 2 {
        return Err("capturing callback did not serve both requests".into());
    }
    Ok(())
}
fn check() -> Result<(), String> {
    let healthy = Environment::new().map_err(|e| e.to_string())?;
    let healthy_root = healthy.root().to_owned();
    let peer = Stub::for_source("healthy.invalid", "text/plain", |_| Ok("healthy".into()))?;
    peer.retain(&healthy);
    fs::write(healthy.root().join("healthy-marker"), b"unrelated scratch")
        .map_err(|e| e.to_string())?;
    std::thread::scope(|scope| {
        let finish = scope.spawn(|| aborted(true));
        let cleanup = scope.spawn(|| aborted(false));
        fs::remove_dir_all(finish.join().map_err(|_| "finish proof panicked")??)
            .map_err(|e| e.to_string())?;
        fs::remove_dir_all(cleanup.join().map_err(|_| "cleanup proof panicked")??)
            .map_err(|e| e.to_string())
    })?;
    if fs::read(healthy.root().join("healthy-marker")).map_err(|e| e.to_string())?
        != b"unrelated scratch"
        || retained(&healthy)?.exists()
    {
        return Err("unrelated concurrent fixture scratch changed or was retained".into());
    }
    peer.finish()?;
    drop(healthy);
    if healthy_root.exists() {
        return Err("successful fixture scratch was not cleaned".into());
    }
    Ok(())
}
fn retained(env: &Environment) -> Result<std::path::PathBuf, String> {
    Ok(super::super::network_stub::evidence_directory()
        .map_err(|e| e.to_string())?
        .join(env.root().file_name().ok_or("scratch name")?))
}
fn aborted(finish: bool) -> Result<std::path::PathBuf, String> {
    let stub = Stub::for_source("fixture.invalid", "text/plain", |_| Ok("synthetic".into()))?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    stub.retain(&env);
    let retained = retained(&env)?;
    fs::write(env.root().join("synthetic-marker"), b"retained scratch")
        .map_err(|e| e.to_string())?;
    env.config(&serde_json::json!({"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let mut client = TcpStream::connect(stub.address).map_err(|e| e.to_string())?;
    let peer = client.local_addr().map_err(|e| e.to_string())?;
    client
        .write_all(b"CONNECT fixture.invalid:443 HTTP/1.1\r\n")
        .map_err(|e| e.to_string())?;
    client
        .shutdown(Shutdown::Write)
        .map_err(|e| e.to_string())?;
    // EOF acknowledges that the worker observed the incomplete CONNECT.
    let mut end = [0];
    if client.read(&mut end).map_err(|e| e.to_string())? != 0 {
        return Err("partial CONNECT unexpectedly received a response".into());
    }
    drop(env);
    if finish {
        let error = stub
            .finish()
            .err()
            .ok_or("forced disconnect was accepted")?;
        if !error.contains("CONNECT header") || !error.contains(&peer.to_string()) {
            return Err(format!("forced disconnect lost its phase or peer: {error}"));
        }
    } else {
        drop(stub);
    }
    let result = (|| {
        if fs::read(retained.join("scratch/synthetic-marker")).map_err(|e| e.to_string())?
            != b"retained scratch"
        {
            return Err("failed fixture lost scratch bytes".into());
        }
        let manifest: serde_json::Value = serde_json::from_slice(
            &fs::read(retained.join("manifest.json")).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        if manifest.get("phase") != Some(&serde_json::json!("CONNECT header"))
            || manifest.get("peer") != Some(&serde_json::json!(peer.to_string()))
            || manifest.get("ordinal") != Some(&serde_json::json!(1))
            || manifest.get("hosts") != Some(&serde_json::json!(["fixture.invalid"]))
        {
            return Err(format!(
                "failed fixture lost diagnostic context: {manifest}"
            ));
        }
        Ok(())
    })();
    result?;
    Ok(retained)
}

#[expect(
    clippy::disallowed_methods,
    reason = "The retention proof invokes its runner with isolated child settings"
)]
fn persistence() -> Result<(), Box<dyn std::error::Error>> {
    for explicit in [false, true] {
        let env = Environment::new()?;
        let scratch = env.root().join("job-tmp");
        fs::create_dir(&scratch)?;
        let args = [
            "--exact", "schema-discovery/failed-local-peer-child", "--ignored",
        ].map(str::to_owned);
        let mut command = env.command(std::env::current_exe()?, &args);
        command.env("TMPDIR", &scratch).env("TMP", &scratch).env("TEMP", &scratch);
        let expected = if explicit {
            let override_path = env.root().join("explicit-evidence");
            command.env("BIOMCP_EVIDENCE_DIR", &override_path);
            override_path.join("fixture-failures")
        } else {
            command.env_remove("BIOMCP_EVIDENCE_DIR");
            super::super::launch::checkout().join("../../target/fixture-evidence")
        };
        let output = command.output()?;
        if !output.status.success() {
            return Err(format!("fixture child failed: {output:?}").into());
        }
        let stderr = std::str::from_utf8(&output.stderr)?;
        let paths: Vec<String> = serde_json::from_str(
            stderr.lines().find_map(|line| line.split_once("; evidence=").map(|(_, p)| p))
                .ok_or("fixture child omitted retained evidence")?,
        )?;
        let retained = std::path::PathBuf::from(paths.first().ok_or("missing evidence path")?);
        if retained.parent() != Some(expected.canonicalize()?.as_path()) {
            return Err(format!("fixture evidence used unexpected path: {retained:?}").into());
        }
        fs::remove_dir_all(&scratch)?;
        if fs::read(retained.join("scratch/synthetic-marker"))? != b"retained scratch"
            || !retained.join("manifest.json").is_file()
        {
            return Err("fixture evidence did not survive scratch cleanup".into());
        }
        fs::remove_dir_all(retained)?;
    }
    Ok(())
}
