use super::next;
use crate::{Action, Bundle, Ctx, Outcome, status};
use biodata::UniProtIsoform;
use biomcp_adapters::{hgnc, uniprot};
use biomcp_fetch::{Find, KeyKind, Leg, RequestError};
use biomcp_types::ReplyBody;
use biomcp_types::{
    ErrorMessage, ProteinCard, ProteinCardItem, ProteinGet, ProteinInput, ReplyData, Request,
    Section, Source, SourceNotice, SourceState, Status, StepKind, UsageError,
};
/// ChEMBL target counts declare a longer budget than the default; other legs keep their caps.
fn budget(cx: &Ctx<'_>, q: &ProteinGet) -> std::time::Duration {
    if super::sections::wants(q, biomcp_types::ProteinSection::Chembl) {
        cx.fetch
            .source_budget(&biomcp_adapters::chembl::TARGET_COUNTS, cx.budget)
    } else {
        cx.budget
    }
}
/// Exact accessions resolve independently and remain in caller order.
pub(crate) fn get(cx: &Ctx<'_>, q: &ProteinGet) -> Outcome {
    let command = cx.fetch.command_deadline(budget(cx, q));
    let deadline = command.deadline();
    let hgnc_leg = cx.fetch.leg(&hgnc::SPEC, deadline).fresh(cx.fresh);
    let leg = cx.fetch.leg(&uniprot::SPEC, deadline).fresh(cx.fresh);
    let mut out = super::empty(
        (ProteinCard {
            items: Box::default(),
            coordinate_note: None,
        })
        .with_query(q.clone()),
    );
    let mut hgnc_error = None;
    let mut protein = ProteinReads::default();
    let mut items = Vec::new();
    let mut unread = Vec::new();
    for input in q.input() {
        let mut item = empty_item(input.clone());
        match ids(&hgnc_leg, &leg, input, &mut out, &mut protein) {
            Ok(ids) => {
                protein.asked |= !ids.is_empty();
                unread.extend(hydrate(&leg, ids, q, &mut item, &mut out, &mut protein));
            }
            Err((source, e)) => {
                if source == Source::UniProt {
                    protein.error = Some(e);
                } else {
                    hgnc_error = Some(e);
                }
                out.local = out.local.or(status::local(e));
            }
        }
        items.push(item);
    }
    if q.input().iter().any(|v| matches!(v, ProteinInput::Gene(_))) {
        out.sources
            .push(status::source(Source::Hgnc, hgnc_leg.finish(), hgnc_error));
    }
    if protein.asked {
        let mut source = status::source(Source::UniProt, leg.finish(), protein.error);
        source.notices.extend(protein.notices);
        out.sources.push(source);
    }
    if let Some(scope) = protein.scope
        && matches!(q.input(), [ProteinInput::Accession(_)])
    {
        out.body = ReplyData::Usage(UsageError::message(scope));
        command.finish(deadline.remaining());
        return out;
    }
    if out
        .sources
        .iter()
        .any(|s| s.state == biomcp_types::SourceState::Stale)
    {
        for item in &mut items {
            if item.status == Status::Complete {
                item.status = crate::status::item_status(true, false, true);
            }
        }
    }
    super::source_sections::supplement(cx, q, &mut items, &mut out, deadline);
    status::get(&mut out, items.iter().map(|i| i.status));
    if out.next.is_empty() {
        out.next = next::cards(&items, &mut out.problems);
    }
    let steps = next::unread(cx, q, &unread, &mut out.problems);
    out.next.extend(steps);
    recovery(q, &mut out);
    let spans = items
        .iter()
        .filter_map(|item| item.sections.as_ref())
        .any(|sections| {
            matches!(&sections.features, Some(Section::Data { data, .. }) if !data.items.is_empty())
                || matches!(&sections.domains, Some(Section::Data { data, .. }) if super::source_relations::spans(&data.items))
        });
    out.body = (ProteinCard {
        items: items.into(),
        coordinate_note: biomcp_types::coordinate_note(spans),
    })
    .with_query(q.clone());
    command.finish(deadline.remaining());
    out
}
fn empty_item(input: ProteinInput) -> ProteinCardItem {
    ProteinCardItem {
        input,
        status: crate::status::item_status(false, true, false),
        header: None,
        candidates: Box::default(),
        audit: None,
        sections: None,
        more: Box::default(),
    }
}
#[derive(Default)]
struct ProteinReads {
    asked: bool,
    error: Option<RequestError>,
    notices: Vec<SourceNotice>,
    scope: Option<ErrorMessage>,
}

impl ProteinReads {
    fn notice(&mut self, message: String) {
        let notice = SourceNotice::RequestGuidance {
            data_date: None,
            message: message.into(),
            retry_after_seconds: None,
        };
        if !self.notices.contains(&notice) {
            self.notices.push(notice);
        }
    }
}

