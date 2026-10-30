//! Optional site payload failures retain trial rows through the public command.
use super::super::{launch::Environment, nci::invoke, response_cases::copy_capture};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "trial-nci/nci-facility-site-boundaries",
        || boundaries().map_err(Into::into),
    )]
}
fn boundaries() -> Result<(), String> {
    let matching = json!({"org_name":"Dana-Farber Cancer Institute","org_city":"Boston"});
    for (sites, matches, field) in [
        (None, 0, "sites.org_name"),
        (Some(Value::Null), 0, "sites.org_name"),
        (Some(json!({"invalid":"collection"})), 0, "sites"),
        (Some(json!([false, matching.clone()])), 1, "sites"),
        (
            Some(json!([{"org_name":17}, matching])),
            1,
            "sites.org_name",
        ),
        (
            Some(json!([{"org_name":"Dana-Farber Cancer Institute","org_city":17}])),
            1,
            "sites.org_city",
        ),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = env.root().join("reply");
        copy_capture(
            "trial/nci/facility/synthetic-matches",
            &replay,
            |_, record| {
                if let Some(row) = record
                    .pointer_mut("/exchanges/0/body/value/data/0")
                    .and_then(Value::as_object_mut)
                {
                    row.remove("sites");
                    if let Some(sites) = &sites {
                        row.insert("sites".into(), sites.clone());
                    }
                }
            },
        )
        .map_err(|e| e.to_string())?;
        let mut args: Vec<String> = [
            "search",
            "trial",
            "--source",
            "nci",
            "--facility",
            "Dana-Farber Cancer Institute",
            "--limit",
            "1",
        ]
        .map(str::to_owned)
        .to_vec();
        args.extend(env.globals(Some(&replay), false));
        let answer = invoke(&env, &args, 0)?;
        if answer.pointer("/result/items/0/nct_id") != Some(&json!("NCT02296125"))
            || answer.pointer("/result/items/0/matching_sites") != Some(&json!(matches))
            || answer.pointer("/page/total") != Some(&json!(3))
            || !answer
                .get("problems")
                .and_then(Value::as_array)
                .is_some_and(|problems| {
                    problems
                        .iter()
                        .any(|p| p.get("field") == Some(&json!(field)))
                })
        {
            return Err(format!(
                "optional site shape lost trial rows or field problem: {sites:?}"
            ));
        }
    }
    Ok(())
}
