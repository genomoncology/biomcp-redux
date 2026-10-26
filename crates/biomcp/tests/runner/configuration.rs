//! Process settings and persistent case-cache ownership through the shipped binary.
use super::{launch::Environment, mcp};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
type Check = Result<(), String>;
pub(super) fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/gene/main")
}
pub(super) fn invoke(env: &Environment, args: Vec<String>, exit: i32) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) || !output.stderr.is_empty() {
        return Err(format!(
            "expected exit {exit}: {:?}; stderr: {}; stdout: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
                .chars()
                .take(2000)
                .collect::<String>()
        ));
    }
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if let Some(kind) = value.get("local_error") {
        println!("LOCAL_ERROR_CLI_BYTES {kind} {}", output.stdout.len());
    }
    if args.iter().any(|word| word == "diagnostic")
        || args.windows(2).any(|words| {
            words.first().is_some_and(|word| word == "sync")
                && words
                    .get(1)
                    .is_some_and(|word| matches!(word.as_str(), "gtr" | "who-ivd" | "fda-cdx"))
        })
    {
        super::diagnostic_retirements::bounded(&value)?;
    }
    Ok(value)
}
pub(super) fn cached(args: &[&str], case: &Path, recording: Option<&Path>) -> Vec<String> {
    let mut args: Vec<String> = args.iter().map(|arg| (*arg).into()).collect();
    if let Some(recording) = recording {
        args.extend(["--replay".into(), recording.to_string_lossy().into()]);
    } else {
        args.push("--offline".into());
    }
    args.extend(["--case-cache".into(), case.to_string_lossy().into()]);
    args
}
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("gene/version-one-default-cache-remains-readable", || {
            default_cache_migration().map_err(Into::into)
        }),
        Trial::test("article/concurrent-process-pool-publication", || {
            article_processes().map_err(Into::into)
        }),
        Trial::test("article/persistent-replay-command-epochs", || {
            article_replay_epoch().map_err(Into::into)
        }),
        Trial::test("article/owners-refused-before-journal-and-tables", || {
            article_owners().map_err(Into::into)
        }),
        Trial::test("gene/next-commands-preserve-hyphen-positionals", || {
            next_positionals().map_err(Into::into)
        }),
        Trial::test("gene/persistent-case-cache-owns-all-writes", || {
            ownership().map_err(Into::into)
        }),
        Trial::test("gene/refuses-unowned-case-cache-paths", || {
            refusals().map_err(Into::into)
        }),
        Trial::test("gene/checked-command-and-config-boundaries", || {
            boundaries().map_err(Into::into)
        }),
    ]
}
fn default_cache_migration() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = super::network_stub::Stub::new()?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    invoke(&env, vec!["sync".into(), "hgnc".into()], 0)?;
    let folder = env.root().join("cache/biomcp");
    let current = fs::read_dir(&folder)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| p.extension().is_some_and(|v| v == "sqlite"))
        .ok_or("missing cache")?;
    let historical = folder.join("cache-1-5.sqlite");
    if current != historical {
        fs::rename(&current, &historical).map_err(|e| e.to_string())?;
        fs::rename(
            current.with_extension("parts"),
            historical.with_extension("parts"),
        )
        .map_err(|e| e.to_string())?;
    }
    let connection = rusqlite::Connection::open(&historical).map_err(|e| e.to_string())?;
    connection.execute_batch("ALTER TABLE reply DROP COLUMN pagination; ALTER TABLE reply DROP COLUMN content_range; UPDATE meta SET version=1 WHERE owner='biomcp-fetch';")
        .map_err(|e| e.to_string())?;
    drop(connection);
    let args = vec![
        "get".into(),
        "gene".into(),
        "BRAF".into(),
        "--offline".into(),
    ];
    let value = invoke(&env, args.clone(), 0)?;
    let (command, globals) = args.split_at_checked(3).ok_or("missing offline command")?;
    let peer = mcp::call_with_globals(&env, command, false, globals)?;
    if value != peer || value.pointer("/result/items/0/answer/gene/symbol") != Some(&json!("BRAF"))
    {
        return Err(format!(
            "upgrade lost the ordinary offline gene bundle: {value}"
        ));
    }
    let article = invoke(
        &env,
        vec![
            "search".into(),
            "article".into(),
            "synthetic".into(),
            "--offline".into(),
        ],
        4,
    )?;
    if article.get("status") != Some(&json!("failed")) {
        return Err("query owner could not open the migrated default cache".into());
    }
    if stub.finish()?.len() != 2 {
        return Err("offline migration opened HTTP".into());
    }
    Ok(())
}
fn ownership() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let user_cache = env.root().join("cache/biomcp");
    fs::create_dir(&user_cache).map_err(|e| e.to_string())?;
    let file = user_cache.join("replies.sqlite");
    fs::write(&file, b"prepopulated user cache sentinel").map_err(|e| e.to_string())?;
    let case = env.root().join("case");
    let replay = invoke(
        &env,
        cached(&["get", "gene", "BRAF"], &case, Some(&root())),
        0,
    )?;
    let offline = invoke(&env, cached(&["get", "gene", "BRAF"], &case, None), 0)?;
    if replay != offline {
        return Err("offline lost the checked replay reply".into());
    }
    let missing = env.root().join("missing");
    fs::create_dir(&missing).map_err(|e| e.to_string())?;
    let failed = invoke(
        &env,
        cached(&["get", "gene", "BRAF"], &case, Some(&missing)),
        5,
    )?;
    if failed.get("status") != Some(&json!("failed"))
        || failed.pointer("/result/items/0/answer").is_some()
    {
        return Err("warm rows masked a missing recording".into());
    }
    if fs::read(&file).map_err(|e| e.to_string())? != b"prepopulated user cache sentinel" {
        return Err("replay wrote the user cache".into());
    }
    let sync_case = env.root().join("sync-case");
    let value = invoke(
        &env,
        cached(&["sync", "hgnc"], &sync_case, Some(&root())),
        0,
    )?;
    if value.get("schema") != Some(&json!("biomcp.sync/4"))
        || value.pointer("/result/rows") != Some(&json!(176))
        || value.pointer("/query/source") != Some(&json!("hgnc"))
    {
        return Err("source sync lost metadata".into());
    }
    let withdrawn = invoke(&env, cached(&["get", "gene", "BRCA3"], &sync_case, None), 0)?;
    if withdrawn.pointer("/result/items/0/answer/resolution") != Some(&json!("withdrawn")) {
        return Err("HGNC sync omitted withdrawn identity".into());
    }
    Ok(())
}
fn refusals() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let case = env.root().join("owned");
    invoke(
        &env,
        cached(&["get", "gene", "MLL2"], &case, Some(&root())),
        0,
    )?;
    for (name, marker, bytes) in [
        ("unmarked", None, b"foreign database".as_slice()),
        (
            "foreign",
            Some(b"another owner".as_slice()),
            b"foreign database".as_slice(),
        ),
        (
            "bad-database",
            Some(b"biomcp.replay-case/1\n".as_slice()),
            b"foreign database".as_slice(),
        ),
    ] {
        let path = env.root().join(name);
        fs::create_dir(&path).map_err(|e| e.to_string())?;
        if let Some(marker) = marker {
            fs::write(path.join(".biomcp-case"), marker).map_err(|e| e.to_string())?;
        }
        fs::write(path.join("replay.sqlite"), bytes).map_err(|e| e.to_string())?;
        invoke(&env, cached(&["get", "gene", "BRAF"], &path, None), 5)?;
        if fs::read(path.join("replay.sqlite")).map_err(|e| e.to_string())? != bytes {
            return Err("refusal changed foreign files".into());
        }
    }
    #[cfg(unix)]
    {
        let link = env.root().join("linked-root");
        std::os::unix::fs::symlink(&case, &link).map_err(|e| e.to_string())?;
        invoke(&env, cached(&["get", "gene", "BRAF"], &link, None), 5)?;
        let path = env.root().join("linked-file");
        fs::create_dir(&path).map_err(|e| e.to_string())?;
        fs::copy(case.join(".biomcp-case"), path.join(".biomcp-case"))
            .map_err(|e| e.to_string())?;
        std::os::unix::fs::symlink(case.join("replay.sqlite"), path.join("replay.sqlite"))
            .map_err(|e| e.to_string())?;
        invoke(&env, cached(&["get", "gene", "BRAF"], &path, None), 5)?;
    }
    Ok(())
}
fn boundaries() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for args in [
        vec!["sync", "hgnc-withdrawn"],
        vec!["get", "gene"],
        vec!["search", "gene"],
        vec!["search", "gene", "BRAF", "--limit", "0"],
        vec!["search", "gene", "--band", "invalid"],
        vec!["get", "gene", "BRAF", "--case-cache", "sensitive"],
        vec!["get", "gene", "BRAF", "--offline", "--replay", "sensitive"],
    ] {
        let value = invoke(&env, args.iter().map(|arg| (*arg).into()).collect(), 2)?;
        if value.to_string().contains("sensitive") || value.get("local_error").is_some() {
            return Err("usage echoed caller input".into());
        }
    }
    let args = vec!["get".into(), "gene".into(), "NOTAGENE".into()];
    let globals = vec!["--replay".into(), root().to_string_lossy().into()];
    mcp::call_with_globals(&env, &args, false, &globals)?;
    for flag in [
        "--record",
        "--replay",
        "--case-cache",
        "--case-cache=sensitive",
    ] {
        mcp::call_with_globals(
            &env,
            &["get".into(), "gene".into(), "BRAF".into(), flag.into()],
            true,
            &[],
        )?;
    }
    let directory = env.root().join("config/biomcp");
    fs::create_dir(&directory).map_err(|e| e.to_string())?;
    for config in [
        json!({"unknown":"sensitive"}),
        json!({"keys":{"unknown":"sensitive"}}),
        json!({"keys":{"oncokb_api_key":9}}),
        json!({"proxy":9}),
        json!({"proxy":"http://[invalid"}),
    ] {
        fs::write(directory.join("config.json"), config.to_string()).map_err(|e| e.to_string())?;
        let value = invoke(
            &env,
            vec![
                "get".into(),
                "gene".into(),
                "BRAF".into(),
                "--offline".into(),
            ],
            5,
        )?;
        if value.to_string().contains("sensitive") || value.get("local_error").is_some() {
            return Err("configuration echoed input".into());
        }
    }
    key_boundaries()
}
fn key_boundaries() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"unknown":"sensitive"}))
        .map_err(|e| e.to_string())?;
    let help = invoke(&env, vec!["help".into(), "key".into(), "oncokb".into()], 0)?;
    if help.pointer("/result/config") != Some(&json!("keys.oncokb_api_key")) {
        return Err("key help read malformed settings".into());
    }
    if env.root().join("cache/biomcp").exists() {
        return Err("key help opened a cache".into());
    }
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"keys":{"oncokb_api_key":""}}))
        .map_err(|e| e.to_string())?;
    let recording = root()
        .parent()
        .ok_or("fixture root")?
        .join("oncokb-synthetic/data");
    let args = vec![
        "--replay".into(),
        recording.to_string_lossy().into(),
        "get".into(),
        "gene".into(),
        "TP53".into(),
    ];
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .env("ONCOKB_TOKEN", "synthetic-override")
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success()
        || value.pointer("/result/items/0/oncokb/version") != Some(&json!("synthetic-1"))
    {
        return Err("environment key did not override the empty file key".into());
    }
    Ok(())
}

