//! Optional retained-body replay; publisher bodies remain outside Git.
use super::Check;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
pub(super) fn trials() -> Vec<Trial> {
    let mut tests = vec![Trial::test(
        "drug-approvals/native-proof-retains-success-and-failure",
        || retention().map_err(Into::into),
    )];
    let Some(root) = std::env::var_os("BIOMCP_FDA_CAPTURE_ROOT") else {
        return tests;
    };
    tests.extend(
        (0..15)
            .map(|n| {
                let root = PathBuf::from(&root);
                Trial::test(format!("drug-approvals/native-retained-{n}"), move || {
                    native(&root, n).map_err(Into::into)
                })
            })
            .collect::<Vec<_>>(),
    );
    tests
}
fn field<'a>(v: &'a Value, k: &str) -> Result<&'a str, String> {
    v.get(k)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing {k}"))
}
fn entries(path: &Path) -> Result<Vec<Value>, String> {
    serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}
fn input(root: &Path, n: usize) -> Result<(Value, String), String> {
    let (entry, path) = if n < 3 {
        let entry = entries(&root.join("ledger.json"))?
            .get(n)
            .cloned()
            .ok_or("new entry")?;
        let path = root.join(field(&entry, "body_file")?);
        (entry, path)
    } else {
        let entry = entries(&root.join("retained/retained.json"))?
            .into_iter()
            .filter(|e| e.get("source") == Some(&json!("drugsfda")))
            .nth(n - 3)
            .ok_or("retained entry")?;
        let path = root.join("retained").join(field(&entry, "file")?);
        (entry, path)
    };
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let hash = Sha256::digest(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    if hash != field(&entry, "sha256")? {
        return Err("native source body hash differs".into());
    }
    Ok((entry, String::from_utf8(bytes).map_err(|e| e.to_string())?))
}
fn native(root: &Path, n: usize) -> Check {
    let (entry, raw) = input(root, n)?;
    let url = field(&entry, "url")?;
    let query = url.split_once("?search=").ok_or("native selector")?.1;
    let (selector, rest) = query.split_once("&limit=").ok_or("native limit")?;
    let (limit, offset) = rest.split_once("&skip=").unwrap_or((rest, "0"));
    let selector_text = decode(selector)?;
    let (key, text) = selector_text
        .split_once(':')
        .ok_or("native field selector")?;
    let text = text
        .strip_prefix('"')
        .and_then(|t| t.strip_suffix('"'))
        .ok_or("native literal")?;
    let status = entry
        .get("http_status")
        .or_else(|| entry.get("status"))
        .and_then(Value::as_u64)
        .ok_or("native status")? as u16;
    let (env, stub) = delivery(raw.clone(), url, status)?;
    let output = root.join("product-proof").join(format!("case-{n:02}"));
    std::fs::create_dir_all(&output).map_err(|e| e.to_string())?;
    let recording = output.join("recording");
    let mut args = if key == "application_number" {
        vec![
            "get".into(),
            "drug".into(),
            text.into(),
            "approvals".into(),
            "--source".into(),
            "drugsfda".into(),
        ]
    } else {
        vec![
            "search".into(),
            "drug".into(),
            text.into(),
            "--source".into(),
            "drugsfda".into(),
            "--limit".into(),
            limit.into(),
            "--offset".into(),
            offset.into(),
        ]
    };
    let exit = if status == 404 { 1 } else { 0 };
    let mut record = vec!["--record".into(), recording.to_string_lossy().into_owned()];
    record.extend(args.clone());
    super::super::proof::retained_cli(&env, &record, exit, &output.join("local-delivery"))?;
    if stub.finish()?.len() != 1 {
        return Err("native local delivery requested more than once".into());
    }
    let mut replay = vec!["--replay".into(), recording.to_string_lossy().into_owned()];
    replay.append(&mut args);
    let value = both(&env, replay.clone(), exit, &output)?;
    let original: Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    verify(&original, &value, key)?;
    let mut peak = super::measure(&value)?;
    if key == "application_number" {
        let selected = selected_product(&env, &replay, &original, &output)?;
        peak = walk(&env, replay, value, &original, &output)?;
        peak = (peak.0.max(selected.0), peak.1.max(selected.1));
    }
    report(n, peak, &entry)
}
fn report(n: usize, peak: (usize, usize), entry: &Value) -> Check {
    let measured = [
        (2443, 2332),
        (2595, 2484),
        (2409, 2298),
        (2889, 2778),
        (2078, 1967),
        (1820, 1709),
        (1628, 1517),
        (1755, 1644),
        (2279, 2168),
        (2608, 2497),
        (2590, 2479),
        (834, 723),
        (879, 768),
        (393, 282),
        (856, 745),
    ];
    super::budget(peak, *measured.get(n).ok_or("native budget")?)?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "FDA native case {n}: peak_full={} peak_comparable={}",
        peak.0,
        peak.1
    )
    .map_err(|e| e.to_string())?;
    writeln!(std::io::stdout(),
"FDA original_url={} original_date={} original_sha256={} local_delivery=true fresh_provider_calls=0",
field(entry,
"url")?,
field(entry,
"retrieved_at")?,
field(entry,
"sha256")?).map_err(|e|e.to_string())
}
fn delivery(
    raw: String,
    url: &str,
    status: u16,
) -> Result<(super::Environment, super::Stub), String> {
    let selector = url.split_once("?search=").ok_or("URL")?.1;
    // Legacy capture URLs encode spaces as +; the shared GET encoder uses %20.
    // Preserve the original URL in provenance and identify this as a local delivery.
    let selector = selector.replace('+', "%20");
    let expected = format!("GET /drug/drugsfda.json?search={selector} HTTP/1.1");
    let stub = super::Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| {
            if request.lines().next() != Some(expected.as_str()) {
                return Err("native request changed".into());
            }
            Ok(raw.clone())
        },
        move |_| status,
    )?;
    let env = super::environment(&stub)?;
    Ok((env, stub))
}
fn verify(raw: &Value, value: &Value, key: &str) -> Check {
    let Some(rows) = raw.get("results").and_then(Value::as_array) else {
        return if value.get("status") == Some(&json!("absent")) {
            Ok(())
        } else {
            Err("native absence lost".into())
        };
    };
    if key == "application_number" {
        if value.pointer("/result/identity/id") != raw.pointer("/results/0/application_number")
            || value.pointer("/result/products") != raw.pointer("/results/0/products")
        {
            return Err(format!("native application products changed: {value}"));
        }
        if let Some(ids) = raw
            .pointer("/results/0/openfda/spl_set_id")
            .and_then(Value::as_array)
        {
            for id in ids {
                let action = json!({"kind":"disambiguate","args":["get","drug",id,"--source","openfda-label"]});
                if !value
                    .get("next")
                    .and_then(Value::as_array)
                    .is_some_and(|next| next.contains(&action))
                {
                    return Err("native SPL set lost its exact identity-card next action".into());
                }
            }
        }
    } else {
        let ids = rows
            .iter()
            .map(|r| r.get("application_number").cloned().ok_or("id"))
            .collect::<Result<Vec<_>, _>>()?;
        let actual = value
            .pointer("/result/items")
            .and_then(Value::as_array)
            .ok_or("candidates")?
            .iter()
            .map(|r| r.pointer("/identity/id").cloned().ok_or("id"))
            .collect::<Result<Vec<_>, _>>()?;
        if actual != ids || value.pointer("/result/total") != raw.pointer("/meta/results/total") {
            return Err("native candidate order or total changed".into());
        }
        for (native, actual) in rows.iter().zip(
            value
                .pointer("/result/items")
                .and_then(Value::as_array)
                .ok_or("items")?,
        ) {
            let mut products = native.get("products").cloned().ok_or("native products")?;
            for product in products.as_array_mut().ok_or("products")? {
                for key in ["route", "reference_standard", "te_code"] {
                    if product.get(key) == Some(&Value::Null) {
                        product.as_object_mut().ok_or("product")?.remove(key);
                    }
                }
            }
            if actual.get("products") != Some(&products) {
                return Err("native candidate product details changed".into());
            }
        }
    }
    Ok(())
}
fn walk(
    env: &super::Environment,
    base: Vec<String>,
    mut page: Value,
    raw: &Value,
    output: &Path,
) -> Result<(usize, usize), String> {
    let expected = raw
        .pointer("/results/0/submissions")
        .and_then(Value::as_array)
        .ok_or("submissions")?;
    let mut seen = Vec::new();
    let mut peak = super::measure(&page)?;
    loop {
        let records = page
            .pointer("/result/approvals/data/records")
            .and_then(Value::as_array)
            .ok_or("history")?;
        for row in records {
            if row.pointer("/event/product_number").is_some() {
                return Err("submission acquired product applicability".into());
            }
            submission(expected.get(seen.len()).ok_or("extra history row")?, row)?;
            seen.push((
                row.pointer("/event/kind").cloned(),
                row.get("submission_number").cloned(),
            ));
        }
        let Some(args) = page
            .get("next")
            .and_then(Value::as_array)
            .and_then(|a| a.iter().find(|n| n.get("kind") == Some(&json!("page"))))
            .and_then(|n| n.get("args"))
            .and_then(Value::as_array)
        else {
            break;
        };
        let mut next = base.iter().take(2).cloned().collect::<Vec<_>>();
        next.extend(
            args.iter()
                .map(|v| v.as_str().map(str::to_owned).ok_or("arg"))
                .collect::<Result<Vec<_>, _>>()?,
        );
        page = both(env, next, 0, output)?;
        let measured = super::measure(&page)?;
        peak = (peak.0.max(measured.0), peak.1.max(measured.1));
        if seen.len() > expected.len() {
            return Err("history continuation looped".into());
        }
    }
    let expected = expected
        .iter()
        .map(|r| {
            (
                r.get("submission_type").cloned(),
                r.get("submission_number").cloned(),
            )
        })
        .collect::<Vec<_>>();
    if seen != expected {
        return Err("native history walk lost or reordered submissions".into());
    }
    Ok(peak)
}
fn selected_product(
    env: &super::Environment,
    base: &[String],
    raw: &Value,
    output: &Path,
) -> Result<(usize, usize), String> {
    let mut args = base.to_vec();
    args.extend(["--product-number".into(), "001".into()]);
    let value = both(env, args, 0, output)?;
    let products = value
        .pointer("/result/products")
        .and_then(Value::as_array)
        .ok_or("selected products")?;
    if products.len() != 1
        || products.first().and_then(|p| p.get("product_number")) != Some(&json!("001"))
        || value
            .pointer("/result/approvals/data/total")
            .and_then(Value::as_u64)
            != raw
                .pointer("/results/0/submissions")
                .and_then(Value::as_array)
                .map(|r| r.len() as u64)
    {
        return Err(
            "explicit native product changed submission ownership or selected the first product"
                .into(),
        );
    }
    super::measure(&value)
}
fn submission(native: &Value, row: &Value) -> Check {
    for key in [
        "submission_number",
        "review_priority",
        "submission_class_code",
        "submission_class_code_description",
        "submission_property_type",
    ] {
        if native.get(key) != row.get(key) {
            return Err(format!("native submission field changed: {key}"));
        }
    }
    if native.get("submission_status") != row.pointer("/event/status")
        || row.pointer("/event/dates/0/role") != Some(&json!("submission_status_date"))
        || field(native, "submission_status_date")?
            != row
                .pointer("/event/dates/0/date")
                .and_then(Value::as_str)
                .ok_or("status date")?
                .replace('-', "")
    {
        return Err("native submission status or date role changed".into());
    }
    match (native.get("application_docs"), row.get("application_docs")) {
        (None, None) => {}
        (Some(Value::Array(native)), Some(Value::Array(actual)))
            if native.len() == actual.len() =>
        {
            for (native, actual) in native.iter().zip(actual) {
                for key in ["id", "url", "type"] {
                    if native.get(key) != actual.get(key) {
                        return Err("native document identity changed".into());
                    }
                }
                if field(native, "date")? != field(actual, "date")?.replace('-', "") {
                    return Err("document date became event date".into());
                }
            }
        }
        _ => return Err("native documents lost absence or multiplicity".into()),
    }
    Ok(())
}
fn decode(value: &str) -> Result<String, String> {
    let mut bytes = value.bytes();
    let mut out = Vec::new();
    while let Some(b) = bytes.next() {
        if b == b'%' {
            let a = bytes
                .next()
                .and_then(|b| (b as char).to_digit(16))
                .ok_or("escape")?;
            let b = bytes
                .next()
                .and_then(|b| (b as char).to_digit(16))
                .ok_or("escape")?;
            out.push((a * 16 + b) as u8);
        } else {
            out.push(if b == b'+' { b' ' } else { b });
        }
    }
    String::from_utf8(out).map_err(|e| e.to_string())
}

