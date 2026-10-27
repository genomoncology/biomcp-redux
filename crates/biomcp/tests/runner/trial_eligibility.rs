//! Structural eligibility boundaries through recorded CLI requests.
use super::{launch::Environment, response_cases::copy_capture, trial_get::command};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "trial/get/structural-eligibility-pages",
        || boundaries().map_err(|e| e.to_string().into()),
    )]
}
fn boundaries() -> Check {
    let parent = "Excluded registry criterion\r\n";
    let child = "  Exception Ω\t\"\\ remains attached\r\n";
    let pair = format!("{parent}{child}");
    for (name, original, first, continued) in [
        (
            "move parent",
            format!("{}{pair}{}", "a".repeat(5960) + "\r\n", "tail\n".repeat(40)),
            "a".repeat(5960) + "\r\n",
            false,
        ),
        (
            "keep pair",
            format!("{}{pair}{}", "b".repeat(5800) + "\n", "tail".repeat(100)),
            format!("{}{pair}", "b".repeat(5800) + "\n"),
            false,
        ),
        (
            "oversized line",
            format!("{}\nend", "Ω\"\\".repeat(1600)),
            "Ω\"\\".repeat(1000),
            true,
        ),
        (
            "oversized group",
            format!("{parent}{}", "  nested Ω\n".repeat(1500)),
            String::new(),
            true,
        ),
    ] {
        walk(name, &original, &first, continued)?;
    }
    Ok(())
}
fn walk(name: &str, original: &str, expected_first: &str, continued: bool) -> Check {
    let env = Environment::new()?;
    let replay = env.root().join("recordings");
    copy_capture("trial/get/NCT02296125", &replay, |_, record| {
        if record
            .get("url")
            .and_then(Value::as_str)
            .is_some_and(|u| u.ends_with("NCT02296125"))
        {
            if let Some(value) = record.pointer_mut(
                "/exchanges/0/body/value/protocolSection/eligibilityModule/eligibilityCriteria",
            ) {
                *value = json!(original);
            }
        }
    })?;
    let mut args = ["get", "trial", "NCT02296125", "eligibility"]
        .map(str::to_owned)
        .to_vec();
    let mut joined = String::new();
    for page in 0..20 {
        let globals = if page == 0 {
            env.globals(Some(&replay), false)
        } else {
            env.globals(None, true)
        };
        let out = command(&env, &args, &globals, 0)?;
        let data = out
            .pointer("/result/items/0/sections/eligibility/data")
            .ok_or("missing eligibility")?;
        let text = data
            .get("criteria")
            .and_then(Value::as_str)
            .ok_or("missing text")?;
        if text.is_empty() || data.get("offset") != Some(&json!(joined.chars().count())) {
            return Err(format!("{name}: page did not advance exactly").into());
        }
        if page == 0
            && ((!expected_first.is_empty() && text != expected_first)
                || data.get("criterion_continues") != Some(&json!(continued)))
        {
            return Err(format!("{name}: incorrect boundary or continued flag: {data}").into());
        }
        joined.push_str(text);
        match super::action(&out, "page") {
            Ok(next) => args = next,
            Err(_) => {
                return if joined == original {
                    Ok(())
                } else {
                    Err(format!("{name}: text changed").into())
                };
            }
        }
    }
    Err(format!("{name}: walk did not finish").into())
}
