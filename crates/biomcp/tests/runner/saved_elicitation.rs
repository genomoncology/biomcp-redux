//! Exercise saved-work confirmation against the installed stdio and hosted HTTP server.
use super::{
    http::Server,
    launch::{Environment, fixtures},
    mcp::{http_wire, one_value, request},
    publication::Peer,
    saved_files::{run, saved_file},
    transcript::trial,
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        trial("saved-work/stdio-delete-all-confirmation", || {
            stdio().map_err(Into::into)
        }),
        trial("saved-work/http-delete-all-confirmation", || {
            hosted().map_err(Into::into)
        }),
    ]
}

fn arguments(target: Value) -> Value {
    json!({"action":"delete","delete":target})
}

fn delete(id: u32, target: Value) -> Value {
    json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":{
        "name":"saved","arguments":arguments(target)
    }})
}

fn save_search(env: &Environment, name: &str) -> Result<(), String> {
    let replay = fixtures().join("replies/trial/condition");
    let globals = env.globals(Some(&replay), false);
    let mut words = vec![
        "search",
        "trial",
        "--condition",
        "melanoma",
        "--save-as",
        name,
    ];
    words.extend(globals.iter().map(String::as_str));
    run(env, &words)?;
    Ok(())
}

fn field<'a>(value: &'a Value, path: &str) -> &'a Value {
    value.pointer(path).unwrap_or(&Value::Null)
}

fn prompt(form: &Value, message: &str) {
    assert_eq!(
        field(form, "/method"),
        "elicitation/create",
        "delete-all requests elicitation"
    );
    let params = field(form, "/params");
    assert_eq!(field(params, "/mode"), "form", "the request uses form mode");
    assert_eq!(
        field(params, "/message"),
        message,
        "the message gives counts only"
    );
    assert_eq!(
        field(params, "/requestedSchema"),
        &json!({
            "type":"object","properties":{"confirm":{"type":"boolean"}},"required":["confirm"]
        }),
        "confirmation is one required boolean"
    );
}

fn stdio() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    save_search(&env, "private-search")?;
    run(
        &env,
        &[
            "saved",
            "list",
            "add",
            "private-list",
            "--entity",
            "gene",
            "BRAF",
        ],
    )?;
    run(
        &env,
        &["saved", "note", "--project", "default", "private-note"],
    )?;
    let file = saved_file(&env);
    let before = std::fs::read(&file).map_err(|e| e.to_string())?;
    let mut peer = legacy_peer(&env)?;

    for (id, result) in [
        (2, json!({"action":"decline"})),
        (3, json!({"action":"cancel"})),
        (4, json!({"action":"accept","content":{"confirm":false}})),
        (5, json!({"action":"accept","content":{"confirm":true}})),
    ] {
        peer.send(&delete(id, json!({"all":true})))?;
        let elicitation = peer.next()?;
        prompt(
            &elicitation,
            "Permanently delete all saved work? Searches (1), lists (1), notes (1).",
        );
        assert_eq!(
            std::fs::read(&file).map_err(|e| e.to_string())?,
            before,
            "asking leaves the file unchanged"
        );
        peer.send(&json!({"jsonrpc":"2.0","id":field(&elicitation, "/id"),"result":result}))?;
        let reply = peer.next()?;
        assert_eq!(
            field(&reply, "/id"),
            &json!(id),
            "the final reply correlates to the original call"
        );
        if id == 5 {
            let deleted = one_value(field(&reply, "/result"))?;
            assert_eq!(
                field(&deleted, "/result/searches/0/name"),
                "private-search",
                "accept returns the deleted search"
            );
        } else {
            assert_eq!(
                field(&reply, "/result/content/0/text"),
                "Nothing was deleted.",
                "the reply reports no deletion"
            );
            assert_eq!(
                std::fs::read(&file).map_err(|e| e.to_string())?,
                before,
                "declining leaves the file unchanged"
            );
        }
    }
    let shown: Value =
        serde_json::from_slice(&run(&env, &["saved", "show"])?).map_err(|e| e.to_string())?;
    assert_eq!(
        field(&shown, "/result"),
        &json!({"searches":[],"lists":[]}),
        "accept emptied the saved file"
    );
    named_deletions(&env, &mut peer)?;
    peer.finish()
}

fn legacy_peer(env: &Environment) -> Result<Peer, String> {
    let mut peer = Peer::with_globals(env, &["--tool-mode".into(), "typed".into()])?;
    peer.send(
        &json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
            "protocolVersion":"2025-11-25","clientInfo":{"name":"saved-proof","version":"1"},
            "capabilities":{"elicitation":{"form":{}}}
        }}),
    )?;
    let init = peer.next()?;
    assert_eq!(
        field(&init, "/result/protocolVersion"),
        "2025-11-25",
        "negotiates the legacy revision"
    );
    peer.send(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))?;

    Ok(peer)
}

