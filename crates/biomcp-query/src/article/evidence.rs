//! Independent citation graph and explicit JATS reference evidence with stored continuations.
mod graph;
mod page;
mod text;
use crate::{Action, Ctx, Outcome, status, store::Store};
use biodata::{
    ArticleEvidence, ArticleEvidenceResult, CitationContexts, CitationParagraphs, QueryBody,
    Request, Section, Source, SourceOutcome, Status, StepKind,
};
use biomcp_fetch::{CacheError, Deadline, RequestError};
use sha2::{Digest, Sha256};
pub(super) struct Snapshot {
    graph: Section<CitationContexts>,
    target: Option<graph::Target>,
    fulltext: Option<Section<CitationParagraphs>>,
    terminal: bool,
    cursor: Option<u32>,
    matched: bool,
    sources: Vec<SourceOutcome>,
}
impl Snapshot {
    fn empty() -> Self {
        Self {
            graph: Section::Empty {
                source: Source::SemanticScholar,
                record: None,
            },
            target: None,
            fulltext: None,
            terminal: false,
            cursor: None,
            matched: false,
            sources: Vec::new(),
        }
    }
}
/// Keep every successful evidence part beside any independent source failure.
pub fn run(cx: &Ctx<'_>, query: &ArticleEvidence) -> Outcome {
    let mut out = super::empty();
    out.body = QueryBody::ArticleEvidence(ArticleEvidenceResult {
        semantic_scholar: Section::Empty {
            source: Source::SemanticScholar,
            record: None,
        },
        fulltext: query.fulltext().then_some(Section::NotApplicable),
    });
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    if let Err(error) = execute(cx, query, &mut deadline, &mut out) {
        out.local = status::local(error);
        out.status = Status::Failed;
    }
    command.finish(deadline.remaining());
    if out
        .sources
        .iter()
        .any(|s| s.reason == Some(biodata::Reason::RateLimited))
    {
        out.next
            .insert(0, (StepKind::Key, Action::KeyHelp(Source::SemanticScholar)));
    }
    if matches!(out.status, Status::Failed | Status::Partial) && out.next.is_empty() {
        out.next.push((
            StepKind::Fresh,
            Action::Run {
                request: Request::ArticleEvidence(query.identity()),
                fresh: true,
            },
        ));
    }
    out
}
fn execute<'a>(
    cx: &Ctx<'a>,
    query: &ArticleEvidence,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    // Disabled source content is never restored from a previous continuation snapshot.
    let graph_disabled = cx.fetch.disabled(Source::SemanticScholar);
    let text_disabled = cx.fetch.disabled(Source::PmcOa);
    let mut store = Store::open(cx.fetch.cache_access()?, *deadline)?;
    let identity = serde_json::to_vec(&query.identity()).map_err(|_| CacheError::Database)?;
    let key = Sha256::digest([b"biomcp.article_evidence/1\0".as_slice(), &identity].concat());
    let mut restored = false;
    let mut snapshot = if let Some(id) = query.pool() {
        match store.evidence(id.as_str(), &key, |source| cx.fetch.disabled(source))? {
            crate::store::EvidenceState::Missing => {
                out.body = QueryBody::PoolError { mismatch: false };
                restart(query, out);
                return Ok(());
            }
            crate::store::EvidenceState::Disabled => Snapshot::empty(),
            crate::store::EvidenceState::Ready(payload) => {
                restored = true;
                decode(&payload)?
            }
        }
    } else {
        Snapshot::empty()
    };
    if graph_disabled {
        snapshot.graph = Section::unavailable(Source::SemanticScholar, biodata::Reason::Disabled);
        snapshot.terminal = true;
        snapshot
            .sources
            .retain(|s| s.source != Source::SemanticScholar);
    } else if !snapshot.terminal {
        graph::read(cx, query, &mut snapshot, deadline, out)?;
    }
    if query.fulltext() {
        if text_disabled {
            snapshot.fulltext = Some(Section::unavailable(
                Source::PmcOa,
                biodata::Reason::Disabled,
            ));
            snapshot.sources.retain(|s| s.source != Source::PmcOa);
        } else if snapshot.fulltext.is_none() {
            text::read(cx, query, &mut snapshot, deadline, out)?;
        }
    }
    store.set_deadline(*deadline);
    let payload=encode(&snapshot)?;
    let id = store.save_evidence(
        query.pool().filter(|_| restored),
        &key,
        &payload,
        &snapshot.sources,
    )?;
    page::render(query, snapshot, id, out)?;
    Ok(())
}
pub(super) fn source(snapshot: &mut Snapshot, incoming: SourceOutcome) {
    if let Some(original) = snapshot
        .sources
        .iter_mut()
        .find(|s| s.source == incoming.source)
    {
        status::retain_source_state(original, &incoming);
    } else {
        snapshot.sources.push(incoming)
    }
}
pub(super) fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::SemanticScholar,
        reason: biodata::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}

fn restart(query: &ArticleEvidence, out: &mut Outcome) {
    out.next.push((
        StepKind::Restart,
        Action::Run {
            request: Request::ArticleEvidence(query.identity()),
            fresh: false,
        },
    ));
}

// Private versioned store layout uses existing checked domain values. The query
// crate does not define a second public wire model or depend on serialization derives.
type Saved = (
    Section<CitationContexts>, Option<Section<CitationParagraphs>>,
    (bool,Option<u32>,bool), Vec<SourceOutcome>,
    Option<(Option<biodata::Pmid>,Option<biodata::Doi>)>,
);
fn encode(snapshot:&Snapshot)->Result<String,CacheError> {
    serde_json::to_string(&(
        &snapshot.graph,&snapshot.fulltext,
        (snapshot.terminal,snapshot.cursor,snapshot.matched),&snapshot.sources,
        snapshot.target.as_ref().map(|target|(target.pmid,target.doi.as_ref())),
    )).map_err(|_|CacheError::Database)
}
fn decode(payload:&str)->Result<Snapshot,CacheError> {
    let (graph,fulltext,(terminal,cursor,matched),sources,target):Saved=serde_json::from_str(payload).map_err(|_|CacheError::Database)?;
    Ok(Snapshot {graph,fulltext,terminal,cursor,matched,sources,target:target.map(|(pmid,doi)|graph::Target{pmid,doi})})
}
