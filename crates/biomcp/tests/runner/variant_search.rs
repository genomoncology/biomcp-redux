//! Declared MyVariant query construction and native paging cross the real TLS boundary.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "variant-search/component-terms-preserve-comparable-payload",
            || terms_budget().map_err(Into::into),
        ),
        Trial::test(
            "variant-search/source-coordinate-build-and-unknown-metadata",
            || metadata().map_err(Into::into),
        ),
        Trial::test(
            "variant-search/native-window-discloses-unretrieved-total",
            || window().map_err(Into::into),
        ),
        Trial::test(
            "variant-search/declared-symbol-escapes-query-syntax",
            || literal_gene().map_err(Into::into),
        ),
        Trial::test(
            "variant-search/raw-region-build-and-native-duplicates",
            || raw_and_region().map_err(Into::into),
        ),
        Trial::test(
            "variant-search/valid-zero-and-failed-page-cache-admission",
            || zero_and_failed().map_err(Into::into),
        ),
        Trial::test(
            "variant-search/declared-protein-native-walk-and-warm-page",
            || walk().map_err(Into::into),
        ),
    ]
}
fn walk() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", body)?;
    let env = environment(&stub)?;
    let mut args: Vec<String> = [
        "search",
        "variant",
        "--gene",
        "EGFR",
        "--protein",
        "L858R",
        "--limit",
        "1",
    ]
    .into_iter()
    .map(Into::into)
    .collect();
    let mut ids = Vec::new();
    for offset in 0..2 {
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if !output.status.success()
            || value.pointer("/page/offset") != Some(&json!(offset))
            || value.pointer("/page/total") != Some(&json!(2))
            || value.pointer("/query/build").is_some()
            || value.pointer("/result/build").is_some()
            || value.pointer("/result/items/0/identity/representations/0/build")
                != Some(&json!("GRCh37"))
            || value.pointer("/result/source_query")
                != Some(&json!("dbnsfp.genename:EGFR AND dbnsfp.hgvsp:\"p.L858R\""))
        {
            return Err(format!(
                "declared source query or native page lost: {value}"
            ));
        }
        ids.push(
            value
                .pointer("/result/items/0/record/id")
                .cloned()
                .ok_or("candidate missing")?,
        );
        if offset == 0 {
            let page = value
                .get("next")
                .and_then(Value::as_array)
                .and_then(|n| n.iter().find(|v| v.get("kind") == Some(&json!("page"))))
                .ok_or("page action absent")?;
            args = serde_json::from_value(page.get("args").cloned().ok_or("page args absent")?)
                .map_err(|e| e.to_string())?;
        }
    }
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success()
        || ids != [json!("chr7:g.55259515T>G"), json!("chr7:g.55259516T>G")]
        || stub.finish()?.len() != 2
    {
        return Err("walk reordered, lost candidates, or warm page requested again".into());
    }
    Ok(())
}
fn body(request: &str) -> Result<String, String> {
    if !request.starts_with(
        "GET /v1/query?q=dbnsfp.genename%3AEGFR%20AND%20dbnsfp.hgvsp%3A%22p.L858R%22&size=1&from=",
    ) || request.contains("assembly=")
    {
        return Err(format!("incorrect query request: {request}"));
    }
    let id = if request.contains("&from=1&") {
        "chr7:g.55259516T>G"
    } else {
        "chr7:g.55259515T>G"
    };
    Ok(json!(
        {"total":2,"hits":[{"_id":id,"chrom":"7","hg19":{"start":if id.ends_with("15T>G") {55259515} else {55259516},
        "end":if id.ends_with("15T>G") {55259515} else {55259516}},"dbsnp":{"gene":{"symbol":"EGFR"}}}]}
    )
    .to_string())
}

