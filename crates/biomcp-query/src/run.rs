use crate::{Ctx, gene};
use biodata::{PageInfo, Problem, QueryBody, Request, SourceOutcome, Status, StepKind};
use biomcp_fetch::RequestError;
/// Housekeeping selector independent of an entity query.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bundle {
    /// The HGNC identity source, including approved and withdrawn records.
    Hgnc,
    /// ClinGen validity and dosage files.
    ClinGen,
}
/// Typed action before an interface spells command arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    /// Probe source reachability and report publisher terms.
    Health,
    /// Run a query, retaining fresh policy.
    Run {
        #[doc = "Checked query."]
        request: Request,
        #[doc = "Bypass cache reads."]
        fresh: bool,
    },
    /// Explain configuration names for a keyed source.
    KeyHelp(biodata::Source),
    /// Synchronize one declared file.
    Sync {
        #[doc = "Bundle selector."]
        bundle: Bundle,
    },
}
/// Query result before the interface adds its typed envelope and next spelling.
#[derive(Debug)]
pub struct Outcome {
    /// Overall outcome.
    pub status: Status,
    /// Typed response body.
    pub body: QueryBody,
    /// Search counts.
    pub page: Option<PageInfo>,
    /// Sources actually asked.
    pub sources: Vec<SourceOutcome>,
    /// Rejected fields.
    pub problems: Vec<Problem>,
    /// Typed next actions.
    pub next: Vec<(StepKind, Action)>,
    /// Local errors remain distinct even alongside useful answers.
    pub local: Option<RequestError>,
}
/// Run one checked request through the shared query layer.
pub fn run(cx: &Ctx<'_>, request: &Request) -> Outcome {
    match request {
        Request::GetVariant(query) => crate::variant::get(cx, query),
        Request::ArticleEvidence(query) => crate::article::evidence::run(cx, query),
        Request::ArticleRecommend(query) => crate::article::recommend::run(cx, query),
        Request::ArticleSupplement(query) => crate::article::supplement::read(cx, query),
        Request::ArticleText(query) => crate::article::text::read(cx, query),
        Request::ArticleCitations(query) => crate::article::edges(cx, query, false),
        Request::ArticleReferences(query) => crate::article::edges(cx, query, true),
        Request::GetArticle(query) => crate::article::get::get(cx, query),
        Request::SearchArticle(query) => crate::article::search(cx, query),
        Request::GetGene(query) => gene::get::get(cx, query),
        Request::SearchGene(query) => gene::search::search(cx, query),
    }
}