fn next_positionals() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for input in [
        vec!["get", "gene", "--offline", "--", "-BRAF"],
        vec!["search", "gene", "--offline", "--", "-BRAF"],
        vec![
            "get",
            "gene",
            "--offline",
            "--section",
            "constraint",
            "--",
            "-BRAF",
        ],
        vec![
            "get",
            "gene",
            "--offline",
            "--section",
            "constraint",
            "constraint",
        ],
    ] {
        let original = invoke(&env, input.into_iter().map(str::to_owned).collect(), 4)?;
        let args = original
            .pointer("/next/0/args")
            .and_then(Value::as_array)
            .ok_or("missing fresh command")?;
        let mut args = args
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .ok_or("invalid command argument")
                    .map(str::to_owned)
            })
            .collect::<Result<Vec<_>, _>>()?;
        if !args.iter().any(|arg| arg == "--fresh") {
            return Err("fresh policy was lost".into());
        }
        args.insert(0, "--offline".into());
        let repeated = invoke(&env, args, 4)?;
        if repeated.get("query") != original.get("query") {
            return Err("next command changed its positional query".into());
        }
    }
    Ok(())
}

fn article_owners() -> Check {
    for owned in [false, true] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let parent = env.root().join("cache/biomcp");
        fs::create_dir_all(&parent).map_err(|e| e.to_string())?;
        let file = parent.join(format!(
            "cache-{}-{}.sqlite",
            biomcp_fetch::CACHE_NAMESPACE_VERSION,
            biomcp_query::store::VERSION
        ));
        let case = env.root().join("case");
        let replay = if owned {
            Some(biomcp_fetch::ReplayCache::create_case(&case).map_err(|e| e.to_string())?)
        } else {
            None
        };
        let fetch = biomcp_fetch::Fetch::open(biomcp_fetch::FetchConfig {
            cache: Some(file),
            mode: biomcp_fetch::Mode::Offline,
            replay_cache: replay,
            ..Default::default()
        })
        .map_err(|e| e.to_string())?;
        let connection = fetch
            .cache_access()
            .map_err(|e| e.to_string())?
            .connect()
            .map_err(|e| e.to_string())?;
        drop(fetch);
        connection
            .execute(
                "INSERT INTO meta(owner,version) VALUES ('biomcp-query',?1)",
                [biomcp_query::store::VERSION + 1],
            )
            .map_err(|e| e.to_string())?;
        connection
            .pragma_update(None, "journal_mode", "DELETE")
            .map_err(|e| e.to_string())?;
        let mut args = vec![
            "search".into(),
            "article".into(),
            "AKT1 E17K".into(),
            "--offline".into(),
        ];
        if owned {
            args.extend(["--case-cache".into(), case.to_string_lossy().into()]);
        }
        let result = invoke(&env, args, 5)?;
        if result.pointer("/error/code") != Some(&json!("local.failure"))
            || result.get("local_error") != Some(&json!("version"))
        {
            return Err("wrong owner was not a local configuration error".into());
        }
        let journal = connection
            .query_row("PRAGMA journal_mode", [], |r| r.get::<_, String>(0))
            .map_err(|e| e.to_string())?;
        let pool = connection
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name='pool')",
                [],
                |r| r.get::<_, bool>(0),
            )
            .map_err(|e| e.to_string())?;
        if journal != "delete" || pool {
            return Err("wrong query owner allowed journal or table mutation".into());
        }
    }
    Ok(())
}

