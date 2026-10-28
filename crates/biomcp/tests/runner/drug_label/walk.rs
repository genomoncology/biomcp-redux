use crate::runner::transcript::trial as transcript_trial;
use super::super::configuration::invoke;
use super::field;
use super::{Check, both, get_args, get_setup, identity, record};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = Vec::new();
    for (n, limit) in [(4, 1), (5, 3), (2, 50)] {
        trials.push(transcript_trial(
            format!("drug-label/complete-walk-{n}-{limit}"),
            move || walk(n, limit).map_err(Into::into),
        ));
    }
    trials.push(transcript_trial(
        "drug-label/missing-field-is-unavailable",
        || missing().map_err(Into::into),
    ));
    trials
}
fn walk(n: usize, limit: u32) -> Check {
    let id = identity(n)?;
    let originals = vec![String::new(); 49]
        .into_iter()
        .chain([
            " α😀\\\"\n\t\u{1f} ".repeat(2000),
            String::new(),
            " final β\n ".to_owned(),
            String::new(),
        ])
        .collect::<Vec<_>>();
    let mut record = record(&id);
    *record
        .get_mut("dosage_and_administration")
        .ok_or("dose field")? = json!(originals);
    let (env, stub) = get_setup(&id, record)?;
    let mut args = get_args(&id)?;
    args.extend([
        "label".into(),
        "--label-section".into(),
        "dosage_and_administration".into(),
        "--section-limit".into(),
        limit.to_string(),
    ]);
    reconstruct(&env, args, &originals, limit)?;
    if stub.finish()?.len() != 1 {
        return Err("label continuation fetched the cached source again".into());
    }
    Ok(())
}
fn missing() -> Check {
    let id = identity(0)?;
    let mut record = record(&id);
    record
        .as_object_mut()
        .ok_or("record")?
        .remove("boxed_warning");
    let (env, stub) = get_setup(&id, record)?;
    let mut args = get_args(&id)?;
    args.extend([
        "label".into(),
        "--label-section".into(),
        "boxed_warning".into(),
    ]);
    let value = both(&env, args, 0)?;
    super::budget(&value, 608)?;
    if field(&value, "status") != "partial"
        || value.pointer("/result/label/state") != Some(&json!("unavailable"))
        || value.pointer("/result/label/reason") != Some(&json!("not_found"))
    {
        return Err("missing source field became an empty clinical answer".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("warm unavailable section fetched again".into());
    }
    Ok(())
}

pub(super) fn reconstruct(
    env: &super::Environment,
    mut args: Vec<String>,
    originals: &[String],
    limit: u32,
) -> Check {
    let mut rebuilt = Rebuilt {
        rows: vec![String::new(); originals.len()],
        seen: vec![false; originals.len()],
    };
    let mut offset = 0;
    let mut pages = 0;
    loop {
        let value = if pages == 0 {
            both(env, args.clone(), 0)?
        } else {
            invoke(env, args.clone(), 0)?
        };
        super::measure(&value)?;
        super::budget(&value, 7612)?;
        super::super::schema::Registry::embedded()?.validate(&value, &args, true)?;
        let data = value.pointer("/result/label/data").ok_or("label data")?;
        if field(data, "offset") != offset {
            return Err("fragment offset changed unit".into());
        }
        let fragments = field(data, "fragments").as_array().ok_or("fragments")?;
        if fragments.is_empty() || fragments.len() > limit as usize {
            return Err("fragment maximum changed".into());
        }
        rebuilt.append(data)?;
        offset += fragments.len();
        pages += 1;
        let Some(next) = value
            .get("next")
            .and_then(Value::as_array)
            .and_then(|steps| steps.iter().find(|s| field(s, "kind") == "page"))
        else {
            if field(data, "total") != offset {
                return Err("walk ended before final source fragment".into());
            }
            break;
        };
        args = field(next, "args")
            .as_array()
            .ok_or("next args")?
            .iter()
            .map(|v| v.as_str().map(str::to_owned).ok_or("arg".to_owned()))
            .collect::<Result<_, _>>()?;
        if pages > 100 {
            return Err("nonadvancing label walk".into());
        }
    }
    if rebuilt.rows != originals || rebuilt.seen.iter().any(|seen| !*seen) {
        return Err("label array reconstruction changed source text or an empty boundary".into());
    }
    Ok(())
}
struct Rebuilt {
    rows: Vec<String>,
    seen: Vec<bool>,
}
impl Rebuilt {
    fn append(&mut self, data: &Value) -> Check {
        let mut chars = field(data, "text").as_str().ok_or("text")?.chars();
        for fragment in field(data, "fragments").as_array().ok_or("fragments")? {
            let source_part = field(fragment, "source_part")
                .as_u64()
                .ok_or("source part")? as usize;
            let rebuilt = self.rows.get_mut(source_part).ok_or("array boundary")?;
            *self.seen.get_mut(source_part).ok_or("array boundary")? = true;
            if field(fragment, "source_offset") != rebuilt.chars().count() {
                return Err("native scalar offset changed".into());
            }
            rebuilt.extend(
                chars
                    .by_ref()
                    .take(field(fragment, "characters").as_u64().ok_or("characters")? as usize),
            );
        }
        if chars.next().is_some() {
            return Err("unpartitioned text".into());
        }
        Ok(())
    }
}
