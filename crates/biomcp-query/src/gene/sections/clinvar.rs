use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, HgncId};

use biomcp_adapters::clinvar;
use biomcp_fetch::Deadline;
use biomcp_types::{GeneSection, Reason, Section};
pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, genes: &[(HgncId, Gene)]) -> SectionResult {
    let leg = cx.fetch.leg(&clinvar::SPEC, deadline).fresh(cx.fresh);
    let mut error = None;
    let mut local = None;
    let values: Vec<_> = genes
        .iter()
        .filter_map(|(id, gene)| {
            // A gene without an NCBI Gene ID keeps its not-applicable section.
            let entrez = gene.entrez_id?;
            let section = match clinvar::gene_counts(&leg, entrez.get()) {
                Ok(data) => Section::Data {
                    source: GeneSection::ClinVar.source(),
                    data,
                },
                Err(failure) => {
                    error = Some(failure);
                    local = local.or(status::local(failure));
                    Section::unavailable(
                        GeneSection::ClinVar.source(),
                        status::reason(failure).unwrap_or(Reason::Unavailable),
                    )
                }
            };
            Some((*id, SectionValue::ClinVar(section)))
        })
        .collect();
    let report = leg.finish();
    SectionResult {
        dropped: false,
        values,
        source: status::source(GeneSection::ClinVar.source(), report, error),
        problems: Vec::new(),
        local,
    }
}
