//! Feature drug-purple-book: a failed manual update leaves the active FDA release readable.
use crate::runner::{launch, launch::Environment, transcript_trial, trial_get::command};
use libtest_mimic::Trial;
use serde_json::Value;
use std::fs;

pub(super) fn trial() -> Trial {
    transcript_trial("drug-purple-book/manual-release-import", || {
        import().map_err(|error| error.to_string().into())
    })
}

fn import() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::new()?;
    let offline = ["--offline".into()];
    let mut sync = ["sync", "purple-book"].map(str::to_owned).to_vec();
    let manual = command(&env, &sync, &offline, 4)?;
    let guidance = notice(&manual);
    if !guidance.contains("FDA does not allow automatic download")
        || !guidance.contains("https://purplebooksearch.fda.gov/downloads")
        || !guidance.contains("October 2026")
    {
        return Err(format!(
            "plain sync did not explain the manual update or active snapshot: {manual}"
        )
        .into());
    }
    let file = env.root().join("monthly.csv");
    let release = fs::read_to_string(launch::fixtures().join("fixtures/purple-book/newer.csv"))?;
    fs::write(&file, &release)?;
    sync.extend(["--file".into(), "monthly.csv".into()]);
    let imported = command(&env, &sync, &offline, 0)?;
    if imported
        .pointer("/result/data_date")
        .and_then(Value::as_str)
        != Some("2026-11")
    {
        return Err(format!("import did not report the FDA release month: {imported}").into());
    }
    // A newer report containing only the change section is not a complete replacement.
    let incomplete = release
        .split_once("\n\n")
        .ok_or("fixture's full roster is missing")?
        .0
        .replace("November 2026", "December 2026");
    fs::write(&file, incomplete)?;
    let failed = command(&env, &sync, &offline, 4)?;
    if !notice(&failed).contains("release is unavailable")
        || !notice(&failed).contains("November 2026")
    {
        return Err(format!("failed import did not name the retained release: {failed}").into());
    }
    let search = ["search", "drug", "etanercept", "--source", "purple-book"].map(str::to_owned);
    let retained = command(&env, &search, &offline, 0)?;
    if retained
        .pointer("/result/release_month")
        .and_then(Value::as_str)
        != Some("November 2026")
        || retained.pointer("/page/total").and_then(Value::as_u64) != Some(2)
        || retained
            .pointer("/result/items/1/applicant")
            .and_then(Value::as_str)
            != Some("Samsung Bioepis Co., Ltd.")
    {
        return Err(format!("failed import changed the active full roster: {retained}").into());
    }
    let search = ["search", "drug", "761066", "--source", "purple-book"].map(str::to_owned);
    let product = command(&env, &search, &offline, 0)?;
    if product.pointer("/result/items/0/bla").and_then(Value::as_str) != Some("BLA761066")
        || product.pointer("/page/total").and_then(Value::as_u64) != Some(1)
    {
        return Err(format!("the import did not index the biosimilar BLA: {product}").into());
    }
    Ok(())
}

fn notice(value: &Value) -> &str {
    value
        .pointer("/sources/0/notices/0/message")
        .and_then(Value::as_str)
        .unwrap_or_default()
}