fn retention() -> Check {
    for fail in [false, true] {
        let root = tempfile::tempdir().map_err(|e| e.to_string())?;
        let mut application = super::application();
        let product = application
            .pointer_mut("/products/0")
            .and_then(Value::as_object_mut)
            .ok_or("missing synthetic product")?;
        product.insert("product_number".into(), json!("001"));
        if fail {
            product.insert("reference_standard".into(), Value::Null);
        }
        let raw =
            json!({"meta":{"results":{"skip":0,"limit":1,"total":1}},"results":[application]})
                .to_string();
        let hash = Sha256::digest(raw.as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        std::fs::write(root.path().join("synthetic.json"), raw).map_err(|e| e.to_string())?;
        std::fs::write(
            root.path().join("ledger.json"),
            json!([{"body_file":"synthetic.json",
"sha256":hash,
"url":"https://api.fda.gov/drug/drugsfda.json?search=application_number%3A%22NDA021335%22&limit=1",
"http_status":200,
"retrieved_at":"2001-05-10T00:00:00Z"}])
            .to_string(),
        )
        .map_err(|e| e.to_string())?;
        let result = native(root.path(), 0);
        if result.is_err() != fail {
            return Err(format!("unexpected synthetic FDA result: {result:?}"));
        }
        if fail
            && !result
                .as_ref()
                .err()
                .is_some_and(|e| e.contains("native application products changed"))
        {
            return Err(format!(
                "failed before the intended FDA comparison: {result:?}"
            ));
        }
        let output = root.path().join("product-proof/case-00");
        if !output.join("recording/drugsfda").is_dir()
            || !output.join("replay-0/cli.stdout").is_file()
            || !output.join("replay-0/mcp.stdout").is_file()
        {
            return Err("FDA proof artifacts disappeared after cleanup".into());
        }
    }
    Ok(())
}

fn both(
    env: &super::Environment,
    args: Vec<String>,
    exit: i32,
    output: &Path,
) -> Result<Value, String> {
    let index = std::fs::read_dir(output)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .filter(|v| v.file_name().to_string_lossy().starts_with("replay-"))
        .count();
    super::super::proof::retained_both(env, &args, exit, &output.join(format!("replay-{index}")))
}
