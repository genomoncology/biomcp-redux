//! Prove that another process reads the ordinary shared checked reply cache.
use super::*;
#[test]
#[expect(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "Launch an isolated copy of this test executable to prove process sharing"
)]
fn another_process_reads_checked_cache_without_connecting() -> Check {
    let fixture = Fixture::with_messages(Hold::None, 0, false, false, messages())?;
    let source = fixture.source(false, KeyUse::None);
    let directory = tempfile::tempdir()?;
    let cache = directory.path().join("shared.sqlite");
    let live = fetch(
        &fixture,
        Mode::default(),
        Some(cache.clone()),
        Limits::default(),
    )?;
    get(&live, source, METHOD, SCHEMA, Freshness::Hours(1))?;
    let mut process = std::process::Command::new(std::env::current_exe()?);
    process.env_clear();
    for name in ["PATH", "SystemRoot", "WINDIR", "TMP", "TEMP"] {
        if let Some(value) = std::env::var_os(name) {
            process.env(name, value);
        }
    }
    let result = process
        .args(["--exact", "process::offline_child", "--nocapture"])
        .env("RPC_TEST_CACHE", cache)
        .env("RPC_TEST_ORIGIN", source.base)
        .output()?;
    assert!(
        result.status.success(),
        "child: {} {}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(fixture.count.load(std::sync::atomic::Ordering::SeqCst), 1);
    fixture.finish()
}
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "Read only the explicitly passed synthetic child settings"
)]
fn offline_child() -> Check {
    let Some(cache) = std::env::var_os("RPC_TEST_CACHE") else {
        return Ok(());
    };
    let base = Box::leak(std::env::var("RPC_TEST_ORIGIN")?.into_boxed_str());
    let source = Box::leak(Box::new(SourceSpec {
        base,
        source: biomcp_types::Source::MyGene,
        quota: &QUOTA,
        key: KeyUse::None,
        user_agent: None,
        deadline: Duration::from_secs(5),
        terms: biomcp_fetch::Terms {
            name: "Synthetic fixture",
            url: "https://example.test/terms",
            usage: biomcp_types::Use::Open,
        },
    }));
    let offline = Fetch::open(FetchConfig {
        mode: Mode::Offline,
        cache: Some(cache.into()),
        ..Default::default()
    })?;
    assert_eq!(
        get(&offline, source, METHOD, SCHEMA, Freshness::Hours(1))?,
        [1, 2, 3, 4]
    );
    Ok(())
}