fn environment(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("synthetic-ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn command(env: &Environment, args: &[&str]) -> Result<Value, String> {
    let args = args.iter().map(|s| (*s).into()).collect::<Vec<_>>();
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn raw_and_region() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", selectors_body)?;
    let env = environment(&stub)?;
    let query = "  dbsnp.rsid:rs113488022 OR _id:chr7*  ";
    let raw = command(
        &env,
        &[
            "search",
            "variant",
            "--query",
            query,
            "--source",
            "myvariant",
            "--build",
            "GRCh38",
        ],
    )?;
    let region = command(
        &env,
        &[
            "search",
            "variant",
            "--region",
            "chr7:140753336-140753336",
            "--build",
            "GRCh38",
        ],
    )?;
    if raw.pointer("/result/source_query") != Some(&json!(query))
        || raw.pointer("/query/query") != Some(&json!(query))
        || raw.pointer("/page/returned") != Some(&json!(2))
        || raw.pointer("/result/items/0") != raw.pointer("/result/items/1")
        || region.pointer("/result/source_query") != Some(&json!("chr7:140753336-140753336"))
        || region.pointer("/result/items/0/identity/representations/0/build")
            != Some(&json!("GRCh38"))
    {
        return Err(format!(
            "raw syntax, duplicate source candidates or regional build lost: {raw} {region}"
        ));
    }
    restricted_regions(&env)?;
    if stub.finish()?.len() != 3 {
        return Err("selector made unexpected source calls".into());
    }
    Ok(())
}
fn selectors_body(request: &str) -> Result<String, String> {
    let raw = "GET /v1/query?q=%20%20dbsnp.rsid%3Ars113488022%20OR%20_id%3Achr7%2A%20%20&";
    let region = "GET /v1/query?q=chr7%3A140753336-140753336&";
    if !(request.starts_with(raw) || request.starts_with(region))
        || !request.contains("&assembly=hg38 HTTP/1.1")
    {
        return Err(format!("raw/region query changed: {request}"));
    }
    Ok(json!(
        {"total":2,"hits":[{"_id":"chr7:g.140753336A>T","chrom":"7","hg38":{"start":140753336,"end":140753336}},
        {"_id":"chr7:g.140753336A>T","chrom":"7","hg38":{"start":140753336,"end":140753336}}]}
    )
    .to_string())
}
fn zero_and_failed() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", outcomes_body)?;
    let env = environment(&stub)?;
    let zero = command(
        &env,
        &[
            "search",
            "variant",
            "--query",
            "synthetic-zero",
            "--source",
            "myvariant",
        ],
    )?;
    if zero.pointer("/status") != Some(&json!("absent"))
        || zero.pointer("/page/total") != Some(&json!(0))
        || zero.pointer("/sources/0/state") != Some(&json!("ok"))
    {
        return Err(format!("valid native zero was not preserved: {zero}"));
    }
    let error = command(
        &env,
        &[
            "search",
            "variant",
            "--query",
            "synthetic-error",
            "--source",
            "myvariant",
        ],
    )?;
    if error.pointer("/sources/0/reason") != Some(&json!("bad_reply")) {
        return Err(format!("provider error became valid zero: {error}"));
    }
    for _ in 0..2 {
        let bad = command(
            &env,
            &[
                "search",
                "variant",
                "--query",
                "synthetic-bad",
                "--source",
                "myvariant",
            ],
        )?;
        if bad.pointer("/status") != Some(&json!("failed"))
            || bad.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
            || bad.get("page").is_some()
        {
            return Err(format!("malformed source page became a valid zero: {bad}"));
        }
    }
    let offline = command(
        &env,
        &[
            "search",
            "variant",
            "--query",
            "synthetic-bad",
            "--source",
            "myvariant",
            "--offline",
        ],
    )?;
    if offline.pointer("/sources/0/reason") != Some(&json!("offline")) || stub.finish()?.len() != 4
    {
        return Err("invalid source page entered or came from cache".into());
    }
    Ok(())
}
fn outcomes_body(request: &str) -> Result<String, String> {
    if request.contains("q=synthetic-error&") {
        return Ok(json!({"error":"synthetic provider failure","total":0,"hits":[]}).to_string());
    }
    if request.contains("q=synthetic-zero&") {
        Ok(json!({"total":0,"hits":[]}).to_string())
    } else {
        Ok(json!({"total":1,"hits":[]}).to_string())
    }
}

