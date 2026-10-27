//! Remove follow-ups whose article or author source is disabled.
use biomcp_query::{Action, Outcome};
use biomcp_types::{ArticleInput, ArticleSectionWord as Section, Request, Source};

pub(super) fn filter(fetch: &biomcp_fetch::Fetch, result: &mut Outcome) {
    let mut removed = Vec::new();
    result.next.retain(|(kind, action)| {
        let disabled = sources(action)
            .into_iter()
            .filter(|source| fetch.disabled(*source))
            .collect::<Vec<_>>();
        if disabled.is_empty() {
            return true;
        }
        removed.extend(disabled.into_iter().map(|source| (*kind, source)));
        false
    });
    for (kind, source) in removed {
        if !result.next.iter().any(|(kept, _)| *kept == kind)
            && !result
                .sources
                .iter()
                .any(|outcome| outcome.source == source)
        {
            result.sources.push(biomcp_adapters::registry::outcome(
                source,
                biomcp_types::SourceState::Skipped,
                Some(biomcp_types::Reason::Disabled),
            ));
        }
    }
}

fn sources(action: &Action) -> Vec<Source> {
    let request = match action {
        Action::Run { request, .. } => request,
        Action::AuthorPapers(query) => return vec![author(query.id())],
        _ => return Vec::new(),
    };
    match request {
        Request::ArticleRecommend(_) | Request::ArticlePdf(_) => {
            vec![Source::SemanticScholar]
        }
        Request::ArticleEvidence(query) => evidence(query),
        Request::ArticleCitations(query) | Request::ArticleReferences(query) => {
            vec![match query.source() {
                biomcp_types::EdgeSource::Icite => Source::Icite,
                biomcp_types::EdgeSource::SemanticScholar => Source::SemanticScholar,
            }]
        }
        Request::GetArticle(query) => article(query),
        Request::SearchAuthor(_) => vec![Source::SemanticScholar],
        Request::GetAuthor(query) => query.ids().iter().map(author).collect(),
        Request::AuthorPapers(query) => vec![author(query.id())],
        Request::ArticleSupplement(query) => text(query.input()),
        Request::ArticleText(query) => text(query.input()),
        Request::SearchArticle(query) => query
            .sources()
            .iter()
            .map(|source| match source {
                biomcp_types::ArticleSource::Pubmed => Source::PubMed,
                biomcp_types::ArticleSource::Europepmc => Source::EuropePmc,
                biomcp_types::ArticleSource::Litvar => Source::LitVar,
                biomcp_types::ArticleSource::SemanticScholar => Source::SemanticScholar,
                biomcp_types::ArticleSource::Litsense => Source::LitSense,
                biomcp_types::ArticleSource::Pubtator => Source::PubTator,
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn text(input: &ArticleInput) -> Vec<Source> {
    match input {
        ArticleInput::EuropePmc(_) => vec![Source::EuropePmc],
        ArticleInput::Pmid(_) => vec![Source::PubMed, Source::PmcOa],
        _ => vec![Source::PmcOa],
    }
}

fn evidence(query: &biomcp_types::ArticleEvidence) -> Vec<Source> {
    let mut sources = vec![Source::SemanticScholar];
    if !matches!(query.cited(), ArticleInput::Doi(_))
        || query.fulltext() && !matches!(query.citing(), ArticleInput::Pmcid(_))
    {
        sources.push(Source::PubMed);
    }
    if query.fulltext() {
        sources.push(Source::PmcOa);
    }
    sources
}

fn article(query: &biomcp_types::ArticleGet) -> Vec<Source> {
    let mut sources = query
        .input()
        .iter()
        .map(|input| match input {
            ArticleInput::EuropePmc(_) => Source::EuropePmc,
            _ => Source::PubMed,
        })
        .collect::<Vec<_>>();
    for section in query.sections() {
        match section {
            Section::Annotations => sources.push(Source::PubTator),
            Section::Citations => sources.push(Source::Icite),
            Section::Tldr | Section::Pdf => sources.push(Source::SemanticScholar),
            Section::Supplements => sources.push(Source::PmcOa),
            Section::Fulltext => sources.extend(query.input().iter().map(|input| match input {
                ArticleInput::EuropePmc(_) => Source::EuropePmc,
                _ => Source::PmcOa,
            })),
            Section::All => sources.extend([
                Source::PubTator,
                Source::Icite,
                Source::SemanticScholar,
                Source::PmcOa,
            ]),
            _ => {}
        }
    }
    sources
}

fn author(id: &biodata::AuthorId) -> Source {
    match id {
        biodata::AuthorId::SemanticScholar(_) => Source::SemanticScholar,
        biodata::AuthorId::Orcid(_) => Source::Orcid,
    }
}
