use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, HgncId, Text};
use biomcp_adapters::clingen::{self, actionability};
use biomcp_fetch::{Deadline, Leg, LegReport, RequestError};
use biomcp_types::{
    ClinGenActionability, ClinGenActionabilityContext, ClinGenGene, GeneSection, Section,
    SourceOutcome,
};

pub(super) fn run(
    cx: &Ctx<'_>,
    deadline: Deadline<'_>,
    genes: &[(HgncId, Gene)],
    single_gene: bool,
) -> SectionResult {
    let leg = cx.fetch.leg(&clingen::SPEC, deadline).fresh(cx.fresh);
    let evidence_leg = leg.sibling(&clingen::REGION_SPEC);
    let mut local = None;
    let mut dropped = false;
    let mut problems = Vec::new();
    let mut error = None;
    let mut report = LegReport::default();
    let values = genes
        .iter()
        .map(|(id, gene)| {
            let validity = clingen::validity(&leg, *id, &gene.symbol);
            let validity_error = validity.as_ref().err().copied();
            let validity_status = part_status(&leg, validity_error, &mut report);
            let dosage = clingen::dosage(&leg, *id, &gene.symbol);
            let dosage_error = dosage.as_ref().err().copied();
            let dosage_status = part_status(&leg, dosage_error, &mut report);
            error = dosage_error.or(validity_error).or(error);
            local = local
                .or(validity_error.and_then(status::local))
                .or(dosage_error.and_then(status::local));
            let dosage = dosage
                .map(|mapped| {
                    dropped |= mapped.refusals == biomcp_adapters::Refusal::Refused;
                    problems.extend(mapped.problems);
                    mapped.value
                })
                .unwrap_or_default();
            let validity = validity
                .map(|mapped| {
                    dropped |= mapped.refusals == biomcp_adapters::Refusal::Refused;
                    problems.extend(mapped.problems);
                    mapped.value
                })
                .unwrap_or_default();
            let (evidence, dosage_evidence_status) =
                dosage_evidence(&evidence_leg, gene, &mut problems, &mut dropped);
            let data = ClinGenGene {
                validity_status,
                dosage_status,
                validity,
                haploinsufficiency: dosage
                    .as_ref()
                    .and_then(|row| row.haploinsufficiency.clone()),
                triplosensitivity: dosage
                    .as_ref()
                    .and_then(|row| row.triplosensitivity.clone()),
                dosage_date: dosage.and_then(|row| row.date),
                dosage_evidence_status,
                haploinsufficiency_evidence: evidence.haploinsufficiency,
                triplosensitivity_evidence: evidence.triplosensitivity,
                actionability: single_gene.then(|| read_actionability(cx, deadline, gene)),
            };
            (
                *id,
                SectionValue::ClinGen(Section::Data {
                    source: GeneSection::ClinGen.source(),
                    data,
                }),
            )
        })
        .collect();
    SectionResult {
        dropped,
        values,
        source: status::source(GeneSection::ClinGen.source(), report, error),
        problems,
        local,
    }
}

fn read_actionability(cx: &Ctx<'_>, deadline: Deadline<'_>, gene: &Gene) -> ClinGenActionability {
    let read = |context| {
        let leg = cx.fetch.leg(&actionability::SPEC, deadline).fresh(cx.fresh);
        let (curations, error) = match actionability::curations(&leg, &gene.symbol, context) {
            Ok(rows) => (rows, None),
            Err(error) => (Box::default(), Some(error)),
        };
        let mut status = status::source(GeneSection::ClinGen.source(), leg.finish(), error);
        status.count = error.is_none().then_some(curations.len() as u64);
        ClinGenActionabilityContext { status, curations }
    };
    ClinGenActionability {
        report_index: Text::new(actionability::REPORT_INDEX)
            .expect("constant ClinGen report index URL"),
        adult: read(actionability::Context::Adult),
        pediatric: read(actionability::Context::Pediatric),
    }
}

/// Keep the established summary usable when the detailed file is unavailable.
fn dosage_evidence(
    leg: &Leg<'_>,
    gene: &Gene,
    problems: &mut Vec<biomcp_types::Problem>,
    dropped: &mut bool,
) -> (clingen::Evidence, Option<SourceOutcome>) {
    let Some(id) = gene.entrez_id else {
        return (clingen::Evidence::default(), None);
    };
    let result = clingen::evidence(leg, id);
    let source = status::source(
        GeneSection::ClinGen.source(),
        leg.take_report(),
        result.as_ref().err().copied(),
    );
    let evidence = result
        .map(|mapped| {
            *dropped |= mapped.refusals == biomcp_adapters::Refusal::Refused;
            problems.extend(mapped.problems);
            mapped.value
        })
        .unwrap_or_default();
    (evidence, Some(source))
}

fn part_status(
    leg: &Leg<'_>,
    error: Option<RequestError>,
    combined: &mut LegReport,
) -> SourceOutcome {
    let report = leg.take_report();
    combined.stale |= report.stale;
    combined.failures.extend(report.failures.iter().copied());
    combined
        .cache_problems
        .extend(report.cache_problems.iter().copied());
    status::source(GeneSection::ClinGen.source(), report, error)
}
