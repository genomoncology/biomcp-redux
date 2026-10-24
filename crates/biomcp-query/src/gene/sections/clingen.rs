use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, HgncId, Reason, Section, Source};
use biomcp_adapters::clingen;
use biomcp_fetch::{Deadline, RequestError};
pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, genes: &[(HgncId, Gene)]) -> SectionResult {
    let leg = cx.fetch.leg(&clingen::SPEC, deadline).fresh(cx.fresh);
    let mut error = None;
    let values = genes
        .iter()
        .map(|(id, _)| {
            let section = match clingen::gene(&leg, *id) {
                Ok(Some(data)) => Section::Data {
                    source: Source::ClinGen,
                    data,
                },
                Ok(None) => Section::Empty {
                    source: Source::ClinGen,
                    record: None,
                },
                Err(failure) => {
                    error = Some(failure);
                    Section::Unavailable {
                        source: Source::ClinGen,
                        reason: match failure {
                            RequestError::Source(failure) => failure.reason,
                            _ => Reason::Unavailable,
                        },
                    }
                }
            };
            (*id, SectionValue::ClinGen(section))
        })
        .collect();
    let report = leg.finish();
    let mut values: Vec<_> = values;
    if !report.failures.is_empty() {
        let reason = report
            .failures
            .first()
            .map_or(Reason::Unavailable, |e| e.reason);
        for (_, value) in &mut values {
            *value = SectionValue::ClinGen(Section::Unavailable {
                source: Source::ClinGen,
                reason,
            });
        }
    }
    SectionResult {
        values,
        source: status::source(Source::ClinGen, report, error),
        problems: Vec::new(),
        local: error.and_then(status::local),
    }
}
