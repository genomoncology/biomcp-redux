//! Exercise saved-work confirmation against the installed stdio and hosted HTTP server.
use super::{
    http::Server,
    launch::{Environment, fixtures},
    mcp::{http_wire, one_value, request},
    publication::Peer,
    saved_files::{run, saved_file},
    transcript::trial,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};

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

fn prompt(form: &Value, message: &str) {
    assert_eq!(
        form["method"], "elicitation/create",
        "delete-all requests elicitation"
    );
    let params = &form["params"];
    assert_eq!(params["mode"], "form", "the request uses form mode");
    assert_eq!(params["message"], message, "the message gives counts only");
    assert_eq!(
        params["requestedSchema"],
        json!({
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
    let mut peer = Peer::with_globals(&env, &["--tool-mode".into(), "typed".into()])?;
    peer.send(
        &json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
            "protocolVersion":"2025-11-25","clientInfo":{"name":"saved-proof","version":"1"},
            "capabilities":{"elicitation":{"form":{}}}
        }}),
    )?;
    let init = peer.next()?;
    assert_eq!(
        init["result"]["protocolVersion"], "2025-11-25",
        "negotiates the legacy revision"
    );
    peer.send(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))?;

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
            "Permanently delete 1 searches, 1 lists and 1 notes?",
        );
        assert_eq!(
            std::fs::read(&file).map_err(|e| e.to_string())?,
            before,
            "asking leaves the file unchanged"
        );
        peer.send(&json!({"jsonrpc":"2.0","id":elicitation["id"],"result":result}))?;
        let reply = peer.next()?;
        assert_eq!(
            reply["id"], id,
            "the final reply correlates to the original call"
        );
        if id == 5 {
            let deleted = one_value(&reply["result"])?;
            assert_eq!(
                deleted["result"]["searches"][0]["name"], "private-search",
                "accept returns the deleted search"
            );
        } else {
            assert_eq!(
                reply["result"]["content"][0]["text"], "Nothing was deleted.",
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
        shown["result"],
        json!({"searches":[],"lists":[]}),
        "accept emptied the saved file"
    );
    named_deletions(&env, &mut peer)?;
    peer.finish()
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
            json!({"note":note["result"]["user_notes"][0]["id"]}),
            "/result/user_notes/0/text",
            json!("one-note"),
        ),
    ] {
        peer.send(&delete(id, target))?;
        let reply = peer.next()?;
        assert_eq!(reply["id"], id, "a named deletion answers directly");
        let deleted = one_value(&reply["result"])?;
        assert_eq!(
            deleted.pointer(pointer),
            Some(&wanted),
            "the specified record was deleted"
        );
    }
    Ok(())
}

fn hosted() -> Result<(), String> {
    let folder = tempfile::tempdir().map_err(|e| e.to_string())?;
    let mut server = Server::configured_globals(
        "127.0.0.1:0",
        Some(json!({"mcp_tool_mode":"typed"})),
        None,
        None,
        None,
        &[
            "--hosted".into(),
            "--saved-folder".into(),
            folder.path().to_string_lossy().into_owned(),
        ],
    )?;
    let account = "x-biomcp-account: test-account\r\n";
    let call = |id, args| {
        request(
            json!(id),
            "tools/call",
            json!({"name":"saved","arguments":args}),
        )
    };
    let add = |id, name| {
        http_wire(
            &server,
            &call(
                id,
                json!({
                    "action":"list_add","list_add":{"name":name,"entity":"gene","ids":["BRAF"]}
                }),
            ),
            account,
        )
    };
    let start = |id| {
        let mut frame = call(id, arguments(json!({"all":true})));
        frame["params"]["_meta"]["io.modelcontextprotocol/clientCapabilities"]["elicitation"] =
            json!({"form":{}});
        frame
    };
    let ask = |id| -> Result<(String, Value), String> {
        let reply = http_wire(&server, &start(id), account)?;
        assert_eq!(
            reply["result"]["resultType"], "input_required",
            "HTTP returns an input-required result"
        );
        let result = &reply["result"];
        let inputs = result["inputRequests"]
            .as_object()
            .ok_or("missing input requests")?;
        assert_eq!(inputs.len(), 1, "delete-all requests one form");
        let (key, form) = inputs.iter().next().ok_or("missing form")?;
        prompt(form, "Permanently delete 0 searches, 1 lists and 0 notes?");
        assert!(
            result["requestState"].is_string(),
            "the response includes opaque request state"
        );
        Ok((key.clone(), result["requestState"].clone()))
    };
    let retry = |id, (key, state): (String, Value)| {
        let mut frame = start(id);
        frame["params"]["requestState"] = state;
        frame["params"]["inputResponses"] =
            json!({(key):{"action":"accept","content":{"confirm":true}}});
        http_wire(&server, &frame, account)
    };

    add(1, "first")?;
    let waiting = ask(2)?;
    http_wire(
        &server,
        &call(3, arguments(json!({"name":"first"}))),
        account,
    )?;
    add(4, "replacement")?;
    let stale = retry(5, waiting)?;
    assert_eq!(
        stale["result"]["content"][0]["text"],
        "Saved work changed or confirmation expired. Nothing was deleted.",
        "a different list with the same count needs its own confirmation"
    );
    let accepted = retry(7, ask(6)?)?;
    assert_eq!(
        accepted["result"]["resultType"], "complete",
        "the retry completes the original call"
    );
    let deleted = one_value(&accepted["result"])?;
    assert_eq!(
        deleted["result"]["lists"][0]["name"], "replacement",
        "the approved list was deleted"
    );
    let shown = http_wire(&server, &call(8, json!({"action":"show"})), account)?;
    assert_eq!(
        one_value(&shown["result"])?["result"],
        json!({"searches":[],"lists":[]}),
        "the account's saved file is empty"
    );
    server.stop()
}
