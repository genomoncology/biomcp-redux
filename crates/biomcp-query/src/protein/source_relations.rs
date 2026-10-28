//! Complete checked source collections page without admitting inferred memberships.
use crate::{Action, Ctx, Outcome, status, store::Store};
use biodata::{Text, UniProtIsoform};
use biomcp_adapters::{complex_portal, interpro};
use biomcp_fetch::{CacheError, Deadline, RequestError, SourceError};
use biomcp_types::{
    PageInfo, ProteinComplexMembersQuery, ProteinDomainsQuery, ProteinRelationQuery, QueryBody,
    Request, Source, Status, StepKind,
};
use sha2::{Digest, Sha256};
mod saved;
use saved::Saved;
#[derive(Clone)]
pub(super) enum Selection {
    Domains(ProteinDomainsQuery),
    Complexes(ProteinRelationQuery),
    Members(ProteinComplexMembersQuery),
}
impl Selection {
    fn accession(&self) -> &UniProtIsoform {
        match self {
            Self::Domains(q) => q.accession(),
            Self::Complexes(q) => q.accession(),
            Self::Members(q) => q.accession(),
        }
    }
    fn window(&self) -> (u32, u32, Option<&Text>) {
        match self {
            Self::Domains(q) => (q.limit(), q.offset(), q.pool()),
            Self::Complexes(q) => (q.limit(), q.offset(), q.pool()),
            Self::Members(q) => (q.limit(), q.offset(), q.pool()),
        }
    }
    fn source(&self) -> Source {
        match self {
            Self::Domains(_) => Source::InterPro,
            _ => Source::ComplexPortal,
        }
    }
    fn spec(&self) -> &'static biomcp_fetch::SourceSpec {
        match self {
            Self::Domains(_) => &interpro::SPEC,
            _ => &complex_portal::SPEC,
        }
    }
    fn request(&self, page: Option<(Text, u32)>) -> Request {
        match self {
            Self::Domains(q) => {
                Request::ProteinDomains(page.map_or_else(|| q.restart(), |(p, o)| q.at_page(p, o)))
            }
            Self::Complexes(q) => Request::ProteinComplexes(
                page.map_or_else(|| q.restart(), |(p, o)| q.at_page(p, o)),
            ),
            Self::Members(q) => Request::ProteinComplexMembers(
                page.map_or_else(|| q.restart(), |(p, o)| q.at_page(p, o)),
            ),
        }
    }
    fn key(&self) -> Result<Vec<u8>, CacheError> {
        let mut h = Sha256::new();
        h.update(b"biomcp.protein_source_relations/2\0");
        h.update(serde_json::to_vec(&self.request(None)).map_err(|_| CacheError::Database)?);
        Ok(h.finalize().to_vec())
    }
    pub(super) fn empty(&self) -> QueryBody {
        match self {
            Self::Domains(_) => QueryBody::ProteinDomains(None),
            Self::Complexes(_) => QueryBody::ProteinComplexes(None),
            Self::Members(_) => QueryBody::ProteinComplexMembers(None),
        }
    }
}
pub(super) fn run(cx: &Ctx<'_>, q: Selection) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut out = super::empty(q.empty());
    if let Err(e) = read(cx, &q, deadline, &mut out) {
        out.local = status::local(e);
        out.next.push((
            StepKind::Fresh,
            Action::Run {
                request: q.request(None),
                fresh: true,
            },
        ));
    }
    command.finish(deadline.remaining());
    out
}
fn restart(q: &Selection, out: &mut Outcome, mismatch: bool) {
    out.body = QueryBody::PoolError { mismatch };
    out.next.push((
        StepKind::Restart,
        Action::Run {
            request: q.request(None),
            fresh: false,
        },
    ));
}
pub(super) fn read(
    cx: &Ctx<'_>,
    q: &Selection,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let leg = cx.fetch.leg(q.spec(), deadline).fresh(cx.fresh);
    if cx.fetch.disabled(q.source()) {
        let e = SourceError {
            source: q.source(),
            reason: biomcp_types::Reason::Disabled,
            status: None,
            retry_after: None,
        }
        .into();
        out.sources
            .push(status::source(q.source(), leg.finish(), Some(e)));
        return Err(e);
    }
    let (limit, offset, selected) = q.window();
    if selected.is_none() && offset != 0 || selected.is_some() && cx.fresh {
        restart(q, out, false);
        return Ok(());
    }
    let mut store = Store::open(cx.fetch.cache_access()?, deadline)?;
    let Some((saved, pool)) = load(cx, q, leg, &mut store, out)? else {
        return Ok(());
    };
    if let (Selection::Members(query), Saved::Members(None)) = (q, &saved) {
        out.problems.push(biomcp_types::Problem {
            source: q.source(),
            record: Text::new(query.complex().as_str()).ok(),
            field: format!("participants.anchor.{}", query.accession()).into(),
            code: biomcp_types::ProblemCode::ValueUnknown,
        });
        return Ok(());
    }
    let total = saved.total();
    if offset as u64 > total {
        restart(q, out, false);
        return Ok(());
    }
    let returned = limit.min(total.saturating_sub(offset.into()) as u32);
    saved.member_steps(q, offset, limit, &mut store, out)?;
    saved.render(q, offset, limit, out)?;
    out.status = if !out.problems.is_empty()
        || out
            .sources
            .iter()
            .any(|s| s.state != biomcp_types::SourceState::Ok)
    {
        Status::Partial
    } else {
        Status::Complete
    };
    out.page = Some(PageInfo {
        offset,
        returned,
        total,
        pool: Some(pool.clone()),
        truncated: None,
    });
    if u64::from(offset + returned) < total {
        out.next.push((
            StepKind::Page,
            Action::Run {
                request: q.request(Some((pool.id, offset + returned))),
                fresh: false,
            },
        ));
    }
    Ok(())
}

fn load(
    cx: &Ctx<'_>,
    q: &Selection,
    leg: biomcp_fetch::Leg<'_>,
    store: &mut Store<'_>,
    out: &mut Outcome,
) -> Result<Option<(Saved, biomcp_types::PoolInfo)>, RequestError> {
    let (_, _, selected) = q.window();
    let key = q.key()?;
    let pool = if cx.fresh {
        None
    } else {
        store.find(&key, selected.map(Text::as_str))?
    };
    let loaded = if let Some(pool) = pool {
        if pool.key != key {
            restart(q, out, true);
            return Ok(None);
        }
        let Some(payload) =
            store.source_snapshot(pool.info.id.as_str(), &key, |s| cx.fetch.disabled(s))?
        else {
            restart(q, out, false);
            return Ok(None);
        };
        out.sources = pool.sources;
        (Saved::decode(q, &payload)?, pool.info)
    } else {
        if selected.is_some() {
            restart(q, out, false);
            return Ok(None);
        }
        let result = Saved::fetch(q, &leg);
        let error = result.as_ref().err().copied();
        out.sources
            .push(status::source(q.source(), leg.finish(), error));
        let saved = match result {
            Ok(saved) => saved,
            Err(error) => {
                super::reply_problem(out, q.source(), q.accession(), error);
                return Err(error);
            }
        };
        let size = saved.total() as u32;
        let pool = store.publish_source_pool(&key, &saved.encode()?, size, &out.sources)?;
        (saved, pool.info)
    };
    Ok(Some(loaded))
}
