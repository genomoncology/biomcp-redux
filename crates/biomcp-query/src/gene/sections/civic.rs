use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, GeneSection, HgncId, Problem, ProblemCode, Reason, Section, Source, Text};
use biomcp_adapters::civic;
use biomcp_fetch::{Deadline, RequestError};
pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, genes: &[(HgncId, Gene)]) -> SectionResult {
    let ids: Vec<_> = genes
        .iter()
        .filter_map(|(_, gene)| gene.entrez_id.map(|id| (gene.symbol.clone(), id)))
        .collect();
    let leg = cx.fetch.leg(&civic::SPEC, deadline).fresh(cx.fresh);
    let result = if ids.is_empty() {
        Ok(Vec::new())
    } else {
        civic::genes(&leg, &ids)
    };
    let error = result.as_ref().err().copied();
    let mut source = status::source(Source::Civic, leg.finish(), error);
    let mut problems = Vec::new();
    let values = genes
        .iter()
        .map(|(id, gene)| {
            let section = if let Some(entrez) = gene.entrez_id {
                match &result {
                    Ok(values) => match values.iter().find(|value| value.id == entrez && value.symbol == gene.symbol) {
                        Some(value) => {
                            status::append(&mut problems, value.civic.problems.clone());
                            value.civic.value.clone()
                        }
                        None => Section::Unavailable {
                            source: Source::Civic,
                            reason: Reason::BadReply,
                        },
                    },
                    Err(error) => Section::Unavailable {
                        source: Source::Civic,
                        reason: match error {
                            RequestError::Source(error) => error.reason,
                            _ => Reason::Unavailable,
                        },
                    },
                }
            } else {
                problems.push(Problem {
                    source: Source::Hgnc,
                    record: Text::new(&id.to_string()).ok(),
                    field: "entrez_id".into(),
                    code: ProblemCode::ValueUnknown,
                });
                Section::Unavailable {
                    source: Source::Civic,
                    reason: Reason::NoJoinId,
                }
            };
            (*id, SectionValue::Civic(section))
        })
        .collect();
    if ids.is_empty() {
        source.state = biodata::SourceState::Failed;
        source.reason = Some(Reason::NoJoinId);
    }
    SectionResult {
        section: GeneSection::Civic,
        values,
        source,
        problems,
        local: error.and_then(status::local),
    }
}