fn literal_gene() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", literal_body)?;
    let env = environment(&stub)?;
    let value = command(
        &env,
        &[
            "search",
            "variant",
            "--gene",
            "HER-2/neu",
            "--protein",
            "W24*",
        ],
    )?;
    if value.pointer("/result/source_query")
        != Some(&json!(
            r#"dbnsfp.genename:HER\-2\/neu AND dbnsfp.hgvsp:"p.W24\*""#
        ))
        || value.pointer("/query/gene") != Some(&json!("HER-2/neu"))
        || value.pointer("/status") != Some(&json!("absent"))
        || stub.finish()?.len() != 1
    {
        return Err(format!(
            "literal field query changed symbol meaning: {value}"
        ));
    }
    Ok(())
}
fn literal_body(request: &str) -> Result<String, String> {
    if !request.starts_with("GET /v1/query?q=dbnsfp.genename%3AHER%5C-2%5C%2Fneu%20AND%20dbnsfp.hgvsp%3A%22p.W24%5C%2A%22&") {
        return Err(format!("source query metacharacter was not escaped: {request}"));
    }

    Ok(json!({"total":0,"hits":[]}).to_string())
}

fn window() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", window_body)?;
    let env = environment(&stub)?;
    let value = command(
        &env,
        &[
            "search",
            "variant",
            "--query",
            "synthetic-window",
            "--source",
            "myvariant",
            "--limit",
            "2",
            "--offset",
            "9999",
        ],
    )?;
    if value.pointer("/status") != Some(&json!("partial"))
        || value.pointer("/page/total") != Some(&json!(10001))
        || value.pointer("/page/returned") != Some(&json!(1))
        || value.pointer("/page/truncated") != Some(&json!(true))
        || value
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|steps| {
                steps
                    .iter()
                    .any(|step| step.get("kind") == Some(&json!("page")))
            })
        || stub.finish()?.len() != 1
    {
        return Err(format!(
            "bounded native window hid its remaining total: {value}"
        ));
    }
    Ok(())
}
fn window_body(request: &str) -> Result<String, String> {
    if !request.contains("&size=1&from=9999&") {
        return Err(format!(
            "source request exceeded declared window: {request}"
        ));
    }
    Ok(json!({"total":10001,"hits":[{"_id":"chr7:g.55259515T>G"}]}).to_string())
}

fn metadata() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", metadata_body)?;
    let env = environment(&stub)?;
    for index in 0..7 {
        let value = command(
            &env,
            &[
                "search",
                "variant",
                "--query",
                &format!("metadata-{index}"),
                "--source",
                "myvariant",
                "--build",
                "GRCh38",
            ],
        )?;
        let build = value.pointer("/result/items/0/identity/representations/0/build");
        if value.pointer("/query/build") != Some(&json!("GRCh38"))
            || value.pointer("/result/build") != Some(&json!("GRCh38"))
            || value.pointer("/result/items/0/record/id").is_none()
            || (index == 0 && build != Some(&json!("GRCh37")))
            || (index != 0
                && (build.is_some() || value.pointer("/status") != Some(&json!("partial"))))
            || (index != 0 && value.get("next").is_some())
        {
            return Err(format!(
                "caller build replaced source metadata or unknown identity lost: {value}"
            ));
        }
    }
    if stub.finish()?.len() != 7 {
        return Err("metadata checks made unexpected requests".into());
    }
    Ok(())
}
fn metadata_body(request: &str) -> Result<String, String> {
    if !request.contains("%2Cchrom%2Chg19%2Chg38&assembly=hg38 HTTP/1.1") {
        return Err(format!("source metadata fields missing: {request}"));
    }
    let index = request
        .split("q=metadata-")
        .nth(1)
        .and_then(|s| s.split('&').next())
        .ok_or("metadata selector absent")?;
    let mut hit =
        json!({"_id":"chr7:g.55259515T>G","chrom":"7","hg19":{"start":55259515,"end":55259515}});
    match index {
        "0" => {}
        "1" => {
            hit.as_object_mut().ok_or("object absent")?.remove("hg19");
        }
        "2" => *hit.get_mut("chrom").ok_or("chrom absent")? = json!("8"),
        "3" => *hit.pointer_mut("/hg19/start").ok_or("start absent")? = json!(55259514),
        "4" => *hit.pointer_mut("/hg19/end").ok_or("end absent")? = json!("55259515"),
        "5" => {
            let span = hit.get("hg19").cloned().ok_or("hg19 absent")?;
            hit.as_object_mut()
                .ok_or("object absent")?
                .insert("hg38".into(), span);
        }
        "6" => *hit.get_mut("_id").ok_or("id absent")? = json!("chr7:g.55259515del"),
        _ => return Err("unexpected metadata selector".into()),
    }
    Ok(json!({"total":1,"hits":[hit]}).to_string())
}

