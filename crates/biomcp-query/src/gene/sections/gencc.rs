use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, HgncId};

use biomcp_adapters::gencc;
use biomcp_fetch::Deadline;
use biomcp_types::{Reason, Section, Source};
pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, genes: &[(HgncId, Gene)]) -> SectionResult {
    let leg = cx.fetch.leg(&gencc::SPEC, deadline).fresh(cx.fresh);
    let mut error = None;
    let mut local = None;
    let values = genes
        .iter()
        .map(|(id, _)| {
            let section = match gencc::gene(&leg, *id) {
                Ok(Some(data)) => Section::Data {
                    source: Source::Gencc,
                    data,
                },
                Ok(None) => Section::Empty {
                    source: Source::Gencc,
                    record: None,
                },
                Err(failure) => {
                    error = Some(failure);
                    local = local.or(status::local(failure));
                    Section::unavailable(
                        Source::Gencc,
                        crate::status::reason(failure).unwrap_or(biomcp_types::Reason::Unavailable),
                    )
                }
            };
            (*id, SectionValue::Gencc(section))
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
            *value = SectionValue::Gencc(Section::unavailable(Source::Gencc, reason));
        }
    }
    SectionResult {
        values,
        source: status::source(Source::Gencc, report, error),
        problems: Vec::new(),
        local,
    }
}
