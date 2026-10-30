use super::{failure, identity};
use crate::{Action, Outcome};
use biomcp_adapters::{NativeWindow, chembl, opentargets};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    DrugGet, DrugHeader, DrugIndications, DrugRecords, DrugSection, DrugTargets, PageInfo,
    QueryBody, Request, Section, Source, Status, StepKind,
};
pub(super) fn chembl(
    leg: &Leg<'_>,
    query: &DrugGet,
    fresh: bool,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let source = Source::Chembl;
    let (row, metadata) = chembl::molecule(leg, query.input())?;
    if !matches!(
        query.sections(),
        [DrugSection::Targets] | [DrugSection::Indications]
    ) {
        chembl_card(row, metadata.fetched, query, fresh, out)?;
        return Ok(());
    }
    let header = identity::chembl_header(&row, &mut out.problems).map_err(|_| failure(source))?;
    let drug = identity::neutral(&header).map_err(|_| failure(source))?;
    let limit = query.section_limit().unwrap_or(10);
    let offset = query.section_offset().unwrap_or(0);
    match query.sections() {
        [DrugSection::Targets] => {
            let (mut page, meta) = chembl::mechanisms(leg, query.input(), limit, offset)?;
            let records = chembl::map_targets(&drug, std::mem::take(&mut page.window.rows))
                .map_err(|_| failure(source))?;
            out.problems.extend(page.problems);
            out.problems.extend(records.problems);
            targets(
                out,
                query,
                header,
                page.window.with_rows(records.value.into_vec()),
                meta.fetched,
                fresh,
            );
        }
        [DrugSection::Indications] => {
            let (mut page, meta) = chembl::indications(leg, query.input(), limit, offset)?;
            let records = chembl::map_indications(&drug, std::mem::take(&mut page.window.rows))
                .map_err(|_| failure(source))?;
            out.problems.extend(page.problems);
            out.problems.extend(records.problems);
            indications(
                out,
                query,
                header,
                page.window.with_rows(records.value.into_vec()),
                meta.fetched,
                fresh,
            );
        }
        _ => return Ok(()),
    }
    Ok(())
}
pub(super) fn opentargets(
    leg: &Leg<'_>,
    query: &DrugGet,
    fresh: bool,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let (row, meta) = opentargets::drug(leg, query.input())?;
    let Some(row) = row else {
        out.status = Status::Absent;
        return Ok(());
    };
    let source = Source::Opentargets;
    let header = opentargets::header(&row).map_err(|_| failure(source))?;
    out.problems.extend(header.problems);
    let header = header.value;
    let drug = identity::neutral(&header).map_err(|_| failure(source))?;
    let limit = query.section_limit().unwrap_or(10) as usize;
    let offset = query.section_offset().unwrap_or(0) as usize;
    match query.sections() {
        [DrugSection::Targets] => {
            let rows = opentargets::map_targets(&drug, row.mechanisms_of_action)
                .map_err(|_| failure(source))?;
            out.problems.extend(rows.problems);
            let rows = rows.value;
            let total = rows.len() as u64;
            let records = rows
                .into_vec()
                .into_iter()
                .skip(offset)
                .take(limit)
                .collect();
            targets(
                out,
                query,
                header,
                NativeWindow::new(
                    records,
                    offset as u32,
                    limit as u32,
                    Some(total),
                    total.saturating_sub(offset as u64).min(limit as u64) as u32,
                ),
                meta.fetched,
                fresh,
            );
        }
        [DrugSection::Indications] => {
            let rows = opentargets::map_indications(&drug, row.indications)
                .map_err(|_| failure(source))?;
            out.problems.extend(rows.problems);
            let rows = rows.value;
            let total = rows.len() as u64;
            let records = rows
                .into_vec()
                .into_iter()
                .skip(offset)
                .take(limit)
                .collect();
            indications(
                out,
                query,
                header,
                NativeWindow::new(
                    records,
                    offset as u32,
                    limit as u32,
                    Some(total),
                    total.saturating_sub(offset as u64).min(limit as u64) as u32,
                ),
                meta.fetched,
                fresh,
            );
        }
        _ => {
            out.status = Status::Complete;
            out.body = QueryBody::DrugCard(Some(identity::card(header, meta.fetched)));
            card_next(out, query, fresh, false);
        }
    }
    Ok(())
}
fn card_next(out: &mut Outcome, query: &DrugGet, fresh: bool, xrefs: bool) {
    for section in [
        DrugSection::Targets,
        DrugSection::Indications,
        DrugSection::Xrefs,
    ]
    .into_iter()
    .filter(|s| *s != DrugSection::Xrefs || xrefs)
    {
        if let Ok(get) = query.for_section(section) {
            out.next.push((
                StepKind::Section,
                Action::Run {
                    request: Request::GetDrug(get),
                    fresh,
                },
            ));
        }
    }
}
fn page<T>(
    out: &mut Outcome,
    query: &DrugGet,
    window: NativeWindow<T>,
    fetched: Option<i64>,
    fresh: bool,
) -> Section<DrugRecords<T>> {
    let offset = window.offset();
    let total = window.total().unwrap_or(0);
    let next_offset = window.next_offset();
    let records = window.rows.into_boxed_slice();
    let returned = records.len() as u32;
    let source = query.source().source();
    out.status = if total == 0 {
        Status::Absent
    } else {
        Status::Complete
    };
    out.page = Some(PageInfo {
        offset,
        returned,
        total,
        pool: None,
        truncated: None,
    });
    if let Some(offset) = next_offset
        && let Ok(get) = query.at_section_offset(offset)
    {
        out.next.push((
            StepKind::Page,
            Action::Run {
                request: Request::GetDrug(get),
                fresh,
            },
        ));
    }
    if total == 0 {
        Section::Empty {
            source,
            record: query
                .input()
                .to_string()
                .parse()
                .ok()
                .map(|id| biomcp_types::SourceRecord { id, url: None }),
        }
    } else {
        Section::Data {
            source,
            data: DrugRecords {
                records,
                offset,
                total: Some(total),
                truncated: None,
                version: None,
                retrieved: biomcp_adapters::drug_sources::received_date(fetched),
            },
        }
    }
}
fn targets(
    out: &mut Outcome,
    query: &DrugGet,
    identity: DrugHeader,
    window: NativeWindow<biodata::DrugTargetRelation>,
    fetched: Option<i64>,
    fresh: bool,
) {
    let targets = page(out, query, window, fetched, fresh);
    out.body = QueryBody::DrugTargets(Some(DrugTargets { identity, targets }));
}
fn indications(
    out: &mut Outcome,
    query: &DrugGet,
    identity: DrugHeader,
    window: NativeWindow<biodata::DrugIndicationClaim>,
    fetched: Option<i64>,
    fresh: bool,
) {
    let indications = page(out, query, window, fetched, fresh);
    out.body = QueryBody::DrugIndications(Some(DrugIndications {
        identity,
        indications,
    }));
}

