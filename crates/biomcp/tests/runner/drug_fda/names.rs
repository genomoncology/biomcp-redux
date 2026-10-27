//! Feature drug-approvals: replay keeps application identity and exact shared brand names.
use super::{Check, application, both, invoke, setup};
use libtest_mimic::Trial;
use serde_json::json;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "drug-approvals/application-search-names-replay",
        || names().map_err(Into::into),
    )]
}

fn names() -> Check {
    for brands in [
        vec!["Synthetic brand"],
        vec!["Synthetic brand", "Synthetic brand"],
        vec!["Synthetic brand", "SYNTHETIC BRAND"],
        vec![],
    ] {
        let mut app = application();
        let product = app["products"][0].clone();
        app["products"] = brands
            .iter()
            .enumerate()
            .map(|(n, brand)| {
                let mut product = product.clone();
                product["product_number"] = json!(format!("{n:03}"));
                product["brand_name"] = json!(brand);
                product
            })
            .collect();
        let (env, stub) = setup(
            json!({"meta":{"results":{"skip":0,"limit":1,"total":1}},"results":[app.clone()]}),
            "products.active_ingredients.name%3A%22Synthetic%22",
        )?;
        let recording = env.root().join("recording");
        let args = [
            "search",
            "drug",
            "Synthetic",
            "--source",
            "drugsfda",
            "--limit",
            "1",
        ];
        let mut record = vec!["--record".into(), recording.to_string_lossy().into_owned()];
        record.extend(args.map(str::to_owned));
        invoke(&env, record, 0)?;
        if stub.finish()?.len() != 1 {
            return Err("synthetic recording did not make exactly one local request".into());
        }
        let mut replay = vec!["--replay".into(), recording.to_string_lossy().into_owned()];
        replay.extend(args.map(str::to_owned));
        let value = both(&env, replay, 0)?;
        let item = value
            .pointer("/result/items/0")
            .ok_or("application candidate")?;
        let expected = brands
            .first()
            .filter(|brand| brands.iter().all(|b| b == *brand));
        if item.get("id") != Some(&json!("NDA021335"))
            || item.pointer("/identity/id") != item.get("id")
            || item.pointer("/identity/identity_level") != Some(&json!("fda_application"))
            || item.get("name") != expected.map(|brand| json!(brand)).as_ref()
            || item.get("products") != app.get("products")
        {
            return Err(format!(
                "application name, identity or products changed: {item}"
            ));
        }
    }
    Ok(())
}