fn named_deletions(env: &Environment, peer: &mut Peer) -> Result<(), String> {
    save_search(env, "one-search")?;
    let note: Value = serde_json::from_slice(&run(
        env,
        &["saved", "note", "--project", "default", "one-note"],
    )?)
    .map_err(|e| e.to_string())?;
    for (id, target, pointer, wanted) in [
        (
            6,
            json!({"name":"one-search"}),
            "/result/searches/0/name",
            json!("one-search"),
        ),
        (
            7,
            json!({"note":field(&note, "/result/user_notes/0/id")}),
            "/result/user_notes/0/text",
            json!("one-note"),
        ),
    ] {
        peer.send(&delete(id, target))?;
        let reply = peer.next()?;
        assert_eq!(
            field(&reply, "/id"),
            &json!(id),
            "a named deletion answers directly"
        );
        let deleted = one_value(field(&reply, "/result"))?;
        assert_eq!(
            deleted.pointer(pointer),
            Some(&wanted),
            "the specified record was deleted"
        );
    }
    Ok(())
}

const ACCOUNT: &str = "x-biomcp-account: test-account\r\n";

fn hosted_call(id: u32, args: Value) -> Value {
    request(
        json!(id),
        "tools/call",
        json!({"name":"saved","arguments":args}),
    )
}

fn hosted_delete(id: u32) -> Value {
    let mut frame = hosted_call(id, arguments(json!({"all":true})));
    if let Some(capabilities) =
        frame.pointer_mut("/params/_meta/io.modelcontextprotocol~1clientCapabilities")
    {
        *capabilities = json!({"elicitation":{"form":{}}});
    }
    frame
}

fn ask(server: &Server, id: u32) -> Result<(String, Value), String> {
    let reply = http_wire(server, &hosted_delete(id), ACCOUNT)?;
    let result = field(&reply, "/result");
    assert_eq!(
        field(result, "/resultType"),
        "input_required",
        "HTTP returns an input-required result"
    );
    let inputs = field(result, "/inputRequests")
        .as_object()
        .ok_or("missing input requests")?;
    assert_eq!(inputs.len(), 1, "delete-all requests one form");
    let (key, form) = inputs.iter().next().ok_or("missing form")?;
    prompt(
        form,
        "Permanently delete all saved work? Searches (0), lists (1), notes (0).",
    );
    assert!(
        field(result, "/requestState").is_string(),
        "the response includes opaque request state"
    );
    Ok((key.clone(), field(result, "/requestState").clone()))
}

fn accept(server: &Server, id: u32, (key, state): (String, Value)) -> Result<Value, String> {
    let mut frame = hosted_delete(id);
    let params = frame
        .get_mut("params")
        .and_then(Value::as_object_mut)
        .ok_or("missing tool params")?;
    params.insert("requestState".into(), state);
    params.insert(
        "inputResponses".into(),
        json!({(key):{"action":"accept","content":{"confirm":true}}}),
    );
    http_wire(server, &frame, ACCOUNT)
}

fn hosted() -> Result<(), String> {
    let folder = tempfile::tempdir().map_err(|e| e.to_string())?;
    let key = URL_SAFE_NO_PAD.encode(Sha256::digest(folder.path().as_os_str().as_encoded_bytes()));
    let start = || {
        Server::configured_globals(
            "127.0.0.1:0",
            Some(json!({"mcp_tool_mode":"typed"})),
            None,
            Some(("BIOMCP_TRIAL_CONTINUATION_KEY", &key)),
            None,
            &[
                "--hosted".into(),
                "--saved-folder".into(),
                folder.path().to_string_lossy().into_owned(),
            ],
        )
    };
    let mut first = start()?;
    let mut second = start()?;
    let add = |id, name| {
        let args = json!({
            "action":"list_add","list_add":{"name":name,"entity":"gene","ids":["BRAF"]}
        });
        http_wire(&first, &hosted_call(id, args), ACCOUNT)
    };

    add(1, "first")?;
    let waiting = ask(&first, 2)?;
    http_wire(
        &first,
        &hosted_call(3, arguments(json!({"name":"first"}))),
        ACCOUNT,
    )?;
    add(4, "replacement")?;
    let stale = accept(&second, 5, waiting)?;
    assert_eq!(
        field(&stale, "/result/content/0/text"),
        "Saved work changed or confirmation expired. Nothing was deleted.",
        "a different list with the same count needs its own confirmation"
    );
    let accepted = accept(&second, 7, ask(&first, 6)?)?;
    assert_eq!(
        field(&accepted, "/result/resultType"),
        "complete",
        "another worker can complete the retry"
    );
    let deleted = field(&accepted, "/result/structuredContent");
    assert_eq!(
        field(deleted, "/result/lists/0/name"),
        "replacement",
        "the approved list was deleted"
    );
    let shown = http_wire(&second, &hosted_call(8, json!({"action":"show"})), ACCOUNT)?;
    assert_eq!(
        field(&one_value(field(&shown, "/result"))?, "/result"),
        &json!({"searches":[],"lists":[]}),
        "the account's saved file is empty"
    );
    first.stop()?;
    second.stop()
}
