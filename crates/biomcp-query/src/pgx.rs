//! CPIC retrieval preserves source contexts and performs no clinical inference.
use biomcp_types::ReplyBody;
mod annotations;
mod annotations_record;
mod next;
use crate::{Ctx, Outcome, status};
pub(crate) use annotations::annotation;
use biomcp_adapters::cpic;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    PgxCard, PgxDiplotype, PgxGet, PgxGuidelines, PgxHeader, PgxPage, PgxPageInfo,
    PgxRecommendations, PgxSearch, PgxSection, PgxSource, PgxTotalKind, Problem, ProblemCode,
    Reason, ReplyData, Request, Source, SourceOutcome, SourceState,
};
/// Discover one independent native CPIC pair page.
pub(crate) fn search(cx: &Ctx<'_>, query: &PgxSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&cpic::SPEC, deadline).fresh(cx.fresh);
    let mut problems = Vec::new();
    let mut consumed = 0;
    let mut refusals = biomcp_adapters::Refusal::None;
    let reply = cpic::search(&leg, query).map(|page| {
        refusals.include(page.refusals);
        problems.extend(page.problems);
        consumed = page.consumed;
        let info = page_info(query.offset(), page.items.len(), page.total);
        (PgxPage {
            header: header(query.gene()),
            items: page.items.into(),
            page: info,
        })
        .with_query(query.clone())
    });
    let empty = (PgxPage {
        header: header(query.gene()),
        items: Box::default(),
        page: page_info(query.offset(), 0, 0),
    })
    .with_query(query.clone());
    let mut out = finish(leg, reply, empty, Source::Cpic);
    out.include_refusals(refusals);
    status::append(&mut out.problems, problems);
    out.next = next::steps_with(&Request::SearchPgx(query.clone()), &mut out, Some(consumed));
    command.finish(deadline.remaining());
    out
}
/// Read one explicit card or section with the same source policy on every path.
pub(crate) fn get(cx: &Ctx<'_>, query: &PgxGet) -> Outcome {
    if query.source() == PgxSource::Clinpgx {
        return annotations::get(cx, query);
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&cpic::SPEC, deadline).fresh(cx.fresh);
    let mut problems = Vec::new();
    let mut consumed = 0;
    let mut refusals = biomcp_adapters::Refusal::None;
    let mut empty_reason = None;
    let reply = read(
        &leg,
        query,
        &mut problems,
        &mut consumed,
        &mut refusals,
        &mut empty_reason,
    );
    let mut out = finish(leg, reply, empty(query), Source::Cpic);
    if let Some(message) = empty_reason
        && let Some(source) = out.sources.iter_mut().find(|s| s.source == Source::Cpic)
    {
        source
            .notices
            .push(biomcp_types::SourceNotice::RequestGuidance {
                data_date: None,
                message: message.into(),
                retry_after_seconds: None,
            });
    }
    out.include_refusals(refusals);
    status::append(&mut out.problems, problems);
    out.next = next::steps_with(&Request::GetPgx(query.clone()), &mut out, Some(consumed));
    command.finish(deadline.remaining());
    out
}
fn read(
    leg: &Leg<'_>,
    q: &PgxGet,
    problems: &mut Vec<Problem>,
    consumed: &mut u32,
    refusals: &mut biomcp_adapters::Refusal,
    empty_reason: &mut Option<String>,
) -> Result<ReplyData, RequestError> {
    let header = header(q.gene());
    Ok(match q.section() {
        None => {
            let page = cpic::card(leg, q)?;
            *consumed = page.consumed;
            refusals.include(page.refusals);
            problems.extend(page.problems);
            biomcp_types::PgxCard::into_reply(
                q.clone(),
                page.items
                    .into_iter()
                    .next()
                    .map(|identity| PgxCard { header, identity }),
            )
        }
        Some(PgxSection::Recommendations) if q.drug_id().is_some() => {
            let p = cpic::drug_recommendations(leg, q)?;
            *consumed = p.consumed;
            refusals.include(p.refusals);
            problems.extend(p.problems);
            let phenotypes = drug_phenotypes(leg, q, &p.items, p.total, empty_reason);
            (PgxRecommendations::Table {
                header,
                phenotypes: phenotypes.into(),
                page: page_info(q.section_offset(), p.items.len(), p.total),
                items: p.items.into(),
            })
            .with_query(q.clone())
        }
        Some(PgxSection::Recommendations) => {
            let p = cpic::recommendations(leg, q)?;
            *consumed = p.consumed;
            refusals.include(p.refusals);
            problems.extend(p.problems);
            (PgxRecommendations::View {
                header,
                page: page_info(q.section_offset(), p.items.len(), p.total),
                items: p.items.into(),
            })
            .with_query(q.clone())
        }
        Some(PgxSection::Guidelines) => {
            let p = cpic::guidelines(leg, q)?;
            *consumed = p.consumed;
            refusals.include(p.refusals);
            problems.extend(p.problems);
            (PgxGuidelines {
                header,
                page: page_info(q.section_offset(), p.items.len(), p.total),
                items: p.items.into(),
            })
            .with_query(q.clone())
        }
        Some(PgxSection::Diplotype) => {
            let page = cpic::diplotype(leg, q)?;
            *consumed = page.consumed;
            refusals.include(page.refusals);
            problems.extend(page.problems);
            (PgxDiplotype {
                header,
                diplotypes: page.items.into(),
            })
            .with_query(q.clone())
        }
        Some(PgxSection::Annotations) => return Err(biomcp_fetch::InputError::Serialization.into()),
    })
}
/// Say why a gene and drug have no recommendation rows, from CPIC's own pair row.
/// The pair read only explains; when it fails, the notice names that failure instead.
fn empty_pair(leg: &Leg<'_>, q: &PgxGet) -> Option<String> {
    let drug = q.drug_id()?;
    let gene = q.gene();
    let page = match cpic::pair(leg, q) {
        Ok(page) => page,
        Err(error) => {
            let reason = status::reason(error)
                .map_or_else(|| "a local error".to_owned(), |reason| reason.to_string());
            return Some(format!(
                "CPIC has no {gene} recommendations for drug {drug}; reading CPIC's pair row to say why failed ({reason})."
            ));
        }
    };
    let Some(pair) = page.items.first() else {
        return Some(format!(
            "CPIC lists no {gene} pair with drug {drug}, so it has no recommendations for it; search pgx --gene {gene} --source cpic lists the gene's pairs."
        ));
    };
    let name = pair
        .drugname
        .as_deref()
        .map_or_else(|| drug.to_string(), |name| format!("{name} ({drug})"));
    let guideline = if pair.guidelinename.is_none() && pair.guidelineurl.is_none() {
        " has no CPIC guideline and"
    } else {
        ""
    };
    Some(if pair.provisional == Some(true) {
        format!(
            "CPIC marks the {gene} and {name} pair provisional; the pair{guideline} has no CPIC recommendations."
        )
    } else {
        format!(
            "CPIC lists the {gene} and {name} pair, but the pair{guideline} has no CPIC recommendations."
        )
    })
}
/// The first page's phenotype list, and why a table is empty or its list is missing.
fn drug_phenotypes(
    leg: &Leg<'_>,
    q: &PgxGet,
    items: &[biodata::PgxDrugRecommendation],
    total: u64,
    empty_reason: &mut Option<String>,
) -> Vec<biomcp_types::PgxPhenotypeRows> {
    let unfiltered = q.phenotype().is_none();
    let mut failed = None;
    // Rows for the gene and drug before any phenotype filter, when known.
    let (phenotypes, all_rows) = if q.section_offset() != 0 {
        (Vec::new(), unfiltered.then_some(total))
    } else if unfiltered && total == items.len() as u64 {
        (
            tally(items.iter().filter_map(|r| gene_phenotype(r, q))),
            Some(total),
        )
    } else {
        match cpic::drug_phenotypes(leg, q) {
            Ok(list) => (tally(list.phenotypes.into_iter()), Some(list.rows)),
            Err(error) => {
                failed = Some(
                    status::reason(error)
                        .map_or_else(|| "a local error".to_owned(), |reason| reason.to_string()),
                );
                (Vec::new(), unfiltered.then_some(total))
            }
        }
    };
    let (gene, drug) = (q.gene(), drug_label(q));
    *empty_reason = match (total, failed, q.phenotype(), all_rows) {
        (0, Some(reason), Some(wanted), _) => Some(format!(
            "CPIC has no {gene} recommendations for {drug} with phenotype {}; \
             reading the phenotypes CPIC lists failed ({reason}).",
            wanted.as_str()
        )),
        (_, Some(reason), _, _) => Some(format!(
            "Reading the phenotypes CPIC lists for {gene} and {drug} failed ({reason}); \
             the rows shown are complete."
        )),
        (0, None, _, Some(0)) => empty_pair(leg, q),
        (0, None, Some(_), Some(rows)) if phenotypes.is_empty() => Some(format!(
            "CPIC's {rows} {gene} recommendation rows for {drug} carry no phenotype, so \
             --phenotype cannot select them; drop --phenotype to read them."
        )),
        (0, None, Some(wanted), Some(_)) => Some(format!(
            "CPIC has no {gene} recommendations for {drug} with phenotype {}; \
             result.phenotypes lists the phenotypes it has, spelled as --phenotype needs them.",
            wanted.as_str()
        )),
        _ => None,
    };
    phenotypes
}
fn drug_label(q: &PgxGet) -> String {
    q.drug_id().map_or_else(String::new, ToString::to_string)
}
fn gene_phenotype(row: &biodata::PgxDrugRecommendation, q: &PgxGet) -> Option<Box<str>> {
    let gene = q.gene().to_string();
    row.phenotypes
        .as_ref()?
        .iter()
        .find(|(k, _)| k.to_string() == gene)
        .map(|(_, v)| v.clone())
}
/// Count rows per phenotype, keeping first-seen order.
fn tally(phenotypes: impl Iterator<Item = Box<str>>) -> Vec<biomcp_types::PgxPhenotypeRows> {
    let mut out: Vec<biomcp_types::PgxPhenotypeRows> = Vec::new();
    for phenotype in phenotypes {
        match out.iter_mut().find(|p| p.phenotype == phenotype) {
            Some(seen) => seen.rows += 1,
            None => out.push(biomcp_types::PgxPhenotypeRows { phenotype, rows: 1 }),
        }
    }
    out
}
fn empty(q: &PgxGet) -> ReplyData {
    let header = header(q.gene());
    let page = page_info(q.section_offset(), 0, 0);
    match q.section() {
        None | Some(PgxSection::Annotations) => biomcp_types::PgxCard::into_reply(q.clone(), None),
        Some(PgxSection::Recommendations) if q.drug_id().is_some() => (PgxRecommendations::Table {
            header,
            phenotypes: Box::default(),
            items: Box::default(),
            page,
        })
        .with_query(q.clone()),
        Some(PgxSection::Recommendations) => (PgxRecommendations::View {
            header,
            items: Box::default(),
            page,
        })
        .with_query(q.clone()),
        Some(PgxSection::Guidelines) => (PgxGuidelines {
            header,
            items: Box::default(),
            page,
        })
        .with_query(q.clone()),
        Some(PgxSection::Diplotype) => (PgxDiplotype {
            header,
            diplotypes: Box::default(),
        })
        .with_query(q.clone()),
    }
}
fn finish(
    leg: Leg<'_>,
    reply: Result<ReplyData, RequestError>,
    empty: ReplyData,
    source: Source,
) -> Outcome {
    let error = reply.as_ref().err().copied();
    let source = status::source(source, leg.finish(), error);
    let state = if reply.is_err() {
        crate::status::item_status(false, true, false)
    } else if source.state == SourceState::Stale {
        crate::status::item_status(true, false, true)
    } else {
        crate::status::item_status(true, false, false)
    };
    crate::status::finish(
        [state],
        crate::status::Parts {
            dropped: false,
            body: reply.unwrap_or(empty),
            page: None,
            problems: record_problem(&source).into_iter().collect(),
            sources: vec![source],
            next: vec![],
            local: error.and_then(status::local),
        },
    )
}
fn header(gene: &biodata::GeneSymbol) -> PgxHeader {
    PgxHeader {
        gene: gene.clone(),
        source: PgxSource::Cpic,
    }
}
fn page_info(offset: u32, returned: usize, total: u64) -> PgxPageInfo {
    PgxPageInfo {
        offset,
        returned: returned as u32,
        total,
        total_kind: PgxTotalKind::Source,
    }
}

fn record_problem(source: &SourceOutcome) -> Option<Problem> {
    (source.reason == Some(Reason::BadReply)).then(|| Problem {
        source: source.source,
        record: None,
        field: "record".into(),
        code: ProblemCode::ValueUnknown,
    })
}
