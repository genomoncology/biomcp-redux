use crate::{Action, Ctx, Outcome, gene, status};

use biomcp_adapters::hgnc;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{ArticleSearch, ArticleType, Request, Source, Status, StepKind};
use biomcp_types::{GeneInput, GeneResolution};
pub(super) struct Resolved {
    pub term: String,
    pub symbol: Option<String>,
    pub entrez: Option<biodata::NcbiGeneId>,
}
pub(super) fn resolve<'a>(
    cx: &Ctx<'a>,
    query: &ArticleSearch,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<Option<Resolved>, RequestError> {
    let words = query.words();
    let Some(input) = query.gene() else {
        return Ok(Some(Resolved {
            term: filters(query, words),
            symbol: None,
            entrez: None,
        }));
    };
    let leg = cx.fetch.leg(&hgnc::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let resolved = gene::resolve(&leg, input);
    *deadline = super::consume(*deadline, started, leg.remaining());
    out.sources.push(status::source(
        Source::Hgnc,
        leg.finish(),
        resolved.as_ref().err().copied(),
    ));
    let mapped = resolved?;
    status::append(&mut out.problems, mapped.problems);
    match mapped.value {
        GeneResolution::Resolved { gene, .. } => {
            let mut names = vec![gene.symbol.to_string()];
            let typed = match input {
                GeneInput::Text(text) => Some(text.as_str()),
                _ => None,
            };
            for name in typed.into_iter().map(str::to_owned) {
                if !names.iter().any(|n| n.eq_ignore_ascii_case(&name)) {
                    names.push(name);
                }
            }
            let group = if names.len() > 1 {
                format!("({})", names.join(" OR "))
            } else {
                names.join("")
            };
            let symbol = gene.symbol.to_string();
            Ok(Some(Resolved {
                term: filters(
                    query,
                    if words.is_empty() {
                        group
                    } else {
                        format!("{group} AND {words}")
                    },
                ),
                symbol: Some(symbol),
                entrez: gene.entrez_id,
            }))
        }
        GeneResolution::Ambiguous { candidates } => {
            for candidate in candidates.iter().take(5) {
                let checked = query.with_gene(GeneInput::HgncId(candidate.hgnc_id));
                out.next.push((
                    StepKind::Disambiguate,
                    Action::Run {
                        request: Request::SearchArticle(checked),
                        fresh: cx.fresh,
                    },
                ));
            }
            Ok(None)
        }
        _ => {
            crate::status::item(out, false, false, false);
            Ok(None)
        }
    }
}

fn filters(query: &ArticleSearch, words: String) -> String {
    let mut parts: Vec<String> = (!words.is_empty()).then_some(words).into_iter().collect();
    if let Some(author) = query.author() {
        parts.push(format!("{author}[au]"));
    }
    if let Some(journal) = query.journal() {
        parts.push(format!("{journal}[ta]"));
    }
    let (since, until) = query.dates();
    if since.is_some() || until.is_some() {
        let date = |value: Option<biodata::PartialDate>, default: &str| {
            value.map_or_else(|| default.to_owned(), |v| v.to_string().replace('-', "/"))
        };
        parts.push(format!(
            "{}:{}[dp]",
            date(since, "1800"),
            date(until, "3000")
        ));
    }
    if let Some(kind) = query.kind() {
        parts.push(
            match kind {
                ArticleType::Trial => "\"clinical trial\"[pt]",
                ArticleType::Randomized => "\"randomized controlled trial\"[pt]",
                ArticleType::Review => "review[pt]",
                ArticleType::SystematicReview => "\"systematic review\"[pt]",
                ArticleType::MetaAnalysis => "\"meta-analysis\"[pt]",
                ArticleType::Guideline => "(guideline[pt] OR \"practice guideline\"[pt])",
                ArticleType::CaseReport => "\"case reports\"[pt]",
            }
            .to_owned(),
        );
    }
    let mut term = parts.join(" AND ");
    if query.open_access() {
        term.push_str(" AND \"pubmed pmc open access\"[filter]");
    }
    if query.no_preprints() {
        term.push_str(" NOT preprint[pt]");
    }
    if query.exclude_retracted() {
        term.push_str(" NOT retracted publication[pt]");
    }
    term
}
