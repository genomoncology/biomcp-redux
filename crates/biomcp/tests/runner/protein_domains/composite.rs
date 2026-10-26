//! Feature protein-domains-complexes: all keeps independent first-page bounds.
use super::*;
use super::super::mcp;
const CLI_ALL_BYTES: usize = 18000;
const MCP_ALL_BYTES: usize = 17999;
pub(super) fn run() -> Result<(), String> {
    for id in ["P00533", "P38398"] {
        let stub = Stub::for_sources(&["rest.uniprot.org", "www.ebi.ac.uk"], |_| "application/json", move |request| {
            let mut value: Value = serde_json::from_str(&super::cards::reply(request)?).map_err(|e|e.to_string())?;
            if request.contains("/uniprotkb/") { *value.get_mut("primaryAccession").ok_or("identity")? = json!(id); }
            if let Some(protein) = value.pointer_mut("/results/0/proteins/0/accession") { *protein = json!(id.to_ascii_lowercase()); }
            Ok(value.to_string().replace("P38398", id))
        })?;
        let env = setup(&stub)?;
        let args = ["get", "protein", id, "all", "--section-limit", "1"];
        let directory = env.root().join("all-record");
        let mut recording = args.to_vec(); recording.extend(["--record", directory.to_str().ok_or("record path")?]);
        let cli = call(&env, &recording, 0)?;
        let mcp_env = Environment::new().map_err(|e|e.to_string())?;
        let globals = mcp_env.globals(Some(&directory), false);
        let mcp = mcp::call_with_globals(&mcp_env, &args.map(str::to_owned), false, &globals)?;
        for value in [&cli, &mcp] {
            let sections = value.pointer("/result/items/0/sections").and_then(Value::as_object).ok_or("all sections")?;
            for name in ["domains", "complexes", "features", "isoforms", "structures", "sequence", "function"] {
                let section = sections.get(name).ok_or("kept section missing")?;
                if section.pointer("/data/items").and_then(Value::as_array).is_some_and(|a|a.len()>1) { return Err(format!("{name} exceeded the common section limit")); }
            }
            super::card_failures::bounded(value, CLI_ALL_BYTES, MCP_ALL_BYTES)?;
        }
        let disabled = call(&env, &["get", "protein", id, "all", "--section-limit", "1", "--disable-sources", "interpro,complex-portal"], 0)?;
        for name in ["domains", "complexes"] {
            if disabled.pointer(&format!("/result/items/0/sections/{name}/state")) != Some(&json!("skipped")) { return Err("disabled optional all source supplied a cached page".into()); }
        }
        if stub.finish()?.len() != 3 { return Err("all repeated a source request or a disabled source reached HTTP".into()); }
    }
    Ok(())
}
