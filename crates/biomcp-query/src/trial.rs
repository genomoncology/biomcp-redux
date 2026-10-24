//! Trial discovery delegates filters, totals and ranking to the named registry.
use crate::{Action, Ctx, Outcome, status};
use biodata::{
    GeneResolution, QueryBody, Request, Source, SourceState, Status, StepKind, TrialPage,
    TrialSearch,
};
use biomcp_adapters::{ctgov, hgnc};
/// Execute a checked native search without locally screening returned studies.
pub(crate) fn search(cx: &Ctx<'_>, query: &TrialSearch) -> Outcome {
    let mut outcome = Outcome {
        status: Status::Failed,
        body: QueryBody::TrialPage(TrialPage {
            items: Box::default(),
            request: Default::default(),
            source_updated: None,
            attribution: "ClinicalTrials.gov; source-native discovery, no eligibility assessment"
                .into(),
            gene_resolution: None,
        }),
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    };
    let Some(gene) = gene(cx, query, &mut outcome) else {
        return outcome;
    };
    let leg = cx
        .fetch
        .leg(&ctgov::SPEC, cx.fetch.deadline(cx.budget))
        .fresh(cx.fresh);
    let params = ctgov::parameters(query, gene.as_deref());
    let result = ctgov::search(&leg, &params);
    let version = match &result {
        Ok(_) => ctgov::version(&leg),
        Err(error) => Err(*error),
    };
    let error = result.as_ref().err().or(version.as_ref().err()).copied();
    let mut source = status::source(Source::ClinicalTrialsGov, leg.finish(), error);
    outcome.status = if result.is_err() {
        Status::Failed
    } else if version.is_err() || source.state == SourceState::Stale {
        Status::Partial
    } else {
        Status::Complete
    };
    if let Ok(mut page) = result {
        source.count = Some(page.total);
        source.fetched = Some(page.items.len() as u32);
        if query.count_only() {
            page.items.clear();
        }
        outcome.page = Some(biodata::PageInfo {
            offset: 0,
            returned: page.items.len() as u32,
            total: page.total,
            pool: None,
            truncated: None,
        });
        if let Some(token) = page.next.filter(|_| !query.count_only()) {
            outcome.next.push((
                StepKind::Page,
                Action::Run {
                    request: Request::SearchTrial(query.at_page(token)),
                    fresh: cx.fresh,
                },
            ));
        }
        if let QueryBody::TrialPage(body) = &mut outcome.body {
            body.items = page.items.into();
        }
    }
    if let QueryBody::TrialPage(body) = &mut outcome.body {
        body.request = params;
        body.source_updated = version.ok();
    }
    outcome.local = error.and_then(status::local);
    outcome.sources.push(source);
    outcome
}
fn gene(cx: &Ctx<'_>, query: &TrialSearch, outcome: &mut Outcome) -> Option<Option<String>> {
    let Some(input) = query.gene() else {
        return Some(None);
    };
    let leg = cx
        .fetch
        .leg(&hgnc::SPEC, cx.fetch.deadline(cx.budget))
        .fresh(cx.fresh);
    let result = crate::gene::resolve(&leg, input);
    let error = result.as_ref().err().copied();
    outcome
        .sources
        .push(status::source(Source::Hgnc, leg.finish(), error));
    match result {
        Ok(mapped) => {
            outcome.problems.extend(mapped.problems);
            match mapped.value {
                GeneResolution::Resolved { gene, .. } => Some(Some(gene.symbol.to_string())),
                resolution => {
                    outcome.status = Status::Partial;
                    if let QueryBody::TrialPage(body) = &mut outcome.body {
                        body.gene_resolution = Some(resolution);
                    }
                    None
                }
            }
        }
        Err(error) => {
            outcome.local = status::local(error);
            None
        }
    }
}
