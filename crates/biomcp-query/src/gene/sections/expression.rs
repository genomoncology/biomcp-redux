use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, HgncId};

use biomcp_adapters::hpa;
use biomcp_fetch::Deadline;
use biomcp_types::{Section, Source};
pub(super) fn applicable(gene: &Gene) -> bool {
    gene.ensembl_id.is_some()
}
pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, genes: &[(HgncId, Gene)]) -> SectionResult {
    let leg = cx.fetch.leg(&hpa::SPEC, deadline).fresh(cx.fresh);
    let genes: Vec<_> = genes
        .iter()
        .filter_map(|(id, gene)| gene.ensembl_id.as_ref().map(|ensembl| (*id, ensembl)))
        .collect();
    let results = leg.batch(&genes, |leg, (_, id)| hpa::expression(leg, id));
    let mut problems = Vec::new();
    let mut error = None;
    let mut local = None;
    let values = genes
        .into_iter()
        .zip(results)
        .map(|((id, _), result)| {
            let section = match result {
                Ok(Some(mapped)) => {
                    status::append(&mut problems, mapped.problems);
                    Section::Data {
                        source: Source::Hpa,
                        data: mapped.value,
                    }
                }
                Ok(None) => Section::Empty {
                    source: Source::Hpa,
                    record: None,
                },
                Err(failure) => {
                    error = Some(failure);
                    local = local.or(status::local(failure));
                    Section::unavailable(Source::Hpa, crate::status::reason(failure))
                }
            };
            (id, SectionValue::Expression(section))
        })
        .collect();
    SectionResult {
        values,
        source: status::source(Source::Hpa, leg.finish(), error),
        problems,
        local,
    }
}