fn ids(
    hgnc_leg: &Leg<'_>,
    leg: &Leg<'_>,
    input: &ProteinInput,
    out: &mut Outcome,
    protein: &mut ProteinReads,
) -> Result<Vec<UniProtIsoform>, (Source, RequestError)> {
    let ProteinInput::Gene(symbol) = input else {
        return Ok(match input {
            ProteinInput::Accession(id) => vec![id.clone()],
            _ => Vec::new(),
        });
    };
    let rows = hgnc::lookup(hgnc_leg, Find::Exact(KeyKind::Symbol, symbol.as_str()))
        .map_err(|error| (Source::Hgnc, error))?;
    status::append(&mut out.problems, rows.problems);
    let mut ids = Vec::new();
    for gene in rows
        .value
        .into_iter()
        .filter(|g| g.symbol.as_str() == symbol.as_str())
    {
        let links = if crate::gene::needs_uniprot_lookup(&gene) {
            protein.asked = true;
            let mapped = uniprot::gene_candidates(leg, &gene.symbol)
                .map_err(|error| (Source::UniProt, error))?;
            status::append(&mut out.problems, mapped.problems);
            mapped.value.into_boxed_slice()
        } else {
            gene.uniprot
        };
        for id in links {
            if let Some(id) = UniProtIsoform::new(id.as_str())
                && !ids.contains(&id)
            {
                ids.push(id);
            }
        }
    }
    Ok(ids)
}
/// Return the accessions whose entries failed; their other sources can still answer.
fn hydrate(
    leg: &Leg<'_>,
    ids: Vec<UniProtIsoform>,
    q: &ProteinGet,
    item: &mut ProteinCardItem,
    out: &mut Outcome,
    protein: &mut ProteinReads,
) -> Vec<UniProtIsoform> {
    let ambiguous = ids.len() > 1;
    let mut missed = Vec::new();
    item.status = crate::status::item_status(false, false, false);
    let mut failed = false;
    for id in ids {
        match uniprot::lookup(leg, &id) {
            Ok(uniprot::Lookup::Outside(reason)) => {
                let (message, reason) = match reason {
                    uniprot::Scope::NonHuman => (ErrorMessage::ProteinNonHuman, "is non-human"),
                    uniprot::Scope::Unreviewed => {
                        (ErrorMessage::ProteinUnreviewed, "is unreviewed")
                    }
                    uniprot::Scope::Deleted => (ErrorMessage::ProteinDeleted, "was deleted"),
                };
                protein.scope = Some(message);
                protein.notice(format!(
                    "{id} {reason}. BioMCP covers active reviewed human proteins only."
                ));
            }
            Ok(uniprot::Lookup::Card(mapped)) => {
                failed |= mapped.refusals == biomcp_adapters::Refusal::Refused;
                if mapped.value.is_none() && mapped.refusals == biomcp_adapters::Refusal::Refused {
                    protein.error = Some(
                        biomcp_fetch::SourceError {
                            source: Source::UniProt,
                            reason: biomcp_types::Reason::BadReply,
                            status: Some(200),
                            tls_cause: None,
                            retry_after: None,
                        }
                        .into(),
                    );
                }
                status::append(&mut out.problems, mapped.problems);
                if let Some(entry) = mapped.value {
                    let primary = &entry.header.protein.accession;
                    if primary != &id {
                        protein.notice(format!(
                            "UniProt redirected secondary accession {id} to {primary}."
                        ));
                    }
                    item.status = crate::status::item_status(true, false, false);
                    if ambiguous {
                        let mut rows = item.candidates.to_vec();
                        rows.push(entry.header);
                        item.candidates = rows.into();
                    } else {
                        item.more = super::sections::NAMES
                            .into_iter()
                            .filter(|name| !super::sections::wants(q, *name))
                            .collect();
                        if !q.sections().is_empty() {
                            let mut sections = super::sections::first(q, &entry);
                            super::sections::continuations(q, &entry, out);
                            if let Some(e) = super::sections::alternative_sequence(
                                q,
                                &entry,
                                leg,
                                &mut sections,
                                out,
                            ) {
                                failed = true;
                                protein.error = Some(e);
                                out.local = out.local.or(status::local(e));
                            }
                            item.sections = Some(sections);
                        }
                        item.header = Some(entry.header);
                        item.audit = Some(entry.audit);
                    }
                }
            }
            Err(e) => {
                super::reply_problem(out, Source::UniProt, &id, e);
                failed = true;
                if !q.sections().is_empty() {
                    item.sections = Some(super::sections::failed(
                        q,
                        crate::status::reason(e).unwrap_or(biomcp_types::Reason::Unavailable),
                    ));
                }
                missed.push(id);
                protein.error = Some(e);
                out.local = out.local.or(status::local(e));
            }
        }
    }
    if failed {
        item.status = if item.header.is_some() || !item.candidates.is_empty() {
            crate::status::item_status(true, false, true)
        } else {
            crate::status::item_status(false, true, false)
        };
    }
    missed
}

fn recovery(q: &ProteinGet, out: &mut Outcome) {
    let fresh = out
        .sources
        .iter()
        .any(|s| matches!(s.state, SourceState::Failed | SourceState::Stale));
    let sync = out.sources.iter().any(|s| {
        s.source == Source::Hgnc
            && matches!(
                s.state,
                SourceState::Syncing | SourceState::Failed | SourceState::Stale
            )
    });
    out.next
        .truncate(5 - usize::from(fresh) - usize::from(sync));
    if sync {
        out.next.push(crate::next::Step::new(
            StepKind::Sync,
            &[biomcp_types::Source::Hgnc],
            Action::Sync {
                bundle: Bundle::HGNC,
            },
        ));
    }
    if fresh {
        out.next.push(crate::next::Step::run(
            StepKind::Fresh,
            &[biomcp_types::Source::UniProt],
            Request::GetProtein(q.clone()),
            true,
        ));
    }
}
