//! Feature vaers-summary: exact native influenza types retain separate tables.
use super::{Check, environment, words};
use crate::runner::{configuration::invoke, network_stub::Stub};
use serde_json::{Value, json};

pub(super) fn run() -> Check {
    let stub = Stub::for_source("wonder.cdc.gov", "application/xml", |request| {
        let (code, label) = biomcp_types::VAERS_INFLUENZA
            .iter()
            .find(|(code, _)| {
                let encoded = code.replace('(', "%28").replace(')', "%29");
                request.contains(&format!("%3Cvalue%3E{encoded}%3C"))
            })
            .ok_or("request did not carry an exact qualified influenza code")?;
        Ok(super::fixture::table(code, "D8.V13-level2").replace("Synthetic vaccine", label))
    })?;
    let env = environment(&stub)?;
    for (code, label) in biomcp_types::VAERS_INFLUENZA {
        let args = words(code);
        let value = invoke(&env, args.clone(), 0)?;
        super::super::schema::Registry::embedded()?.validate(&value, &args, true)?;
        super::budget::check(&value)?;
        if value.pointer("/result/vaccine/code") != Some(&json!(code))
            || value.pointer("/result/vaccine/label") != Some(&json!(label))
            || value.pointer("/result/report_total") != Some(&json!("5"))
            || value.pointer("/result/footer/reports/dt") != Some(&json!("9"))
        {
            return Err(format!(
                "influenza identity or table facts changed: {value}"
            ));
        }
        if super::super::mcp::call_with_globals(&env, &args, false, &[])? != value {
            return Err("influenza CLI and MCP differ".into());
        }
        for step in value
            .get("next")
            .and_then(Value::as_array)
            .ok_or("table actions")?
        {
            if !step
                .get("args")
                .and_then(Value::as_array)
                .is_some_and(|args| args.contains(&json!(code)))
            {
                return Err("influenza continuation lost its qualified selector".into());
            }
        }
    }
    if stub.finish()?.len() != 14 {
        return Err("native influenza code was fetched more than once".into());
    }
    Ok(())
}
