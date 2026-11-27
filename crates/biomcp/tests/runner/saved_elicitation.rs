use super::{
    launch::{Environment, fixtures},
    mcp::one_value,
    publication::Peer,
    saved_files::{run, saved_file},
    transcript::trial,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![trial("saved-work/stdio-delete-all-confirmation", || {
        stdio().map_err(Into::into)
    })]
}

fn delete(id: u32, target: Value) -> Value {
    json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":{
        "name":"saved","arguments":{"action":"delete","delete":target}
    }})
}

fn stdio() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = fixtures().join("replies/trial/condition");
    let globals = env.globals(Some(&replay), false);
    let mut search = vec!["search", "trial", "--condition", "melanoma", "--save-as", "private-search"];
    search.extend(globals.iter().map(String::as_str));
    run(&env, &search)?;
    run(&env, &["saved", "list", "add", "private-list", "--entity", "gene", "BRAF"])?;
    run(&env, &["saved", "note", "--project", "default", "private-note"])?;

    let file = saved_file(&env);
    let before = std::fs::read(&file).map_err(|e| e.to_string())?;
    let mut peer = Peer::with_globals(&env, &["--tool-mode".into(), "typed".into()])?;
    peer.send(&json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
        "protocolVersion":"2025-11-25","clientInfo":{"name":"saved-proof","version":"1"},
        "capabilities":{"elicitation":{"form":{}}}
    }}))?;
    let init = peer.next()?;
    assert_eq!(init["result"]["protocolVersion"], "2025-11-25");
    peer.send(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))?;

    for (id, action) in [(2, "decline"), (3, "cancel"), (4, "accept")] {
        peer.send(&delete(id, json!({"all":true})))?;
        let prompt = peer.next()?;
        assert_eq!(prompt["method"], "elicitation/create");
        assert_eq!(prompt["params"]["mode"], "form");
        assert_eq!(prompt["params"]["message"], "Permanently delete 1 searches, 1 lists and 1 notes?");
        assert_eq!(prompt["params"]["requestedSchema"]["properties"], json!({"confirm":{"type":"boolean"}}));
        assert_eq!(std::fs::read(&file).map_err(|e| e.to_string())?, before);
        let mut result = json!({"action":action});
        if action == "accept" {
            result["content"] = json!({"confirm":true});
        }
        peer.send(&json!({"jsonrpc":"2.0","id":prompt["id"],"result":result}))?;
        let reply = peer.next()?;
        assert_eq!(reply["id"], id);
        if action == "accept" {
            assert_eq!(one_value(&reply["result"])?["result"]["searches"][0]["name"], "private-search");
        } else {
            assert_eq!(reply["result"]["content"][0]["text"], "Nothing was deleted.");
            assert_eq!(std::fs::read(&file).map_err(|e| e.to_string())?, before);
        }
    }
    let shown: Value = serde_json::from_slice(&run(&env, &["saved", "show"])?).map_err(|e| e.to_string())?;
    assert_eq!(shown["result"], json!({"searches":[],"lists":[]}));
    peer.finish()
}
