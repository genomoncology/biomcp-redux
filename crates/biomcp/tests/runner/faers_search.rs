//! Feature adverse-event-search: synthetic literal selectors and native report navigation.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
mod examples;
mod policy;
mod status;
pub(super) fn record_examples(directory: &std::path::Path) -> Check {
    examples::record(directory)
}
pub(super) fn trials() -> Vec<Trial> {
    let mut cases = vec![transcript_trial(
        "adverse-event-search/native-order-and-next",
        || navigation().map_err(Into::into),
    )];
    for (name, flags, suffix) in [
        (
            "serious-lifethreatening",
            vec!["--serious", "lifethreatening"],
            " AND seriousnesslifethreatening:1",
        ),
        (
            "serious-disabling",
            vec!["--serious", "disabling"],
            " AND seriousnessdisabling:1",
        ),
        (
            "serious-congenital-anomaly",
            vec!["--serious", "congenital-anomaly"],
            " AND seriousnesscongenitalanomali:1",
        ),
        (
            "serious-other",
            vec!["--serious", "other"],
            " AND seriousnessother:1",
        ),
        (
            "serious-any",
            vec!["--source", "faers", "--serious", "any"],
            " AND serious:1",
        ),
        (
            "serious-death",
            vec!["--serious", "death"],
            " AND seriousnessdeath:1",
        ),
        (
            "serious-hospitalization",
            vec!["--serious", "hospitalization"],
            " AND seriousnesshospitalization:1",
        ),
        (
            "from",
            vec!["--received-from", "2024-01-01"],
            " AND receivedate:[20240101 TO *]",
        ),
        (
            "to",
            vec!["--received-to", "2024-12-31"],
            " AND receivedate:[* TO 20241231]",
        ),
        (
            "combined",
            vec![
                "--reaction",
                "Synthetic reaction",
                "--serious",
                "death",
                "--received-from",
                "2024-01-01",
                "--received-to",
                "2024-12-31",
            ],
            " AND patient.reaction.reactionmeddrapt:\"Synthetic reaction\" AND seriousnessdeath:1 AND receivedate:[20240101 TO 20241231]",
        ),
    ] {
        cases.push(transcript_trial(
            format!("adverse-event-search/synthetic-{name}"),
            move || selectors(&flags, suffix).map_err(Into::into),
        ));
    }
    cases.push(transcript_trial(
        "adverse-event-search/200-absence-is-rejected-before-cache",
        || status::run().map_err(Into::into),
    ));
    cases.extend(policy::trials());
    cases
}
fn expression() -> &'static str {
    "(patient.drug.openfda.generic_name:\"Synthetic \\\"A\\\\B\" OR \
        patient.drug.openfda.brand_name:\"Synthetic \\\"A\\\\B\" OR \
        patient.drug.medicinalproduct:\"Synthetic \\\"A\\\\B\")"
}
fn args() -> Vec<String> {
    ["search", "adverse-event", "Synthetic \"A\\B"]
        .map(str::to_owned)
        .to_vec()
}
fn encoded(value: &str) -> String {
    value
        .bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                char::from(b).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn body(limit: u32, offset: u32) -> Value {
    let rows: Vec<_> = (0..limit)
        .map(|n| {
            json!(
                {"safetyreportid":(100+offset+n).to_string(),"safetyreportversion":"1","receivedate":"20240102","serious":"1",
                "patient":{"drug":[{"medicinalproduct":"Synthetic A"},{"medicinalproduct":"Synthetic B"}],
                "reaction":[{"reactionmeddrapt":"Synthetic term"}]}}
            )
        })
        .collect();
    json!({"meta":{"last_updated":"2024-09-01","results":{"skip":offset,"limit":limit,"total":20}},"results":rows})
}
fn setup(query: &str, limit: u32) -> Result<(Environment, Stub), String> {
    let expected = format!(
        "GET /drug/event.json?search={}&limit={limit}&skip=0&sort=receivedate%3Adesc HTTP/1.1",
        encoded(query)
    );
    let stub = Stub::for_sources(
        &["api.fda.gov"],
        |_| "application/json",
        move |request| {
            let offset = if request.lines().next() == Some(expected.as_str()) {
                0
            } else if request.lines().next() == Some(expected.replace("skip=0", "skip=5").as_str())
            {
                5
            } else {
                return Err(format!("wrong literal selector: {request}"));
            };
            Ok(body(limit, offset).to_string())
        },
    )?;
    let env = environment(&stub)?;
    Ok((env, stub))
}
fn environment(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn both(env: &Environment, args: Vec<String>, exit: i32) -> Result<Value, String> {
    let cli = invoke(env, args.clone(), exit)?;
    super::schema::Registry::embedded()?.validate(&cli, &args, true)?;
    let start = args.iter().position(|s| s == "search").unwrap_or(0);
    let (globals, command) = args.split_at(start);
    let mcp = super::mcp::call_with_globals(env, command, exit >= 2, globals)?;
    if cli != mcp {
        return Err("CLI/MCP report search differs".into());
    }
    measure(&cli, &args)?;
    Ok(cli)
}
fn measure(value: &Value, args: &[String]) -> Check {
    let full = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let mut comparable = value.clone();
    if let Some(rows) = comparable.get_mut("sources").and_then(Value::as_array_mut) {
        for row in rows {
            let row = row.as_object_mut().ok_or("source outcome")?;
            let source: biomcp_types::Source =
                serde_json::from_value(row.get("source").ok_or("source identity")?.clone())
                    .map_err(|e| e.to_string())?;
            let terms =
                biomcp_adapters::registry::terms(source).ok_or("source terms declaration")?;
            if row.get("use") != Some(&json!(terms.usage))
                || row.get("terms") != Some(&json!(terms.name))
                || row.get("terms_url") != Some(&json!(terms.url))
            {
                return Err("source metadata differs before budget deduction".into());
            }
            for field in ["use", "terms", "terms_url"] {
                row.remove(field).ok_or("source terms")?;
            }
        }
    }
    let bytes = serde_json::to_vec(&comparable)
        .map_err(|e| e.to_string())?
        .len();
    // j12915 observed maxima; each full/comparable ceiling adds 15 percent.
    let caps = if value.get("error").is_some() {
        // j19028: typed argument names, 185 compact bytes plus 15 percent.
        (213, 213)
    } else if value.pointer("/sources/0/reason") == Some(&json!("bad-reply")) {
        // j12936: 316 full / 201 comparable bytes, plus 15 percent.
        (364, 232)
    } else if value.get("status") == Some(&json!("failed")) {
        (396, 264)
    } else if args.iter().any(|v| v == "restricted") {
        (2855, 2723)
    } else if args.iter().any(|v| v == "meddra") {
        (1889, 1757)
    } else if value.pointer("/query/limit") == Some(&json!(7)) {
        (2043, 1911)
    } else if value.pointer("/query/reaction").is_some() {
        (2392, 2260)
    } else if value.pointer("/query/received_from").is_some() {
        (1796, 1663)
    } else {
        (1791, 1659)
    };
    if full > caps.0 || bytes > caps.1 {
        return Err(format!(
            "FAERS synthetic budget exceeded: {full}/{bytes} > {caps:?}"
        ));
    }
    Ok(())
}
fn selectors(flags: &[&str], suffix: &str) -> Check {
    let expected = format!("{}{suffix}", expression());
    let (env, stub) = setup(&expected, 5)?;
    let mut words = args();
    words.extend(flags.iter().map(|v| (*v).to_owned()));
    let value = both(&env, words, 0)?;
    if value.pointer("/result/source_query") != Some(&json!(expected))
        || value.pointer("/page/total") != Some(&json!(20))
        || value.pointer("/result/items/0/serious") != Some(&json!("1"))
    {
        return Err(format!("synthetic selector changed: {value}"));
    }
    let page = value
        .get("next")
        .and_then(Value::as_array)
        .and_then(|v| v.iter().find(|s| s.get("kind") == Some(&json!("page"))))
        .and_then(|s| s.get("args"))
        .ok_or("filtered page action")?;
    let words: Vec<String> = serde_json::from_value(page.clone()).map_err(|e| e.to_string())?;
    let second = both(&env, words, 0)?;
    if second.pointer("/result/source_query") != Some(&json!(expected))
        || second.pointer("/query/offset") != Some(&json!(5))
    {
        return Err("next page lost native selector".into());
    }
    for (key, first) in value
        .get("query")
        .and_then(Value::as_object)
        .ok_or("query")?
    {
        if key != "offset" && second.get("query").and_then(|v| v.get(key)) != Some(first) {
            return Err(format!("page lost {key}"));
        }
    }
    if stub.finish()?.len() != 2 {
        return Err("warm filtered pages performed HTTP".into());
    }
    Ok(())
}
fn navigation() -> Check {
    let (env, stub) = setup(expression(), 7)?;
    let mut words = args();
    words.extend(["--limit".into(), "7".into()]);
    let value = both(&env, words.clone(), 0)?;
    words.extend(["--source".into(), "faers".into()]);
    if invoke(&env, words, 0)? != value {
        return Err("default FAERS differs from explicit source".into());
    }
    let next = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("missing next")?;
    if next.len() != 8
        || value.pointer("/result/source_updated") != Some(&json!("2024-09-01"))
        || value
            .pointer("/result/fetched")
            .and_then(Value::as_u64)
            .is_none()
        || value.pointer("/result/items/0/first_received") != Some(&json!("20240102"))
    {
        return Err(format!(
            "report headers, dates or candidate gets changed: {value}"
        ));
    }
    for (n, step) in next.iter().take(7).enumerate() {
        let args = step
            .get("args")
            .and_then(Value::as_array)
            .ok_or("get args")?;
        if args.get(2) != Some(&json!((100 + n).to_string()))
            || step.get("kind") != Some(&json!("disambiguate"))
        {
            return Err("candidate get lost native order/identity".into());
        }
    }
    let page = next
        .last()
        .and_then(|s| s.get("args"))
        .and_then(Value::as_array)
        .ok_or("page args")?;
    if !page
        .windows(2)
        .any(|v| v == [json!("--offset"), json!("7")])
        || !page.contains(&json!("Synthetic \"A\\B"))
    {
        return Err("page action lost selector".into());
    }
    let md = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &[
                "--format".into(),
                "md".into(),
                "search".into(),
                "adverse-event".into(),
                "Synthetic \"A\\B".into(),
                "--limit".into(),
                "7".into(),
            ],
        )
        .output()
        .map_err(|e| e.to_string())?;
    let markdown_bytes = md.stdout.len();
    if markdown_bytes > 1738 {
        return Err(format!(
            "report Markdown exceeds measured +15% ceiling: {markdown_bytes}"
        ));
    }
    let text = String::from_utf8(md.stdout).map_err(|e| e.to_string())?;
    if !md.status.success()
        || !text.contains("**source\\_query:**")
        || !text.contains("2024-09-01")
        || !text.contains("biomcp get adverse-event 100")
    {
        return Err(format!("Markdown lost report facts or actions: {text}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("warm report search reached HTTP".into());
    }
    Ok(())
}