fn article_replay_epoch() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let root = env.root().join("recordings");
    let pubmed = root.join("pubmed");
    fs::create_dir_all(&pubmed).map_err(|e| e.to_string())?;
    let original = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/replies/article/search/synthetic-small/pubmed");
    for entry in fs::read_dir(original).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let mut value: Value =
            serde_json::from_slice(&fs::read(entry.path()).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let exchanges = value
            .get_mut("exchanges")
            .and_then(Value::as_array_mut)
            .ok_or("exchanges missing")?;
        let first = exchanges.first().ok_or("exchange missing")?.clone();
        *exchanges = vec![first; 32];
        fs::write(
            pubmed.join(entry.file_name()),
            serde_json::to_vec(&value).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    mcp::article_sequence(&env, &env.globals(Some(&root), false))
}

pub(super) fn article_processes() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let case = env.root().join("case");
    let fixture = super::launch::fixtures().join("replies/article/search/synthetic-small");
    let base = ["search", "article", "AKT1 E17K", "--limit", "2"];
    let initial = invoke(&env, cached(&base, &case, Some(&fixture)), 0)?;
    let expected = initial.pointer("/result/items").ok_or("items missing")?;
    let parent = env.root().join("cache/biomcp");
    fs::create_dir_all(&parent).map_err(|e| e.to_string())?;
    let file = parent.join(format!(
        "cache-{}-{}.sqlite",
        biomcp_fetch::CACHE_NAMESPACE_VERSION,
        biomcp_query::store::VERSION
    ));
    // Replay and ordinary caches have the same owned schema. Copy only after
    // the seed process closes; it has checkpointed the WAL and released locks.
    fs::copy(case.join("replay.sqlite"), &file).map_err(|e| e.to_string())?;
    let fetch = biomcp_fetch::Fetch::open(biomcp_fetch::FetchConfig {
        cache: Some(file),
        mode: biomcp_fetch::Mode::Offline,
        cache_owners: vec![biomcp_query::store::OWNER],
        ..Default::default()
    })
    .map_err(|e| e.to_string())?;
    let connection = fetch
        .cache_access()
        .map_err(|e| e.to_string())?
        .connect()
        .map_err(|e| e.to_string())?;
    connection
        .execute_batch("BEGIN IMMEDIATE")
        .map_err(|e| e.to_string())?;
    let replies = super::publication::check(&env, || {
        connection
            .execute_batch("COMMIT")
            .map_err(|e| e.to_string())
    })?;
    let mut pools = std::collections::BTreeSet::new();
    for reply in replies {
        if reply.pointer("/result/items") != Some(expected) {
            return Err("concurrent ordinary-cache search lost items".into());
        }
        let pool = reply
            .pointer("/page/pool/id")
            .and_then(Value::as_str)
            .ok_or("published pool missing")?;
        if !pools.insert(pool.to_owned()) || initial.pointer("/page/pool/id") == Some(&json!(pool))
        {
            return Err("concurrent fresh pools collided".into());
        }
        let mut args: Vec<String> = base.iter().map(|v| (*v).into()).collect();
        args.extend(["--offline".into(), "--pool".into(), pool.into()]);
        if invoke(&env, args, 0)?.pointer("/result/items") != Some(expected) {
            return Err("published snapshot changed".into());
        }
    }
    Ok(())
}
