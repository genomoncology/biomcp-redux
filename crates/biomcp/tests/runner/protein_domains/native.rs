use super::{Environment, Stub, Value, call, fs, json, setup};
use std::{collections::BTreeMap, io::Write, path::Path};
pub(super) fn run(root: &Path) -> Result<(), String> {
    let requests: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read(root.join("requests.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let stub = Stub::for_sources(
        &["www.ebi.ac.uk", "rest.uniprot.org"],
        |_| "application/json",
        move |request| {
            let path = request
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1))
                .ok_or("request path")?;
            requests
                .get(path)
                .cloned()
                .ok_or_else(|| format!("no accepted native request for {path}"))
        },
    )?;
    let replay = root.join("replay");
    fs::create_dir_all(&replay).map_err(|e| e.to_string())?;
    let output = root.join("outputs");
    fs::create_dir_all(&output).map_err(|e| e.to_string())?;
    retained_outputs(&output)?;
    let registry = super::super::schema::Registry::embedded()?;
    let mut paths = fs::read_dir(root.join("cases"))
        .map_err(|e| e.to_string())?
        .map(|p| p.map(|p| p.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    for path in paths {
        let case: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let args = case
            .pointer("/steps/0/run")
            .and_then(Value::as_array)
            .ok_or("native command")?
            .iter()
            .map(|v| v.as_str().ok_or("argument"))
            .collect::<Result<Vec<_>, _>>()?;
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("case name")?;
        if root.join(format!("{name}.done")).is_file() {
            continue;
        }
        let env = setup(&stub)?;
        let captured = env.root().join("recorded");
        let mut record = args.clone();
        record.extend(["--record", captured.to_str().ok_or("native recording")?]);
        let recorded = call(&env, &record, 0)?;
        copy_recordings(&captured, &replay)?;
        let value = super::super::run_case(&path, &registry)?;
        flags(name, &value)?;
        measure(name, &value, &output)?;
        if name.starts_with("complexes-") || name.starts_with("domains-") {
            if name == "complexes-P15056" {
                members_from_complex(&env, &recorded, &output)?;
            }
            walk(&env, recorded, name, &output)?;
        }
        fs::write(root.join(format!("{name}.done")), b"passed").map_err(|e| e.to_string())?;
    }
    let seen = stub.finish()?;
    fs::write(root.join("request-count.txt"), seen.len().to_string()).map_err(|e| e.to_string())?;
    Ok(())
}
fn measure(name: &str, value: &Value, output: &Path) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    let comparable = super::super::source_terms::size(value, "")?;
    let budgets: BTreeMap<String, [usize; 2]> = serde_json::from_str(
        &crate::runner::launch::fixture_text("protein-domain-budgets.json")?,
    )
    .map_err(|e| e.to_string())?;
    let [full, small] = budgets
        .get(name)
        .ok_or_else(|| format!("unmeasured native protein case: {name}"))?;
    if bytes.len() + 1 > (full * 115).div_ceil(100) || comparable > (small * 115).div_ceil(100) {
        return Err(format!(
            "{name}: native output exceeds its measured fifteen-percent ceiling"
        ));
    }

    fs::write(output.join(format!("{name}.json")), bytes.as_slice()).map_err(|e| e.to_string())?;
    writeln!(
        std::io::stdout(),
        "PROTEIN_RELATIONS {name} CLI={} MCP={} comparable={comparable} ceiling={}",
        bytes.len() + 1,
        bytes.len(),
        (comparable * 115).div_ceil(100)
    )
    .map_err(|e| e.to_string())
}
fn walk(env: &Environment, mut value: Value, name: &str, output: &Path) -> Result<(), String> {
    let total = value
        .pointer("/page/total")
        .and_then(Value::as_u64)
        .ok_or("native total")?;
    let pool = value.pointer("/page/pool").cloned().ok_or("native pool")?;
    let mut ids = Vec::new();
    let mut page = 0;
    loop {
        ids.extend(
            value
                .pointer("/result/items")
                .and_then(Value::as_array)
                .ok_or("native rows")?
                .iter()
                .map(|v| v.get("id").cloned().ok_or("native row id"))
                .collect::<Result<Vec<_>, _>>()?,
        );
        if ids.len() as u64 >= total {
            break;
        }
        let action = super::super::action(&value, "page")?;
        let words: Vec<_> = action.iter().map(String::as_str).collect();
        value = call(env, &words, 0)?;
        page += 1;
        if value.pointer("/page/pool") != Some(&pool) {
            return Err("native continuation changed publication identity or lifetime".into());
        }
        flags(name, &value)?;
        measure(&format!("{name}-page-{page}"), &value, output)?;
    }
    let unique = ids
        .iter()
        .map(Value::to_string)
        .collect::<std::collections::BTreeSet<_>>();
    if ids.len() as u64 != total || unique.len() != ids.len() {
        return Err("native walk has a gap or duplicate".into());
    }
    if name == "complexes-P15056" && ids.contains(&json!("CPX-26353")) {
        return Err("BRAF description admitted an unrelated complex".into());
    }
    if name == "complexes-P38398"
        && !["CPX-715", "CPX-845", "CPX-955"]
            .iter()
            .all(|s| ids.contains(&json!(s)))
    {
        return Err("BRCA1 native memberships lost".into());
    }
    Ok(())
}

fn flags(name: &str, value: &Value) -> Result<(), String> {
    if name != "complexes-P38398" {
        return Ok(());
    }
    for row in value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("complex rows")?
    {
        let id = row
            .get("id")
            .and_then(Value::as_str)
            .ok_or("complex identity")?;
        let expected = match id {
            "CPX-715" | "CPX-845" | "CPX-955" => false,
            "CPX-14676" | "CPX-24321" | "CPX-26111" => true,
            _ => return Err("unexpected BRCA1 complex".into()),
        };
        if row.get("predicted") != Some(&json!(expected)) {
            return Err(format!("native predicted flag changed for {id}"));
        }
    }
    Ok(())
}

fn members_from_complex(env: &Environment, value: &Value, output: &Path) -> Result<(), String> {
    let action = super::super::action(value, "section")?;
    let words: Vec<_> = action.iter().map(String::as_str).collect();
    let members = call(env, &words, 0)?;
    if members.pointer("/page/total") != Some(&json!(93)) {
        return Err("native CPX-12671 participant total changed".into());
    }
    measure("members-CPX-12671", &members, output)?;
    walk(env, members, "members-CPX-12671", output)
}

fn copy_recordings(from: &Path, to: &Path) -> Result<(), String> {
    for source in fs::read_dir(from).map_err(|e| e.to_string())? {
        let source = source.map_err(|e| e.to_string())?;
        if !source.file_type().map_err(|e| e.to_string())?.is_dir() {
            continue;
        }
        let destination = to.join(source.file_name());
        fs::create_dir_all(&destination).map_err(|e| e.to_string())?;
        for file in fs::read_dir(source.path()).map_err(|e| e.to_string())? {
            let file = file.map_err(|e| e.to_string())?;
            fs::copy(file.path(), destination.join(file.file_name())).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn retained_outputs(output: &Path) -> Result<(), String> {
    for path in fs::read_dir(output).map_err(|e| e.to_string())? {
        let path = path.map_err(|e| e.to_string())?.path();
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("retained output name")?;
        let value: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if name == "get-all" {
            for section in ["features", "isoforms", "structures", "sequence", "function"] {
                if value.pointer(&format!("/result/items/0/sections/{section}/state"))
                    != Some(&json!("data"))
                {
                    return Err(format!("composite all lost {section}"));
                }
            }
        }
        measure(name, &value, output)?;
    }
    Ok(())
}
