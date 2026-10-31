use crate::{Action, Outcome};
use biodata::{FaersReport, Text};
use biomcp_adapters::{NativePositions, faers::Positions};
use biomcp_fetch::RequestError;
use biomcp_types::{
    AdverseEventGet, AdverseEventHeader, AdverseEventSection as S, AdverseEventSource, QueryBody,
    Request, StepKind,
};
mod identifiers;
mod rows;
pub(super) fn render(
    q: &AdverseEventGet,
    r: FaersReport,
    url: Text,
    snapshot: Text,
    positions: Positions,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let q = q.with_snapshot(snapshot.clone());
    let header = AdverseEventHeader {
        id: r.safetyreportid.clone(),
        source: AdverseEventSource::Faers,
        version: r.safetyreportversion.clone(),
        serious: r.serious.clone(),
        event_type: None,
        first_received: r.receivedate.clone(),
    };
    let components = if q.section() == Some(S::Reactions) {
        biomcp_adapters::adverse_event_sources::components::labels(
            biomcp_types::Source::Faers,
            biomcp_types::Source::Meddra,
        )?
    } else {
        Box::default()
    };
    match q.section() {
        None => {
            card(r, url, header, snapshot, &positions, out)?;
            for section in [S::Drugs, S::Reactions] {
                section_step(&q, section, None, out);
            }
        }
        Some(S::Drugs) => drugs(&q, &r, &positions, header, snapshot, components, out),
        Some(S::Reactions) => {
            let total = positions
                .reactions
                .as_ref()
                .map_or(r.reactions.len() as u64, |p| u64::from(p.total));
            let records = rows::reactions(&q, &r.reactions, positions.reactions.as_ref());
            page(
                &q,
                q.section_offset()
                    .saturating_add(q.section_limit())
                    .min(total as u32),
                total,
                out,
            );
            out.body =
                QueryBody::AdverseEventReactions(Some(biomcp_types::AdverseEventReactions {
                    header,
                    snapshot,
                    reactions: biomcp_types::AdverseEventRecords {
                        records,
                        offset: q.section_offset(),
                        total,
                    },
                    components,
                }));
        }
        Some(section) => {
            let drug = NativePositions::enumerate(&r.drugs, positions.drugs.as_ref())
                .find(|(index, _)| Some(*index) == q.drug_index())
                .map(|(_, row)| row);
            identifiers::render(&q, drug, section, header, snapshot, components, out)?;
        }
    }
    Ok(())
}
fn drugs(
    q: &AdverseEventGet,
    r: &FaersReport,
    positions: &Positions,
    header: AdverseEventHeader,
    snapshot: Text,
    components: Box<[biomcp_types::ComponentLabel]>,
    out: &mut Outcome,
) {
    let total = positions
        .drugs
        .as_ref()
        .map_or(r.drugs.len() as u64, |p| u64::from(p.total));
    let records = rows::drugs(q, &r.drugs, positions.drugs.as_ref());
    page(
        q,
        q.section_offset()
            .saturating_add(q.section_limit())
            .min(total as u32),
        total,
        out,
    );
    out.body = QueryBody::AdverseEventDrugs(Some(biomcp_types::AdverseEventDrugs {
        header,
        snapshot,
        drugs: biomcp_types::AdverseEventRecords {
            records,
            offset: q.section_offset(),
            total,
        },
        components,
    }));
    let initial = out.next.len();
    for (index, drug) in
        NativePositions::enumerate(&r.drugs, positions.drugs.as_ref()).filter(|(index, _)| {
            *index >= q.section_offset()
                && *index < q.section_offset().saturating_add(q.section_limit())
        })
    {
        if let Some(ids) = &drug.openfda {
            for section in identifiers::available(ids) {
                if out.next.len() < initial + 3 {
                    section_step(q, section, Some(index), out);
                }
            }
        }
    }
}
fn section_step(q: &AdverseEventGet, s: S, index: Option<u32>, out: &mut Outcome) {
    if let Ok(q) = q.with_section(s, index) {
        out.next.push(crate::next::Step::new(
            StepKind::Section,
            &[biomcp_types::Source::Faers],
            Action::Run {
                request: Request::GetAdverseEvent(q),
                fresh: false,
            },
        ));
    }
}
fn page(q: &AdverseEventGet, offset: u32, total: u64, out: &mut Outcome) {
    if offset > q.section_offset() && u64::from(offset) < total {
        out.next.push(crate::next::Step::new(
            StepKind::Page,
            &[biomcp_types::Source::Faers],
            Action::Run {
                request: Request::GetAdverseEvent(q.at_section_offset(offset)),
                fresh: false,
            },
        ));
    }
}

fn card(
    r: FaersReport,
    url: Text,
    header: AdverseEventHeader,
    snapshot: Text,
    positions: &Positions,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    out.body = QueryBody::AdverseEventFaersCard(Some(biomcp_types::AdverseEventFaersCard {
        header,
        snapshot,
        latest_received: r.receiptdate,
        latest_received_format: r.receiptdateformat,
        transmitted: r.transmissiondate,
        transmitted_format: r.transmissiondateformat,
        first_received_format: r.receivedateformat,
        seriousness: r.seriousness,
        patient: r.patient,
        reporter: r.reporter,
        drug_count: positions
            .drugs
            .as_ref()
            .map_or(r.drugs.len() as u32, |p| p.total),
        reaction_count: positions
            .reactions
            .as_ref()
            .map_or(r.reactions.len() as u32, |p| p.total),
        source_url: Text::new("https://open.fda.gov/apis/drug/event/")
            .map_err(|_| biomcp_fetch::InputError::Serialization)?,
        report_url: url,
        available_sections: vec![S::Drugs, S::Reactions].into(),
        components: Box::default(),
    }));
    Ok(())
}
