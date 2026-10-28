//! Feature drug-approvals: replay keeps application identity and exact shared brand names.
use super::{Check, application, both, invoke, setup};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::json;

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
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
        let product = app
            .pointer("/products/0")
            .ok_or("synthetic product")?
            .clone();
        *app.get_mut("products").ok_or("synthetic products")? = brands
            .iter()
            .enumerate()
            .map(|(n, brand)| {
                let mut product = product.clone();
                *product.get_mut("product_number").ok_or("product number")? =
                    json!(format!("{n:03}"));
                *product.get_mut("brand_name").ok_or("brand name")? = json!(brand);
                Ok(product)
            })
            .collect::<Result<serde_json::Value, String>>()?;
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
            || item.get("products") != app.get("products").filter(|_| !brands.is_empty())
        {
            return Err(format!(
                "application name, identity or products changed: {item}"
            ));
        }
    }
    Ok(())
}
