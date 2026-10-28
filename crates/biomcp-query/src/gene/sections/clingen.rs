use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, HgncId};
use biomcp_adapters::clingen;
use biomcp_fetch::{Deadline, Leg, LegReport, RequestError};
use biomcp_types::{ClinGenGene, Section, Source, SourceOutcome};

pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, genes: &[(HgncId, Gene)]) -> SectionResult {
    let leg = cx.fetch.leg(&clingen::SPEC, deadline).fresh(cx.fresh);
    let mut local = None;
    let mut error = None;
    let mut report = LegReport::default();
    let values = genes
        .iter()
        .map(|(id, _)| {
            let validity = clingen::validity(&leg, *id);
            let validity_error = validity.as_ref().err().copied();
            let validity_status = part_status(&leg, validity_error, &mut report);
            let dosage = clingen::dosage(&leg, *id);
            let dosage_error = dosage.as_ref().err().copied();
            let dosage_status = part_status(&leg, dosage_error, &mut report);
            error = dosage_error.or(validity_error).or(error);
            local = local
                .or(validity_error.and_then(status::local))
                .or(dosage_error.and_then(status::local));
            let dosage = dosage.unwrap_or_default();
            let data = ClinGenGene {
                validity_status,
                dosage_status,
                validity: validity.unwrap_or_default(),
                haploinsufficiency: dosage.as_ref().map(|row| row.haploinsufficiency.clone()),
                triplosensitivity: dosage.as_ref().map(|row| row.triplosensitivity.clone()),
                dosage_date: dosage.map(|row| row.date),
            };
            (
                *id,
                SectionValue::ClinGen(Section::Data {
                    source: Source::ClinGen,
                    data,
                }),
            )
        })
        .collect();
    SectionResult {
        values,
        source: status::source(Source::ClinGen, report, error),
        problems: Vec::new(),
        local,
    }
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
    status::source(Source::ClinGen, report, error)
}
