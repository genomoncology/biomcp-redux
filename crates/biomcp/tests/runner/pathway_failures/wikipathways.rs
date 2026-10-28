//! Synthetic sync faults keep both previously accepted catalogs intact.
use super::{Check, Environment, call, fixtures};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial(
            "pathway-failures/synthetic-wikipathways-rejected-identity",
            || rejected_identity().map_err(Into::into),
        ),
        transcript_trial(
            "pathway-failures/synthetic-wikipathways-human-scope-absence",
            || identities().map_err(Into::into),
        ),
        transcript_trial("pathway-failures/synthetic-wikipathways-sync", || {
            failures().map_err(Into::into)
        }),
        transcript_trial(
            "pathway-failures/synthetic-partial-independent-legs",
            || partial().map_err(Into::into),
        ),
    ]
}
fn failures() -> Check {
    let template = Environment::new().map_err(|e| e.to_string())?;
    let root = fixtures::seed(&template)?;
    for (label, status, body, delay) in [
        ("429", 429, "{}", 0),
        ("500", 500, "{}", 0),
        ("timeout", 200, "{}", 121000),
        ("invalid JSON", 200, "{", 0),
        ("HTML success", 200, "<html>upstream failed</html>", 0),
        ("oversized", 200, "", 0),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        fixtures::replace_source(
            &root,
            "wikipathways",
            "getPathwayInfo",
            &fixtures::catalog(false).to_string(),
            200,
            0,
        )?;
        call(
            "pathway_failures/wikipathways/failures/1",
            &env,
            &root,
            &["sync", "wikipathways"],
            0,
        )?;
        let prior = call(
            "pathway_failures/wikipathways/failures/2",
            &env,
            &root,
            &["get", "pathway", "WP382"],
            0,
        )?;
        let huge;
        let body = if label == "oversized" {
            huge = " ".repeat(8 * 1024 * 1024 + 1);
            &huge
        } else {
            body
        };
        fixtures::replace_source(&root, "wikipathways", "getPathwayInfo", body, status, delay)?;
        let failed = call(
            "pathway_failures/wikipathways/failures/3",
            &env,
            &root,
            &["sync", "wikipathways"],
            4,
        )?;
        if failed.get("status") != Some(&json!("failed"))
            || failed.pointer("/sources/0/state") != Some(&json!("failed"))
            || failed.pointer("/sources/0/source") != Some(&json!("wikipathways"))
            || failed.pointer("/sources/0/terms") != Some(&json!("CC0 1.0 waiver"))
        {
            return Err(format!(
                "{label}: sync did not report its failed source receipt: {failed}"
            ));
        }
        let retained = call(
            "pathway_failures/wikipathways/failures/4",
            &env,
            &root,
            &["get", "pathway", "WP382"],
            0,
        )?;
        if retained.get("result") != prior.get("result") {
            return Err(format!("{label}: failed replacement changed old copy"));
        }
    }
    Ok(())
}
fn partial() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let root = fixtures::seed(&env)?;
    call(
        "pathway_failures/wikipathways/partial/1",
        &env,
        &root,
        &["sync", "wikipathways"],
        0,
    )?;
    fixtures::replace(&root, "/search/query", "{}", 500, 0)?;
    let value = call(
        "pathway_failures/wikipathways/partial/2",
        &env,
        &root,
        &[
            "search",
            "pathway",
            "Synthetic",
            "--source",
            "reactome,wikipathways",
        ],
        0,
    )?;
    let sources = value
        .get("sources")
        .and_then(Value::as_array)
        .ok_or("sources")?;
    if value.get("status") != Some(&json!("partial"))
        || value.pointer("/result/items/0/id/source") != Some(&json!("wikipathways"))
        || !sources.iter().any(|v| {
            v.get("source") == Some(&json!("reactome")) && v.get("state") == Some(&json!("failed"))
        })
        || !sources.iter().any(|v| {
            v.get("source") == Some(&json!("wikipathways")) && v.get("state") == Some(&json!("ok"))
        })
    {
        return Err(format!(
            "independent successful/failed legs lost their outcome: {value}"
        ));
    }
    Ok(())
}
fn identities() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let root = fixtures::seed(&env)?;
    let mut search = fixtures::catalog(true);
    let mut get = fixtures::catalog(false);
    for catalog in [&mut search, &mut get] {
        *catalog
            .pointer_mut("/pathwayInfo/0/species")
            .ok_or("species")? = json!("Mus musculus");
    }
    fixtures::replace_source(
        &root,
        "wikipathways",
        "findPathwaysByText",
        &search.to_string(),
        200,
        0,
    )?;
    fixtures::replace_source(
        &root,
        "wikipathways",
        "getPathwayInfo",
        &get.to_string(),
        200,
        0,
    )?;
    let synced = call(
        "pathway_failures/wikipathways/identities/1",
        &env,
        &root,
        &["sync", "wikipathways"],
        0,
    )?;
    if synced.pointer("/result/get/human_rows") != Some(&json!(0))
        || synced.pointer("/result/get/total_rows") != Some(&json!(1))
    {
        return Err(format!(
            "catalog discarded native nonhuman row or admitted it as human: {synced}"
        ));
    }
    for words in [
        &["get", "pathway", "WP382"][..],
        &["get", "pathway", "WP99999"][..],
    ] {
        let absent = call(
            "pathway_failures/wikipathways/identities/2",
            &env,
            &root,
            words,
            1,
        )?;
        if absent.get("status") != Some(&json!("absent"))
            || absent.get("result").is_some_and(|v| !v.is_null())
            || absent
                .get("problems")
                .and_then(Value::as_array)
                .is_some_and(|v| !v.is_empty())
        {
            return Err(format!(
                "human-scope absence gained a card or identity conflict: {absent}"
            ));
        }
    }
    let found = call(
        "pathway_failures/wikipathways/identities/3",
        &env,
        &root,
        &["search", "pathway", "Synthetic", "--source", "wikipathways"],
        1,
    )?;
    if found.pointer("/result/page/total") != Some(&json!(0))
        || found.pointer("/result/items").is_some()
    {
        return Err(format!("foreign species entered a result pool: {found}"));
    }
    Ok(())
}

