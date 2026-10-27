use super::*;
pub(super) fn run() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["www.ebi.ac.uk", "rest.uniprot.org"],
        |_| "application/json",
        reply,
    )?;
    let env = setup(&stub)?;
    let card = call(&env, &["get", "protein", "P38398"], 0)?;
    super::super::protein_budgets::check(&card, "synthetic-card/card", "cli")?;
    for section in ["domains", "complexes"] {
        if !card
            .get("next")
            .and_then(Value::as_array)
            .ok_or("card actions")?
            .iter()
            .any(|s| {
                s.get("args")
                    .and_then(Value::as_array)
                    .is_some_and(|args| args.contains(&json!(section)))
            })
        {
            return Err(format!("plain card omitted {section} action"));
        }
    }
    let domains = call(&env, &["get", "protein", "P38398", "domains"], 0)?;
    super::super::protein_budgets::check(&domains, "synthetic-card/domains", "cli")?;
    if domains.pointer("/result/items/0/sections/domains/data/items/0/id")
        != Some(&json!("IPR002117"))
    {
        return Err("domain section differs from native match page".into());
    }
    let complexes = call(&env, &["get", "protein", "P38398", "complexes"], 0)?;
    super::super::protein_budgets::check(&complexes, "synthetic-card/complexes", "cli")?;
    if complexes.pointer("/result/items/0/sections/complexes/data/native_total") != Some(&json!(3))
    {
        return Err("complex section lost its independent native count".into());
    }
    let all = call(&env, &["get", "protein", "P38398", "all"], 0)?;
    super::super::protein_budgets::check(&all, "synthetic-card/all", "cli")?;
    if all.pointer("/result/items/0/sections/domains/state") != Some(&json!("data"))
        || all.pointer("/result/items/0/sections/complexes/state") != Some(&json!("data"))
    {
        return Err("all omitted an available source section".into());
    }
    policy(&env)?;
    if stub.finish()?.len() != 3 {
        return Err("plain cards or retained sections made extra source requests".into());
    }
    Ok(())
}
fn policy(env: &Environment) -> Result<(), String> {
    let skipped = call(
        env,
        &[
            "get",
            "protein",
            "P38398",
            "domains",
            "--disable-sources",
            "interpro",
        ],
        0,
    )?;
    super::super::protein_budgets::check(&skipped, "synthetic-card/disabled-interpro", "cli")?;
    if skipped.pointer("/result/items/0/sections/domains/state") != Some(&json!("skipped")) {
        return Err("disabled source supplied a cached section".into());
    }
    let restricted = call(
        env,
        &[
            "get",
            "protein",
            "P38398",
            "--disable-sources",
            "interpro,complex-portal",
        ],
        0,
    )?;
    super::super::protein_budgets::check(&restricted, "synthetic-card/disabled-relations", "cli")?;
    let actions = restricted
        .get("next")
        .and_then(Value::as_array)
        .ok_or("restricted actions")?;
    if actions.iter().any(|s| {
        s.get("args").and_then(Value::as_array).is_some_and(|args| {
            args.contains(&json!("domains")) || args.contains(&json!("complexes"))
        })
    }) {
        return Err("disabled source card action remained available".into());
    }
    Ok(())
}

pub(super) fn reply(request: &str) -> Result<String, String> {
    if request.contains("/uniprotkb/") {
        return Ok(json!(
            {"primaryAccession":"P38398","uniProtkbId":"SYNTHETIC_HUMAN","entryType":"UniProtKB reviewed (Swiss-Prot)",
            "organism":{"taxonId":9606},"proteinDescription":{"recommendedName":{"fullName":{"value":"Synthetic protein"}}},
            "sequence":{"length":30,"value":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"features":[],"comments":[],"uniProtKBCrossReferences":[]}
        )
        .to_string());
    }
    if request.contains("/interpro/") {
        return Ok(json!(
            {"count":1,"next":null,"results":[{"metadata":{"accession":"IPR002117","name":"Native family","source_database":"interpro",
            "type":"family","member_databases":{}},"proteins":[{"accession":"p38398","protein_length":30,"organism":"9606",
            "entry_protein_locations":[]}]}]}
        )
        .to_string());
    }
    super::complexes::reply(request)
}
