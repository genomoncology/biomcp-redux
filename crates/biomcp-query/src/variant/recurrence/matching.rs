use biodata::{Build, Chromosome, Problem, ProblemCode, Source, Text, Variant};
use biomcp_adapters::cbioportal::Mutation;
use std::collections::BTreeSet;
pub(super) fn count(identity: &Variant, rows: &[Mutation], study: &Text) -> (u64, Vec<Problem>) {
    let mut samples = BTreeSet::new();
    let mut problems = Vec::new();
    for row in rows {
        match matches(identity, row) {
            Some(true) => {
                samples.insert(row.sample_id.as_str());
            }
            Some(false) => {}
            None => problems.push(problem(
                Source::Cbioportal,
                study.as_str(),
                "mutation.exact_allele",
            )),
        }
    }
    problems.dedup();
    (samples.len() as u64, problems)
}
fn matches(identity: &Variant, row: &Mutation) -> Option<bool> {
    let build = match row.ncbi_build.as_ref()?.as_str() {
        "GRCh37" | "37" => Build::Grch37,
        "GRCh38" | "38" => Build::Grch38,
        _ => return None,
    };
    let chromosome = row.chr.as_ref()?.as_str().parse::<Chromosome>().ok()?;
    let reference = row
        .reference_allele
        .as_ref()?
        .as_str()
        .parse::<biodata::Sequence>()
        .ok()?;
    let alternate = row
        .variant_allele
        .as_ref()?
        .as_str()
        .parse::<biodata::Sequence>()
        .ok()?;
    let start = row.start_position?;
    let end = row.end_position?;
    // MAF deletion/insertion conventions require source-qualified placement evidence.
    // Only directly comparable nonempty replacements are matched, never normalized.
    if reference.is_empty()
        || alternate.is_empty()
        || start == 0
        || end < start
        || end - start + 1 != reference.len() as u64
    {
        return None;
    }
    Some(identity.representations.iter().any(|r| {
        let Some(allele) = r.allele.as_ref() else {
            return false;
        };
        r.build == Some(build)
            && Chromosome::from_accession(allele.location().sequence, build).ok()
                == Some(chromosome)
            && u64::from(allele.location().span.start()) + 1 == start
            && u64::from(allele.location().span.end()) == end
            && allele.reference() == &reference
            && allele.alternate() == &alternate
    }))
}
pub(super) fn problem(source: Source, record: &str, field: &str) -> Problem {
    Problem {
        source,
        record: Text::new(record).ok(),
        field: field.into(),
        code: ProblemCode::ValueUnknown,
    }
}
