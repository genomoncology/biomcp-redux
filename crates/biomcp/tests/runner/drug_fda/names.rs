//! Feature drug-approvals: replay keeps application identity and exact shared brand names.
use super::{Check, Environment, Stub, application, both, environment, invoke, setup};
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
        let fallback = brands.len() == 1;
        let (env, stub) = discovery(app.clone(), fallback)?;
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
        if stub.finish()?.len() != if fallback { 2 } else { 1 } {
            return Err("synthetic recording changed the exact-then-word request count".into());
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

fn discovery(app: serde_json::Value, fallback: bool) -> Result<(Environment, Stub), String> {
    let reply = json!({"meta":{"results":{"skip":0,"limit":1,"total":1}},"results":[app]});
    let exact = concat!(
        "products.brand_name.exact%3A%22Synthetic%22%20OR%20",
        "products.active_ingredients.name.exact%3A%22Synthetic%22"
    );
    if !fallback {
        return setup(reply, exact);
    }
    let exact = format!("GET /drug/drugsfda.json?search={exact}&limit=1 HTTP/1.1");
    let word = concat!(
        "GET /drug/drugsfda.json?search=products.brand_name%3A%22Synthetic%22%20OR%20",
        "products.active_ingredients.name%3A%22Synthetic%22&limit=1 HTTP/1.1"
    );
    let stub = Stub::for_source(
        "api.fda.gov",
        "application/json",
        move |request| match request.lines().next() {
            Some(line) if line == exact => Ok(
                json!({"meta":{"results":{"skip":0,"limit":1,"total":0}},"results":[]}).to_string(),
            ),
            Some(line) if line == word => Ok(reply.to_string()),
            _ => Err(format!("unexpected FDA discovery request: {request}")),
        },
    )?;
    let env = environment(&stub)?;
    Ok((env, stub))
}