fn restricted_regions(env: &Environment) -> Result<(), String> {
    for group in ["restricted", "noncommercial"] {
        let allowed = command(
            env,
            &[
                "search",
                "variant",
                "--region",
                "chr7:140753336-140753336",
                "--build",
                "GRCh38",
                "--disable-sources",
                group,
            ],
        )?;
        if allowed.pointer("/status") != Some(&json!("complete"))
            || allowed.pointer("/page/returned") != Some(&json!(2))
        {
            return Err(format!(
                "unrelated component restriction blocked region selector: {allowed}"
            ));
        }
    }
    Ok(())
}

fn terms_budget() -> Result<(), String> {
    let registry = super::schema::Registry::embedded()?;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/cases/variant");
    let expected = json!([{"component": "clinvar",
 "terms": biomcp_adapters::myvariant::components::label(biomcp_adapters::myvariant::components::CLINVAR.terms).map_err(|e| e.to_string())?}]);
    for (name, cap) in [
        ("L858R", 743),
        ("G719A", 743),
        ("S768I", 743),
        ("T790M", 743),
        ("E709K", 745),
        ("L861Q", 745),
        ("Arg117Gly", 758),
        ("G719C", 742),
        ("G719S", 742),
        ("region", 1271),
        ("raw", 1504),
    ] {
        let mut value = super::run_case(&root.join(format!("search-{name}.yaml")), &registry)?;
        let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len() + 1;
        let source_terms = full - super::source_terms::size(&value, "")? - 1;
        value = super::source_terms::comparable(&value, "")?;
        let selector_bytes = subtract_selector(&mut value, !matches!(name, "region" | "raw"))?;
        let items = value
            .pointer_mut("/result/items")
            .and_then(Value::as_array_mut)
            .ok_or("missing source rows")?;
        let mut labeled = 0;
        for item in items {
            if let Some(terms) = item
                .as_object_mut()
                .and_then(|v| v.remove("component_terms"))
            {
                if terms != expected {
                    return Err("component terms changed during subtraction".into());
                }
                labeled += 1;
            }
        }
        let payload = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len() + 1;
        if labeled == 0
            || full - payload != labeled * 205 + selector_bytes + source_terms
            || payload > cap
        {
            return Err(format!(
                "{name}: full={full}, metadata={}, payload={payload}, comparable cap={cap}",
                full - payload
            ));
        }
    }
    Ok(())
}
fn subtract_selector(value: &mut Value, selected: bool) -> Result<usize, String> {
    let expected = serde_json::to_value(
        biomcp_adapters::myvariant::components::label(
            biomcp_adapters::myvariant::components::DBNSFP.terms,
        )
        .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let terms = value
        .pointer_mut("/result")
        .and_then(Value::as_object_mut)
        .ok_or("missing source result")?
        .remove("selector_terms");
    if !selected && terms.is_none() {
        return Ok(0);
    }
    if terms.as_ref() != Some(&expected) {
        return Err("selector terms changed during subtraction".into());
    }
    Ok(serde_json::to_vec(&json!({"selector_terms": expected}))
        .map_err(|e| e.to_string())?
        .len()
        - 1)
}
