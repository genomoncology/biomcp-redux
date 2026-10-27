use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, HgncId};
use biomcp_adapters::clingen;
use biomcp_fetch::Deadline;
use biomcp_types::{ClinGenGene, Section, Source};

pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, genes: &[(HgncId, Gene)]) -> SectionResult {
    let mut local = None;
    let mut source = None;
    let values = genes
        .iter()
        .map(|(id, _)| {
            let validity_leg = cx.fetch.leg(&clingen::SPEC, deadline).fresh(cx.fresh);
            let dosage_leg = cx.fetch.leg(&clingen::SPEC, deadline).fresh(cx.fresh);
            let validity = clingen::validity(&validity_leg, *id);
            let dosage = clingen::dosage(&dosage_leg, *id);
            let validity_error = validity.as_ref().err().copied();
            let dosage_error = dosage.as_ref().err().copied();
            local = local
                .or(validity_error.and_then(status::local))
                .or(dosage_error.and_then(status::local));
            let validity_status =
                status::source(Source::ClinGen, validity_leg.finish(), validity_error);
            let dosage_status = status::source(Source::ClinGen, dosage_leg.finish(), dosage_error);
            for outcome in [&validity_status, &dosage_status] {
                if let Some(source) = &mut source {
                    status::retain_source_state(source, outcome);
                } else {
                    source = Some(outcome.clone());
                }
            }
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
        source: source.unwrap_or_else(|| {
            biomcp_adapters::registry::outcome(Source::ClinGen, biomcp_types::SourceState::Ok, None)
        }),
        problems: Vec::new(),
        local,
    }
}
