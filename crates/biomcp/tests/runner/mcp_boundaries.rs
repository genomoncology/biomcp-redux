//! MCP transport authority is checked before provider requests or filesystem changes.
use super::{http::Server, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
type Check = Result<(), Box<dyn std::error::Error>>;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "mcp-http/refuse-maintenance-and-fresh-before-effects",
            || http().map_err(|e| e.to_string().into()),
        ),
        Trial::test(
            "article-saves/stdio-refuse-replacement-before-effects",
            || stdio().map_err(|e| e.to_string().into()),
        ),
    ]
}
fn stub() -> Result<Stub, String> {
    Stub::for_sources(
        &["www.wikipathways.org", "mygene.info", "www.ebi.ac.uk"],
        |_| "application/json",
        |_| Ok("{}".into()),
    )
}
fn refusal(value: &Value) -> Check {
    assert_eq!(value.get("status"), Some(&json!("usage")), "{value}");
    assert_eq!(
        value.pointer("/error/code"),
        Some(&json!("usage.arguments")),
        "{value}"
    );
    Ok(())
}
fn http() -> Check {
    let source = stub()?;
    let env = Environment::new()?;
    let certificate = env.root().join("source.pem");
    fs::write(&certificate, &source.certificate)?;
    let server = Server::keys(
        json!({"proxy":format!("http://{}", source.address), "ca_bundle":certificate}),
        None,
    )?;
    let directory = server.environment().root().join("cache/biomcp");
    fs::create_dir_all(&directory)?;
    let old = directory.join("cache-1-4.sqlite");
    rusqlite::Connection::open(&old)?
        .execute_batch("CREATE TABLE meta(owner TEXT,version INTEGER); INSERT INTO meta VALUES ('biomcp-query',4); CREATE TABLE pool(id TEXT);")?;
    let before = fs::read(&old)?;
    let entries = fs::read_dir(&directory)?.count();
    for args in [
        vec!["sync", "wikipathways"],
        vec!["store", "clean"],
        vec!["get", "gene", "BRAF", "--fresh"],
    ] {
        let frame = super::mcp::request(
            json!(1),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":args}}),
        );
        let (status, reply) = server.request("POST", "/mcp", &frame.to_string())?;
        assert_eq!(status, 200);
        assert_eq!(
            reply.pointer("/result/isError"),
            Some(&json!(true)),
            "{reply}"
        );
        refusal(
            reply
                .pointer("/result/structuredContent")
                .ok_or("missing tool result")?,
        )?;
        assert_eq!(fs::read(&old)?, before);
        assert_eq!(fs::read_dir(&directory)?.count(), entries);
    }
    assert!(
        source.finish()?.is_empty(),
        "refused HTTP actions contacted a provider"
    );
    Ok(())
}
fn stdio() -> Check {
    let source = stub()?;
    let env = Environment::new()?;
    let certificate = env.root().join("source.pem");
    fs::write(&certificate, &source.certificate)?;
    env.config(&json!({"proxy":format!("http://{}", source.address), "ca_bundle":certificate}))?;
    let path = env.root().join("owned.md");
    fs::write(&path, b"caller content")?;
    for words in [
        vec!["article", "text", "PPR1295425"],
        vec!["article", "supplement", "28944238", "MGG3-5-553-s002.xlsx"],
        vec!["article", "pdf", "28944238"],
        vec!["trial", "document", "NCT04303780", "SAP_001.pdf"],
    ] {
        let mut args: Vec<String> = words.into_iter().map(str::to_owned).collect();
        args.extend([
            "--save".into(),
            path.to_string_lossy().into(),
            "--replace".into(),
        ]);
        let value = super::mcp::call_with_globals(&env, &args, true, &[])?;
        refusal(&value)?;
        assert_eq!(fs::read(&path)?, b"caller content");
    }
    assert!(
        source.finish()?.is_empty(),
        "refused stdio replacement contacted a provider"
    );
    Ok(())
}
