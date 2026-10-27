//! Recover an empty LitVar selection without inferring another variant identity.
use crate::{Action, Outcome};
use biomcp_types::{ArticleSearch, ArticleSource, Request, SourceNotice, StepKind};

pub(super) fn recover(query: &ArticleSearch, out: &mut Outcome) {
    for source in [ArticleSource::Europepmc, ArticleSource::Pubmed] {
        out.next.push((
            StepKind::Disambiguate,
            Action::Run {
                request: Request::SearchArticle(query.with_source(source)),
                fresh: false,
            },
        ));
    }
    if let Some(source) = out.sources.last_mut() {
        source.notices.push(SourceNotice::LitvarRsidHint {
            message:
                "If you have an rsID, look it up in LitVar with --variant RSID --source litvar."
                    .into(),
        });
    }
}