fn chembl_card(
    row: chembl::Molecule,
    fetched: Option<i64>,
    query: &DrugGet,
    fresh: bool,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let source = Source::Chembl;
    let limit = query.section_limit().unwrap_or(10);
    let offset = query.section_offset().unwrap_or(0);
    let mut card =
        identity::chembl_card(row, fetched, &mut out.problems).map_err(|_| failure(source))?;
    let has_xrefs = !card.cross_references.is_empty();
    let xrefs = std::mem::take(&mut card.cross_references);
    if query.sections() == [DrugSection::Xrefs] {
        let total = xrefs.len() as u64;
        let records = xrefs
            .into_vec()
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect();
        let xrefs = page(
            out,
            query,
            NativeWindow::new(
                records,
                offset,
                limit,
                Some(total),
                total
                    .saturating_sub(u64::from(offset))
                    .min(u64::from(limit)) as u32,
            ),
            fetched,
            fresh,
        );
        out.body = QueryBody::DrugXrefs(Some(biomcp_types::DrugXrefs {
            identity: card.header,
            xrefs,
        }));
        return Ok(());
    }
    out.status = Status::Complete;
    out.body = QueryBody::DrugCard(Some(card));
    card_next(out, query, fresh, has_xrefs);
    if let Ok(get) = DrugGet::new(query.input().clone(), biomcp_types::DrugSource::Opentargets) {
        out.next.push((
            StepKind::Section,
            Action::Run {
                request: Request::GetDrug(get),
                fresh,
            },
        ));
    }
    Ok(())
}
