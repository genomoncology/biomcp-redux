//! Stored Orphadata rows mapped into one epidemiology item per Orphanet disorder.
use super::bundle::{AGES, PREVALENCE, SPEC};
use super::raw::{Native, NativePrevalence};
use crate::mapped::Problems;
use biodata::{Attribution, Curie};
use biomcp_fetch::{Find, InputError, KeyKind, Leg, RequestError, SetRow};
use biomcp_types::{OrphanetEpidemiology, OrphanetPrevalence};

/// The ORPHAcode of an Orphanet identifier in any of its common spellings.
pub fn orpha_code(id: &Curie) -> Option<&str> {
    let (prefix, code) = id.as_str().split_once(':')?;
    (["Orphanet", "ORPHA"]
        .iter()
        .any(|p| p.eq_ignore_ascii_case(prefix))
        && !code.is_empty()
        && code.bytes().all(|b| b.is_ascii_digit()))
    .then_some(code)
}

/// Read the stored estimates, onset and inheritance for each Orphanet code, in request order.
/// A code absent from both files has no item.
pub fn epidemiology(
    leg: &Leg<'_>,
    ids: &[Curie],
) -> Result<crate::Mapped<Vec<OrphanetEpidemiology>>, RequestError> {
    let mut codes: Vec<&str> = Vec::new();
    for code in ids.iter().filter_map(orpha_code) {
        if !codes.contains(&code) {
            codes.push(code);
        }
    }
    let mut mapped = crate::Mapped {
        value: Vec::new(),
        problems: Vec::new(),
        refusals: crate::Refusal::None,
    };
    if codes.is_empty() {
        return Ok(mapped);
    }
    let keys: Vec<String> = codes.iter().map(|code| Native::key(code)).collect();
    let keys: Vec<&str> = keys.iter().map(String::as_str).collect();
    let (rows, _) = super::bundle::read(leg, Find::AnyExact(KeyKind::Id, &keys))?;
    for code in codes {
        let rows: Vec<&SetRow<Native>> = rows
            .iter()
            .filter(|r| r.value.code.as_deref() == Some(code))
            .collect();
        if !rows.is_empty() {
            let id: Curie = Native::key(code)
                .parse()
                .map_err(|_| RequestError::Input(InputError::Serialization))?;
            mapped.value.push(item(id, &rows, &mut mapped.problems));
        }
    }
    Ok(mapped)
}

fn item(
    id: Curie,
    rows: &[&SetRow<Native>],
    problems: &mut Vec<biomcp_types::Problem>,
) -> OrphanetEpidemiology {
    let mut cx = Problems {
        source: SPEC.source,
        record: crate::mapped::record(&id),
        list: Vec::new(),
    };
    let part = |name: &str| rows.iter().find(|r| &*r.part == name).map(|r| &r.value);
    let prevalence = part(PREVALENCE);
    let ages = part(AGES);
    let first = prevalence.or(ages);
    let item = OrphanetEpidemiology {
        label: cx.text("Name", first.and_then(|r| r.name.as_deref())),
        disorder_type: cx.text(
            "DisorderType",
            first.and_then(|r| r.disorder_type.as_deref()),
        ),
        prevalence: prevalence
            .into_iter()
            .flat_map(|r| &r.prevalence)
            .map(|row| estimate(row, &mut cx))
            .collect(),
        onset: ages
            .into_iter()
            .flat_map(|r| &r.onset)
            .filter_map(|v| cx.text("AverageAgeOfOnset", Some(v)))
            .collect(),
        inheritance: ages
            .into_iter()
            .flat_map(|r| &r.inheritance)
            .filter_map(|v| cx.text("TypeOfInheritance", Some(v)))
            .collect(),
        attribution: Attribution {
            approved: None,
            published: None,
            source: SPEC.source.text(),
            record: crate::mapped::record(&id),
            // Both files of one release share the version; their dates differ by seconds.
            version: cx.text("JDBOR.version", first.and_then(|r| r.version.as_deref())),
            release: cx.text("JDBOR.date", first.map(|r| r.date.as_str())),
            submitter: None,
            evaluated: None,
            url: None,
        },
        id,
    };
    problems.extend(cx.list);
    item
}
fn estimate(row: &NativePrevalence, cx: &mut Problems) -> OrphanetPrevalence {
    // A class-only estimate has no mean value; the file writes 0.0 in its place.
    let class_only = row.qualification.as_deref() == Some("Class only");
    let value = row
        .value
        .as_deref()
        .filter(|v| !(class_only && *v == "0.0"));
    OrphanetPrevalence {
        id: cx.text("Prevalence.id", row.id.as_deref()),
        kind: cx.text("PrevalenceType", row.kind.as_deref()),
        qualification: cx.text("PrevalenceQualification", row.qualification.as_deref()),
        class: cx.text("PrevalenceClass", row.class.as_deref()),
        value: cx.text("ValMoy", value),
        geography: cx.text("PrevalenceGeographic", row.geography.as_deref()),
        source: cx.text("Source", row.source.as_deref()),
        // PubMed IDs are written as `NNNN[PMID]` in the underscore-separated source text.
        pmids: row
            .source
            .iter()
            .flat_map(|source| source.split('_'))
            .filter_map(|part| part.trim().strip_suffix("[PMID]"))
            .filter(|id| !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit()))
            .filter_map(|id| cx.checked_text("Source", Some(id)))
            .collect(),
        validation: cx.text("PrevalenceValidationStatus", row.validation.as_deref()),
    }
}
