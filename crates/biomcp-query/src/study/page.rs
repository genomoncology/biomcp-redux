use super::failure;
use crate::{Action, Ctx, Outcome, store::Store};
use biodata::StudyHeader;
use biomcp_fetch::{BundleInfo, CacheError, Deadline, RequestError};
use biomcp_types::{
    PageInfo, PoolInfo, QueryBody, Reason, Request, Status, StepKind, StudyGet, StudyPage,
    StudySearch, StudySnapshot,
};
type Retained = (Vec<StudyHeader>, StudySnapshot, u64);
pub(super) fn search(
    cx: &Ctx<'_>,
    q: &StudySearch,
    rows: Vec<StudyHeader>,
    info: BundleInfo,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let snapshot = biomcp_adapters::cbioportal::studies::snapshot(&info)?;
    let key = serde_json::to_vec(&(
        "study-catalog/1",
        q.text(),
        q.cancer_type(),
        q.limit(),
        &snapshot.revision,
    ))
    .map_err(|_| CacheError::Database)?;
    let mut store = Store::open(cx.fetch.cache_access()?, deadline)?;
    let (id, retained) = if let Some(id) = q.pool() {
        let payload = store.source_snapshot(id.as_str(), &key, |s| cx.fetch.disabled(s))?;
        let Some(payload) = payload else {
            out.next.push((
                StepKind::Restart,
                Action::Run {
                    request: Request::SearchStudy(q.with_pool(None).at_offset(0)),
                    fresh: cx.fresh,
                },
            ));
            return Err(failure(Reason::Rejected));
        };
        (
            id.clone(),
            serde_json::from_str::<Retained>(&payload).map_err(|_| CacheError::Database)?,
        )
    } else {
        let retained = (
            rows,
            snapshot,
            u64::try_from(Store::now()).map_err(|_| CacheError::Database)?,
        );
        let payload = serde_json::to_string(&retained).map_err(|_| CacheError::Database)?;
        let id = store.save_source_snapshot(&key, &payload, &[])?;
        (id, retained)
    };
    render(cx, q, id, retained, out)
}
fn render(
    cx: &Ctx<'_>,
    q: &StudySearch,
    id: biodata::Text,
    retained: Retained,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let (rows, snapshot, created) = retained;
    let query = q.with_pool(Some(id.clone()));
    let total = rows.len() as u64;
    let items: Box<[_]> = rows
        .into_iter()
        .skip(q.offset() as usize)
        .take(q.limit() as usize)
        .collect();
    let returned = items.len() as u32;
    let page = PageInfo {
        offset: q.offset(),
        returned,
        total,
        pool: Some(PoolInfo {
            id,
            size: u32::try_from(total).map_err(|_| CacheError::Database)?,
            created: created,
            expires: created.saturating_add(86400),
        }),
        truncated: None,
    };
    for row in &items {
        if let Ok(get) = serde_json::from_value::<StudyGet>(serde_json::json!({"id":row.study_id}))
        {
            out.next.push((
                StepKind::Disambiguate,
                Action::Run {
                    request: Request::GetStudy(get),
                    fresh: cx.fresh,
                },
            ));
        }
    }
    if u64::from(q.offset()) + u64::from(returned) < total {
        out.next.push((
            StepKind::Page,
            Action::Run {
                request: Request::SearchStudy(query.at_offset(q.offset() + returned)),
                fresh: cx.fresh,
            },
        ));
    }
    out.page = Some(page.clone());
    out.body = QueryBody::StudyPage(Some(StudyPage {
        items,
        page,
        snapshot,
    }));
    out.status = Status::Complete;
    Ok(())
}
