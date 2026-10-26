use super::failure;
use crate::{Action, Ctx, Outcome, store::Store};
use biodata::StudyHeader;
use biomcp_fetch::{BundleInfo, CacheError, Deadline, RequestError};
use biomcp_types::{
    PageInfo, QueryBody, Reason, Request, Status, StepKind, StudyGet, StudyPage, StudySearch,
    StudySnapshot,
};
type Retained = (Vec<StudyHeader>, StudySnapshot);
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
    let existing = store.find(&key, q.pool().map(|p| p.as_str()))?;
    let pool = if let Some(pool) = existing.filter(|pool| pool.key == key) {
        pool
    } else if q.pool().is_some() {
        out.next.push((
            StepKind::Restart,
            Action::Run {
                request: Request::SearchStudy(q.with_pool(None).at_offset(0)),
                fresh: cx.fresh,
            },
        ));
        return Err(failure(Reason::Rejected));
    } else {
        let size = u32::try_from(rows.len()).map_err(|_| CacheError::Database)?;
        let payload = serde_json::to_string(&(rows, snapshot)).map_err(|_| CacheError::Database)?;
        store.publish_source_pool(&key, &payload, size, &[])?
    };
    let payload = store
        .source_snapshot(pool.info.id.as_str(), &key, |s| cx.fetch.disabled(s))?
        .ok_or_else(|| failure(Reason::Rejected))?;
    let retained = serde_json::from_str::<Retained>(&payload).map_err(|_| CacheError::Database)?;
    render(cx, q, pool.info, retained, out)
}
fn render(
    cx: &Ctx<'_>,
    q: &StudySearch,
    pool: biomcp_types::PoolInfo,
    retained: Retained,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let (rows, snapshot) = retained;
    let query = q.with_pool(Some(pool.id.clone()));
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
        pool: Some(pool),
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