fn rejected_identity() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let root = fixtures::seed(&env)?;
    call(
        "pathway_failures/wikipathways/rejected_identity/1",
        &env,
        &root,
        &["sync", "wikipathways"],
        0,
    )?;
    let prior = call(
        "pathway_failures/wikipathways/rejected_identity/2",
        &env,
        &root,
        &["get", "pathway", "WP382"],
        0,
    )?;
    for id in ["WP999", "bad-id"] {
        let mut catalog = fixtures::catalog(false);
        *catalog.pointer_mut("/pathwayInfo/0/id").ok_or("identity")? = json!(id);
        fixtures::replace_source(
            &root,
            "wikipathways",
            "getPathwayInfo",
            &catalog.to_string(),
            200,
            0,
        )?;
        let rejected = call(
            "pathway_failures/wikipathways/rejected_identity/3",
            &env,
            &root,
            &["sync", "wikipathways"],
            4,
        )?;
        if rejected.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
            || !rejected
                .get("problems")
                .and_then(Value::as_array)
                .is_some_and(|v| {
                    v.iter().any(|p| {
                        p.get("source") == Some(&json!("wikipathways"))
                            && p.get("field") == Some(&json!("catalog"))
                            && p.get("code") == Some(&json!("collection.incomplete"))
                    })
                })
        {
            return Err(format!(
                "rejected identity lost its failed receipt/catalog problem: {rejected}"
            ));
        }
        let retained = call(
            "pathway_failures/wikipathways/rejected_identity/4",
            &env,
            &root,
            &["get", "pathway", "WP382"],
            0,
        )?;
        if retained.get("result") != prior.get("result") {
            return Err("rejected identity changed the old catalog pair".into());
        }
        let absent = call(
            "pathway_failures/wikipathways/rejected_identity/5",
            &env,
            &root,
            &["get", "pathway", "WP999"],
            1,
        )?;
        if absent.get("result").is_some_and(|v| !v.is_null()) {
            return Err("rejected identity entered get catalog".into());
        }
    }
    Ok(())
}
