//! Explicit external-input proof: unchanged source bodies never enter Git.
use super::field;
use super::{Check, both, get_args, identity, setup_raw, walk};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
pub(super) fn trials() -> Vec<Trial> {
    let Some(root) = std::env::var_os("BIOMCP_LABEL_CAPTURE_ROOT") else {
        return Vec::new();
    };
    (0..9)
        .map(|n| {
            let root = PathBuf::from(&root);
            Trial::test(format!("drug-label-captured/native-{n}"), move || {
                check(&root, n).map_err(Into::into)
            })
        })
        .collect()
}
fn native(root: &Path, n: usize) -> Result<(Value, Value, String), String> {
    let manifest: Vec<Value> = serde_json::from_slice(
        &std::fs::read(root.join("retained-manifest.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let id = identity(n)?;
    let entry = manifest
        .into_iter()
        .find(|m| field(m, "sha256") == field(&id, "original_sha256"))
        .ok_or("source manifest identity")?;
    let path = root
        .join("retained")
        .join(field(&entry, "file").as_str().ok_or("file")?);
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let hash = Sha256::digest(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    if Some(hash.as_str()) != field(&entry, "sha256").as_str() {
        return Err("retained body hash differs".into());
    }
    let body = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    Ok((
        body,
        entry,
        String::from_utf8(bytes).map_err(|e| e.to_string())?,
    ))
}
fn check(root: &Path, n: usize) -> Check {
    let id = identity(n)?;
    let (body, entry, raw) = native(root, n)?;
    if n < 3 {
        let brand = field(&id, "brand").as_str().ok_or("brand")?;
        let (env, stub) = setup_raw(
            raw.clone(),
            format!("openfda.brand_name.exact%3A%22{brand}%22"),
            200,
        )?;
        let args = vec![
            "search",
            "drug",
            brand,
            "--source",
            "openfda-label",
            "--limit",
            "1",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        let value = both(&env, args, 0)?;
        if value.pointer("/result/items/0/id") != Some(field(&id, "set_id")) {
            return Err("captured brand set changed".into());
        }
        if stub.finish()?.len() != 1 {
            return Err("warm native brand request changed".into());
        }
        if n != 2 {
            return report(n, &entry, false);
        }
    }
    // Enhertu's unchanged body was captured by brand; the SPL-get response delivery is synthetic.
    // The other six exact-ID requests retain their original request identity.
    let record = body.pointer("/results/0").ok_or("native record")?.clone();
    let (env, stub) = setup_raw(
        raw,
        format!(
            "openfda.spl_set_id%3A%22{}%22",
            field(&id, "set_id").as_str().ok_or("set")?
        ),
        200,
    )?;
    both(&env, get_args(&id)?, 0)?;
    for heading in biomcp_adapters::openfda_label::HEADINGS {
        let mut args = get_args(&id)?;
        args.extend(["label".into(), "--label-section".into(), heading.into()]);
        if let Some(parts) = field(&record, heading)
            .as_array()
            .filter(|parts| !parts.is_empty())
        {
            let originals = parts
                .iter()
                .map(|part| {
                    part.as_str()
                        .map(str::to_owned)
                        .ok_or("native part".to_owned())
                })
                .collect::<Result<Vec<_>, _>>()?;
            walk::reconstruct(&env, args, &originals, 1)?;
        } else {
            let value = both(&env, args, 0)?;
            if value.pointer("/result/label/state") != Some(&json!("unavailable")) {
                return Err("missing retained field changed state".into());
            }
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("native cached field walks made another request".into());
    }
    report(n, &entry, n == 2)
}
fn report(n: usize, entry: &Value, synthetic_get: bool) -> Check {
    use std::io::Write;
    writeln!(std::io::stdout(),"native label proof {n}: original_date={} original_sha256={} original_url={} local_delivery=true \
        fresh_provider_calls=0 synthetic_get_selector={synthetic_get}",
field(entry,
"retrieved_at"),
field(entry,
"sha256"),
field(entry,
"url")).map_err(|e|e.to_string())
}
