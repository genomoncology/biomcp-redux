//! Features drug-mychem, variant-search and variant-identity: selectable pages stay reachable.
use super::{configuration::invoke, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    ["drug-search", "variant-search", "variant-get"]
        .into_iter()
        .flat_map(|mode| {
            [false, true].into_iter().map(move |terminal| {
                transcript_trial(
                    format!("navigation/{mode}/terminal-{terminal}"),
                    move || check(mode, terminal).map_err(Into::into),
                )
            })
        })
        .collect()
}

fn check(mode: &str, terminal: bool) -> Result<(), String> {
    let drug = mode == "drug-search";
    let total = if terminal { 5 } else { 10 };
    let rows: Vec<_> = (1..=if mode == "variant-get" { total } else { 5 })
        .map(|n| {
            if drug {
                json!({"_id":format!("CHEMBL{n}")})
            } else {
                json!({"_id":format!("chr7:g.14045313{n}A>G"),"chrom":"7","hg19":{"start":140453130+n,"end":140453130+n},"dbsnp":{"rsid":"rs113488022"}})
            }
        })
        .collect();
    let body = json!({"total":total,"hits":rows}).to_string();
    let stub = Stub::for_source_status(
        if drug {
            "mychem.info"
        } else {
            "myvariant.info"
        },
        "application/json",
        move |_| Ok(body.clone()),
        |_| 200,
    )?;
    let env = super::variant::source_environment(&stub)?;
    let args = arguments(mode);
    let value = invoke(&env, args.clone(), 0)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    if super::mcp::call_with_globals(&env, &args, false, &[])? != value {
        return Err("navigation differs over MCP".into());
    }
    verify(&value, mode, terminal)?;
    let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(&value, "")?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "navigation {mode} terminal={terminal}: full={full}, comparable={comparable}"
    )
    .map_err(|e| e.to_string())?;
    let ceiling = match (mode, terminal) {
        ("drug-search", false) => 1647,
        ("drug-search", true) => 1550,
        ("variant-search", false) => 2297,
        ("variant-search", true) => 2136,
        ("variant-get", false) => 4050,
        _ => 3869,
    };
    if comparable > ceiling {
        return Err(format!(
            "navigation exceeds measured budget: {comparable}/{ceiling}"
        ));
    }
    if stub.finish()?.len() != 1 {
        return Err("navigation or warm MCP unexpectedly requested another source".into());
    }
    Ok(())
}

fn arguments(mode: &str) -> Vec<String> {
    match mode {
        "drug-search" => vec!["search", "drug", "synthetic", "--limit", "5"],
        "variant-search" => vec![
            "search",
            "variant",
            "--query",
            "rs113488022",
            "--source",
            "myvariant",
            "--build",
            "GRCh37",
            "--limit",
            "5",
        ],
        _ => vec![
            "get",
            "variant",
            "rs113488022",
            "oncokb",
            "hotspots",
            "--tumor-type",
            "Synthetic tumor",
            "--build",
            "GRCh37",
            "--limit",
            "5",
        ],
    }
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn verify(value: &Value, mode: &str, terminal: bool) -> Result<(), String> {
    if value.get("page")
        != Some(&json!({"offset":0,"returned":5,"total":if terminal {5} else {10}}))
    {
        return Err(format!(
            "candidate page metadata disagrees: {:?}",
            value.get("page")
        ));
    }
    let next = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("next missing")?;
    for n in 1..=5 {
        let row = next.get(n - 1).ok_or("candidate action missing")?;
        let id = if mode == "drug-search" {
            format!("CHEMBL{n}")
        } else {
            format!("chr7:g.14045313{n}A>G")
        };
        let args = row
            .get("args")
            .and_then(Value::as_array)
            .ok_or("candidate args")?;
        let expected = if mode == "drug-search" {
            json!(["get", "drug", id, "--source", "mychem"])
        } else {
            json!(["get", "variant", id, "--build", "GRCh37"])
        };
        if row.get("kind") != Some(&json!("disambiguate"))
            || args != expected.as_array().ok_or("candidate expectation")?
        {
            return Err(format!("candidate {n} get lost or reordered: {next:?}"));
        }
    }
    if terminal {
        if next.iter().any(|s| s.get("kind") == Some(&json!("page"))) {
            return Err("terminal page advertises more candidates".into());
        }
    } else {
        let page = next
            .get(5)
            .ok_or("page step dropped after five candidate gets")?;
        if page.get("kind") != Some(&json!("page")) {
            return Err("page must follow candidates".into());
        }
        let args: Vec<String> =
            serde_json::from_value(page.get("args").cloned().ok_or("page args")?)
                .map_err(|e| e.to_string())?;
        let mut expected = arguments(mode);
        expected.extend(["--offset".into(), "5".into()]);
        // Spelling may order flags differently; every supplied selector must survive.
        for pair in expected
            .windows(2)
            .filter(|w| w.first().is_some_and(|word| word.starts_with("--")))
        {
            if !args.windows(2).any(|w| w == pair) {
                return Err(format!("lost page selector {pair:?}: {args:?}"));
            }
        }
    }
    if mode == "variant-get"
        && (next.last().and_then(|step| step.get("kind")) != Some(&json!("key"))
            || next.len() != 5 + usize::from(!terminal) + 1)
    {
        return Err("key guidance was dropped or displaced candidate navigation".into());
    }
    if next.len() > 5 + usize::from(!terminal) + 3 {
        return Err("more than three extra actions".into());
    }
    Ok(())
}
