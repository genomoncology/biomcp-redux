//! Feature protein-domains-complexes: all keeps independent first-page bounds.
use super::super::mcp;
use super::{Environment, Stub, Value, call, json, setup};
// Synthetic P00533 and P38398 all measured in j15002, with fifteen percent headroom.
const CLI_ALL_BYTES: usize = (2826_usize * 115).div_ceil(100);
const MCP_ALL_BYTES: usize = (2825_usize * 115).div_ceil(100);
// Repeated and disabled envelopes measured in j15006, with fifteen percent headroom.
const CLI_REPEATED_BYTES: usize = 1528;
const MCP_REPEATED_BYTES: usize = 1527;
const CLI_DISABLED_BYTES: usize = 2323;
const MCP_DISABLED_BYTES: usize = 2322;
pub(super) fn run() -> Result<(), String> {
    for id in ["P00533", "P38398"] {
        let stub = Stub::for_sources(
            &["rest.uniprot.org", "www.ebi.ac.uk"],
            |_| "application/json",
            move |request| reply(request, id),
        )?;
        let env = setup(&stub)?;
        let args = ["get", "protein", id, "all", "--section-limit", "1"];
        let directory = env.root().join("all-record");
        let mut recording = args.to_vec();
        recording.extend(["--record", directory.to_str().ok_or("record path")?]);
        let cli = call(&env, &recording, 0)?;
        let mcp_env = Environment::new().map_err(|e| e.to_string())?;
        let globals = mcp_env.globals(Some(&directory), false);
        let mcp = mcp::call_with_globals(&mcp_env, &args.map(str::to_owned), false, &globals)?;

        for value in [&cli, &mcp] {
            check(value)?;
        }
        let repeated = call(
            &env,
            &[
                "get",
                "protein",
                id,
                "features",
                "features",
                "--section-limit",
                "1",
            ],
            0,
        )?;
        if repeated.pointer("/query/section") != Some(&json!(["features", "features"]))
            || repeated
                .pointer("/result/items/0/sections/features/data/items")
                .and_then(Value::as_array)
                .map(Vec::len)
                != Some(1)
        {
            return Err("repeated section changed shared grammar behavior".into());
        }

        let repeated_mcp = mcp::call_with_globals(
            &mcp_env,
            &[
                "get",
                "protein",
                id,
                "features",
                "features",
                "--section-limit",
                "1",
            ]
            .map(str::to_owned),
            false,
            &globals,
        )?;

        super::card_failures::bounded(&repeated, CLI_REPEATED_BYTES, MCP_REPEATED_BYTES)?;
        super::card_failures::bounded(&repeated_mcp, CLI_REPEATED_BYTES, MCP_REPEATED_BYTES)?;
        disabled(&env, &mcp_env, &directory, id)?;
        if stub.finish()?.len() != 3 {
            return Err("all repeated a source request or a disabled source reached HTTP".into());
        }
    }
    Ok(())
}

fn reply(request: &str, id: &str) -> Result<String, String> {
    let mut value: Value =
        serde_json::from_str(&super::cards::reply(request)?).map_err(|e| e.to_string())?;
    if request.contains("/uniprotkb/") {
        *value.get_mut("primaryAccession").ok_or("identity")? = json!(id);
        *value.get_mut("features").ok_or("features")? = json!(
            [ {"type":"Domain","description":"Synthetic first","location":{"start":{"value":1,"modifier":"EXACT"},"end":{"value":2,
            "modifier":"EXACT"}}},{"type":"Region","description":"Synthetic second","location":{"start":{"value":3,"modifier":"EXACT"},
            "end":{"value":4,"modifier":"EXACT"}}}]
        );
        *value.get_mut("comments").ok_or("comments")? = json!([
                    {"commentType":"FUNCTION","texts":[{"value":"Whole synthetic first paragraph."}]},
                    {"commentType":"FUNCTION","molecule":"Isoform 2","texts":[{"value":"Whole synthetic second paragraph."}]},
                    {"commentType":"ALTERNATIVE PRODUCTS","isoforms":[
                        {"name":{"value":"First"},"isoformIds":["P38398-1"],"isoformSequenceStatus":"Displayed"},
                        {"name":{"value":"Second"},"isoformIds":["P38398-2"],"isoformSequenceStatus":"Described"}]}]);
        *value
            .get_mut("uniProtKBCrossReferences")
            .ok_or("structures")? =
            json!([{"database":"PDB","id":"1ABC"},{"database":"PDB","id":"2ABC"}]);
    }
    if let Some(rows) = value.get_mut("results").and_then(Value::as_array_mut) {
        let mut second = rows.first().cloned().ok_or("first domain")?;
        *second
            .pointer_mut("/metadata/accession")
            .ok_or("second IPR")? = json!("IPR010991");
        rows.push(second);
        for row in rows {
            *row.pointer_mut("/proteins/0/accession")
                .ok_or("protein accession")? = json!(id.to_ascii_lowercase());
        }
        *value.get_mut("count").ok_or("domain count")? = json!(2);
    }
    Ok(value.to_string().replace("P38398", id))
}

fn check(value: &Value) -> Result<(), String> {
    let sections = value
        .pointer("/result/items/0/sections")
        .and_then(Value::as_object)
        .ok_or("all sections")?;
    for name in [
        "domains",
        "complexes",
        "features",
        "isoforms",
        "structures",
        "sequence",
        "function",
    ] {
        let section = sections.get(name).ok_or("kept section missing")?;
        if name != "function"
            && section
                .pointer("/data/items")
                .and_then(Value::as_array)
                .is_some_and(|a| a.len() != 1)
        {
            return Err(format!("{name} exceeded the common section limit"));
        }
    }
    for name in ["domains", "complexes", "features", "isoforms", "structures"] {
        if sections
            .get(name)
            .and_then(|s| s.pointer("/data/items"))
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(1)
        {
            return Err(format!("{name} did not return exactly its first row"));
        }
    }
    if sections
        .get("function")
        .and_then(|s| s.pointer("/data/items"))
        .and_then(Value::as_array)
        .map(Vec::len)
        != Some(2)
    {
        return Err("function text lost a whole source paragraph".into());
    }
    super::card_failures::bounded(value, CLI_ALL_BYTES, MCP_ALL_BYTES)?;
    Ok(())
}

fn disabled(
    env: &Environment,
    mcp_env: &Environment,
    replay: &std::path::Path,
    id: &str,
) -> Result<(), String> {
    let args = [
        "get",
        "protein",
        id,
        "all",
        "--section-limit",
        "1",
        "--disable-sources",
        "interpro,complex-portal",
    ];
    let cli = call(env, &args, 0)?;
    let mcp = mcp::call_with_globals(
        mcp_env,
        &args.map(str::to_owned),
        false,
        &mcp_env.globals(Some(replay), false),
    )?;

    for value in [&cli, &mcp] {
        for name in ["domains", "complexes"] {
            if value.pointer(&format!("/result/items/0/sections/{name}/state"))
                != Some(&json!("skipped"))
            {
                return Err("disabled optional all source supplied a cached page".into());
            }
        }
        super::card_failures::bounded(value, CLI_DISABLED_BYTES, MCP_DISABLED_BYTES)?;
    }
    Ok(())
}
