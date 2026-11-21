//! A busy PDF slot cannot become evidence in a hosted label-history reply.
use super::{
    Error, content, held_worker, pending, receive, retry, runner, send, server, start_read, until,
    workers,
};
use nix::{
    sys::signal::{Signal, kill},
    unistd::Pid,
};
use serde_json::json;
use std::{fs, path::Path};

const OLDEST: &str = "be328129378698d589f8ef99393b0e8d8c0378870272de9cd420da946b40b798-0.bin";

pub(super) fn fixtures(replay: &Path, pause_oldest: bool) -> Result<(), Error> {
    let source = runner::launch::fixtures().join("replies/drug/label-history-livalo/drugsfda");
    let destination = replay.join("drugsfda");
    fs::create_dir_all(&destination)?;
    for item in fs::read_dir(source)? {
        let item = item?;
        let target = destination.join(item.file_name());
        if pause_oldest && item.file_name() == OLDEST {
            let mut pdf = fs::read(item.path())?;
            pdf.get_mut(9..14)
                .ok_or("oldest PDF lacks its header comment")?
                .copy_from_slice(b"%hold");
            fs::write(target, pdf)?;
        } else {
            fs::copy(item.path(), target)?;
        }
    }
    Ok(())
}

pub(super) fn busy(after_match: bool) -> Result<(), Error> {
    let (mut server, _replay) = server(after_match)?;
    let mut args = vec![
        "get",
        "drug",
        "NDA022363",
        "label-history",
        "--source",
        "drugsfda",
        "--label-section",
        "adverse_reactions",
        "--find",
        "myalgia",
    ];
    if !after_match {
        args.push("--every-version");
    }
    let request = runner::mcp::request(
        json!(5),
        "tools/call",
        json!({"name":"biomcp", "arguments":{"args":args}}),
    )
    .to_string();
    let (history, held) = if after_match {
        let history = send(&server, &request)?;
        let oldest = until("the first matching label worker to pause", || {
            for pid in workers(&server)? {
                let status = fs::read_to_string(format!("/proc/{pid}/status"))?;
                if status.lines().any(|line| line.starts_with("State:\tT")) {
                    return Ok(Some(pid));
                }
            }
            Ok(None)
        })?;
        let held = start_read(&server)?;
        let pending = pending(&held);
        kill(Pid::from_raw(i32::try_from(oldest)?), Signal::SIGCONT)?;
        pending?;
        held_worker(&server)?;
        (history, held)
    } else {
        let held = start_read(&server)?;
        held_worker(&server)?;
        (send(&server, &request)?, held)
    };
    let reply = receive(history)?;
    drop(held);
    let result = content(&reply)?;
    let body = result.get("result").ok_or("missing label history")?;
    let reads = body
        .get("read")
        .and_then(serde_json::Value::as_array)
        .map_or(0, Vec::len);
    let unread = body.get("not_opened").and_then(serde_json::Value::as_u64);
    let versions = body.get("versions").and_then(serde_json::Value::as_u64);
    if reads != usize::from(after_match)
        || unread != versions.and_then(|count| count.checked_sub(reads as u64))
        || after_match && body.pointer("/read/0/state") != Some(&json!("match"))
        || result
            .get("next")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|steps| {
                steps
                    .iter()
                    .any(|step| step.get("kind") == Some(&json!("page")))
            })
        || body.get("stopped") != Some(&json!(true))
        || body.get("first").is_some()
        || result.get("status") != Some(&json!("partial"))
        || !retry(result)
        || body
            .get("method")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|method| method.contains("both have the text"))
    {
        return Err(format!("a busy label version became evidence: {result}").into());
    }
    server.stop()?;
    Ok(())
}
