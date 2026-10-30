//! Compose bounded first pages under the identity command's original deadline.
use super::{CellLineGet, Ctx, Outcome, QueryBody, Source, Status, StepKind, empty};
use biomcp_types::{CellLineCard, CellLineSection, Reason, Section};
pub(super) fn attach(cx: &Ctx<'_>, deadline: biomcp_fetch::Deadline<'_>, out: &mut Outcome) {
    let QueryBody::CellLineCard(card) = &out.body else {
        return;
    };
    let Some(header) = &card.header else { return };
    let selected =
        CellLineGet::accession(header.accession.clone()).with_section(CellLineSection::Chembl);
    let metadata = super::chembl::get_at(cx, &selected, &header.accession, deadline);
    let section = match &metadata.body {
        QueryBody::CellLineChembl(data) if metadata.status != Status::Failed => {
            if data.items.total == 0 {
                Section::Empty {
                    source: Source::Chembl,
                    record: None,
                }
            } else {
                Section::Data {
                    source: Source::Chembl,
                    data: data.clone(),
                }
            }
        }
        _ => Section::unavailable(
            Source::Chembl,
            metadata
                .sources
                .first()
                .and_then(|s| s.reason)
                .unwrap_or(Reason::BadReply),
        ),
    };
    if metadata.status != Status::Complete && metadata.status != Status::Absent {
        crate::status::item(out, true, false, true);
    }
    out.next.extend(
        metadata
            .next
            .iter()
            .filter(|(kind, _)| *kind == StepKind::Page)
            .cloned(),
    );
    out.sources.extend(metadata.sources);
    out.problems.extend(metadata.problems);
    out.local = out.local.or(metadata.local);
    if let QueryBody::CellLineCard(card) = &mut out.body {
        card.status = out.status();
        card.sections.chembl = Some(section);
    }
    let QueryBody::CellLineCard(card) = &out.body else {
        return;
    };
    let card = card.clone();
    actions(&card, out);
}
fn actions(card: &CellLineCard, out: &mut Outcome) {
    let Some(header) = &card.header else { return };
    for section in [
        CellLineSection::Names,
        CellLineSection::Identity,
        CellLineSection::Xrefs,
        CellLineSection::Variants,
    ] {
        let query = CellLineGet::accession(header.accession.clone()).with_section(section);
        let mut page = empty(QueryBody::CellLineCard(card.clone()));
        super::sections::actions(&query, card, &mut page);
        out.next.extend(
            page.next
                .into_iter()
                .filter(|(kind, _)| *kind == StepKind::Page)
                .take(4usize.saturating_sub(out.next.len())),
        );
    }
}
