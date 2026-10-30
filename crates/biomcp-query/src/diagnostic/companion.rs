use crate::{Action, Bundle, Ctx, Outcome, status};
use biodata::CompanionDiagnosticRelation;
use biomcp_adapters::fda_cdx;
use biomcp_types::{
    DiagnosticCompanion, DiagnosticSearch, DrugGet, DrugSource, PageInfo, QueryBody, Reason,
    Request, Source, SourceState, Status, StepKind,
};
use std::collections::HashSet;

pub(super) fn search(cx: &Ctx<'_>, q: &DiagnosticSearch) -> Outcome {
    let mut out = super::empty(QueryBody::DiagnosticCompanion(DiagnosticCompanion {
        items: Box::default(),
        count_unit: None,
        distinct_id_total: None,
    }));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&fda_cdx::SPEC, deadline);
    let result = fda_cdx::search(&leg, q);
    let error = result.as_ref().err().copied();
    let mut group_unsupported = false;
    if let Ok(page) = result {
        let q = page
            .info
            .content_revision
            .as_deref()
            .and_then(|v| v.parse().ok())
            .map_or_else(|| q.clone(), |v| q.with_bundle_revision(v));
        pivots(&mut out, &page.items, cx.fresh);
        let returned = page.items.len() as u32;
        if u64::from(q.offset()) + u64::from(returned) < page.total {
            super::next::push(
                &mut out,
                StepKind::Page,
                Request::SearchDiagnostic(q.at_offset(q.offset() + returned)),
                cx.fresh,
            );
        }
        {
            let unavailable = !page.problems.is_empty();
            crate::status::item(&mut out, true, false, unavailable);
        };
        group_unsupported = page.group_unsupported;
        out.problems = page.problems;
        out.page = Some(PageInfo {
            offset: q.offset(),
            returned,
            total: page.total,
            pool: None,
            truncated: None,
        });
        out.body = QueryBody::DiagnosticCompanion(DiagnosticCompanion {
            items: page.items,
            count_unit: None,
            distinct_id_total: None,
        });
    }
    out.local = error.and_then(status::local);
    let mut source = status::source(Source::FdaCdx, leg.finish(), error);
    if group_unsupported {
        source.reason = Some(Reason::Unsupported);
    }
    if matches!(source.state, SourceState::Stale) && out.status() == Status::Complete {
        crate::status::item(&mut out, true, false, true);
    }
    if matches!(source.state, SourceState::Syncing | SourceState::Stale) {
        out.next.push((
            StepKind::Sync,
            Action::Sync {
                bundle: Bundle::FdaCdx,
            },
        ));
    }
    out.sources.push(source);
    command.finish(deadline.remaining());
    out
}
fn pivots(out: &mut Outcome, rows: &[CompanionDiagnosticRelation], fresh: bool) {
    let mut seen = HashSet::new();
    for row in rows {
        let (devices, applications) = match row {
            CompanionDiagnosticRelation::Individual {
                device_ids,
                drug_applications,
                ..
            }
            | CompanionDiagnosticRelation::Group {
                device_ids,
                drug_applications,
                ..
            } => (device_ids, drug_applications),
        };
        for device in devices {
            if !(device.as_str().starts_with('P') || device.as_str().starts_with('K'))
                || !seen.insert(device.as_str().to_owned())
            {
                continue;
            }
            let mut parts = device.as_str().split('/');
            if let Some(base) = parts.next() {
                let mut fields = serde_json::json!({"id":base,"source":if base.starts_with('K') {"fda-510k"}else{"fda-pma"}});
                if let Some(supplement) = parts.next() {
                    fields
                        .as_object_mut()
                        .map(|v| v.insert("supplement".into(), serde_json::json!(supplement)));
                }
                if let Ok(get) = serde_json::from_value(fields) {
                    super::next::push(out, StepKind::Pivot, Request::GetDiagnostic(get), fresh);
                }
            }
        }
        for application in applications {
            if seen.insert(application.id.as_str().to_owned())
                && let Ok(id) = application.id.as_str().parse()
                && let Ok(get) = DrugGet::new(id, DrugSource::Drugsfda)
            {
                super::next::push(out, StepKind::Pivot, Request::GetDrug(get), fresh);
            }
        }
    }
}
