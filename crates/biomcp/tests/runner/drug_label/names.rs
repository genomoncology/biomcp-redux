//! Label search widens drug names by default, keeps native SPL membership and the discovery
//! step chosen on its first page, and searches the typed name alone with --exact-term.
use super::{Check, Environment, Stub, Value, invoke, json};
mod checks;
pub(super) fn trials() -> Vec<libtest_mimic::Trial> {
    [
        (
            crate::runner::required_contract::LABEL_NAMES.0,
            checks::complete as fn() -> Check,
        ),
        (
            "drug-label/alias-uncertainty-and-controls",
            checks::controls,
        ),
        ("drug-label/alias-frozen-word-tier", checks::tier),
        ("drug-label/alias-provider-refuses", checks::refused),
    ]
    .into_iter()
    .map(|(name, check)| {
        crate::runner::transcript::trial(name, move || check().map_err(Into::into))
    })
    .collect()
}
fn setup(mode: u8, extra: Value) -> Result<(Environment, Stub), String> {
    let mut lookups = 0;
    let stub = Stub::with_headers(
        &["mychem.info", "api.fda.gov"],
        |_| "application/json",
        move |request| {
            let target = request.split_whitespace().nth(1).ok_or("request target")?;
            let url = url::Url::parse(&format!("https://synthetic.test{target}"))
                .map_err(|e| e.to_string())?;
            let pairs: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
            if target.starts_with("/v1/query?") {
                lookups += 1;
                return Ok(lookup(mode, lookups).to_string());
            }
            if !target.starts_with("/drug/label.json?") {
                return Err(format!("unexpected request {request}"));
            }
            if mode == 6 {
                return Ok(json!({"meta":"malformed"}).to_string());
            }
            let selector = pairs.get("search").ok_or("selector")?;
            let limit: u32 = pairs
                .get("limit")
                .ok_or("limit")?
                .parse()
                .map_err(|_| "bad limit")?;
            let offset: u32 = pairs
                .get("skip")
                .map_or(Ok(0), |s| s.parse())
                .map_err(|_| "bad offset")?;
            // In uncertain modes the label resolver's broad candidate lookup must not invent aliases.
            let lookup = limit != 1;
            // Every name hit here is an exact hit, so no word-only rows follow them.
            let word_only = selector.contains(" AND NOT ");
            // Mode 7: no label states either name, so only label text matches.
            // Mode 9: as mode 7, except the brand's indication wording matches.
            let indication = selector.contains("indications_and_usage:\"");
            let text_only = (mode == 7 || mode == 9)
                && (selector.contains("openfda.")
                    || (indication && !(mode == 9 && selector.contains("Brand"))));
            let total = if lookup || word_only || text_only {
                0
            } else if selector.contains("Brand") {
                2
            } else {
                1
            };
            let rows = if offset >= total {
                vec![]
            } else {
                vec![record(offset, selector)]
            };
            Ok(json!({"meta":{"results":{"skip":offset,"limit":limit,"total":total}},"results":rows}).to_string())
        },
        |_, bytes| bytes,
        move |request| status(mode, request),
        |_| String::new(),
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    stub.retain(env.shared_root());
    let ca = env.root().join("aliases-ca.pem");
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let mut config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca,"disable_sources":["chembl"]});
    config
        .as_object_mut()
        .ok_or("config")?
        .extend(extra.as_object().ok_or("extra")?.clone());
    env.config(&config).map_err(|e| e.to_string())?;
    Ok((env, stub))
}
/// The synthetic MyChem reply for one name lookup.
fn lookup(mode: u8, lookups: u32) -> Value {
    let row = |id| {
        json!({"_id":id,"drugcentral":{"synonyms":["Typed drug", match mode {
            4 => "Long alias ".repeat(200), 5 => "Brand \"quoted\" \\name".into(),
            7 if lookups > 1 => "Changed brand".into(), _ => "Brand name".into()
        }]}})
    };
    if mode == 3 {
        json!({"total":"malformed","hits":[]})
    } else {
        let hits = if mode == 2 {
            vec![
                row("KTUFNOKKBVMGRW-UHFFFAOYSA-N"),
                row("YLMAHDNUQAMNNX-UHFFFAOYSA-N"),
            ]
        } else {
            vec![row("KTUFNOKKBVMGRW-UHFFFAOYSA-N")]
        };
        json!({"total":if mode == 1 {2} else {hits.len()},"hits":hits})
    }
}
/// Mode 8: the label source refuses any request naming the widened brand.
fn status(mode: u8, request: &str) -> u16 {
    if mode == 8 && request.contains("/drug/label.json?") && request.contains("Brand") {
        400
    } else {
        200
    }
}
fn record(offset: u32, selector: &str) -> Value {
    let offset =
        u32::from(offset != 0 || selector.contains("a454cd24-0c6d-46e8-b1e4-197388606175"));
    let id = if offset == 0 {
        "095a08ac-cf0e-497e-a682-ddef38d6b29c"
    } else {
        "a454cd24-0c6d-46e8-b1e4-197388606175"
    };
    json!({"id":id,"set_id":id,"version":if offset==0 {"7"} else {"38"},
        "effective_time":if offset==0 {"20210615"} else {"20260424"},
        "openfda":{"brand_name":["Brand name"],"generic_name":[if offset==0 {"Typed drug"} else {"Other ingredient"}],"spl_set_id":[id]},
        "indications_and_usage":["Synthetic indication."]})
}
fn args(expand: bool) -> Vec<String> {
    let mut words = [
        "search",
        "drug",
        "Typed drug",
        "--source",
        "openfda-label",
        "--limit",
        "1",
    ]
    .map(str::to_owned)
    .to_vec();
    // The retired opt-in is still accepted and changes nothing; the opt-out searches as typed.
    words.push(
        if expand {
            "--expand-drug-names"
        } else {
            "--exact-term"
        }
        .into(),
    );
    words
}
fn selector(value: &Value) -> Result<&str, String> {
    value
        .pointer("/sources/0/term")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("no selector {value}"))
}
