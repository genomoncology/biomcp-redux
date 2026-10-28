//! Release projection requires a matching total in the same source response.
use biodata::{Attribution, Build, PopulationDataType, PopulationFrequency, Text, VariantRef};
use biomcp_adapters::gnomad::variant::{CohortFrequency, Subset, Variant};
use biomcp_types::Source;
use biomcp_types::{Problem, ProblemCode};

pub(super) fn frequency(
    raw: &Variant,
    subset: &Subset,
    data_type: PopulationDataType,
    va: Option<&Vec<CohortFrequency>>,
    name: &str,
    subject: Option<&VariantRef>,
    problems: &mut Vec<Problem>,
) -> Option<PopulationFrequency> {
    let result = checked(subset, va, name, subject);
    let (version, subject) = match result {
        Ok(value) => value,
        Err(field) => {
            // Preserve the existing source-level missing-release problem once.
            if !problems.iter().any(|p| p.field == field) {
                problems.push(Problem {
                    source: Source::Gnomad,
                    record: Some(raw.variant_id.clone()),
                    field,
                    code: ProblemCode::ValueUnknown,
                });
            }
            return None;
        }
    };
    let dataset = Source::Gnomad.text();
    PopulationFrequency::new(
        subject.clone(),
        Build::GRCh38,
        dataset,
        version.clone(),
        Attribution {
            source: Source::Gnomad.text(),
            record: Some(raw.variant_id.clone()),
            release: Some(version.clone()),
            version: None,
            submitter: None,
            evaluated: None,
            url: None,
        },
    )
    .with_stratum(Some(data_type), None, None, None)
    .and_then(|frequency| frequency.with_counts(Some(subset.ac), Some(subset.an), None, None))
    .ok()
    .map(|f| f.with_filters(subset.filters.clone().into()))
}

fn checked<'a>(
    subset: &Subset,
    va: Option<&'a Vec<CohortFrequency>>,
    name: &str,
    subject: Option<&'a VariantRef>,
) -> Result<(Text, &'a VariantRef), Box<str>> {
    let rows = va.ok_or_else(|| Box::<str>::from("dataset_version"))?;
    let mut totals = rows.iter().filter(|v| v.cohort.id.as_str() == "ALL");
    let row = totals
        .next()
        .ok_or_else(|| Box::<str>::from("dataset_version"))?;
    if totals.next().is_some() {
        return Err(format!("va.{name}.cohort").into());
    }
    if (row.focus_allele_count, row.locus_allele_count) != (subset.ac, subset.an) {
        return Err(format!("va.{name}.counts").into());
    }
    let version = row
        .derived_from
        .as_ref()
        .and_then(|d| d.version.as_deref())
        .and_then(|version| Text::new(version).ok())
        .ok_or_else(|| Box::<str>::from("dataset_version"))?;
    let subject = subject.ok_or_else(|| Box::<str>::from("subject"))?;
    Ok((version, subject))
}
