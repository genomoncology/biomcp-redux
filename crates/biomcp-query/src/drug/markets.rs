//! One drug name looked up in each market's own source, one row per market. Each row keeps its
//! source's record, status and date role; no record or date is joined across markets.
use super::resolve::within;
use crate::{Action, Bundle, Ctx, Outcome, status};
use biodata::{AuthorizationDate, PartialDate, Text};
use biomcp_adapters::{drugsfda, who_pq};
use biomcp_types::{
    DrugGet, DrugHeader, DrugMarket, DrugMarketState, DrugMarkets, DrugMarketsGet, DrugSearch,
    DrugSection, DrugSource, DrugWhoGet, DrugWhoRow, ReplyBody, ReplyData, Request, Section,
    Source, SourceState, Status, StepKind,
};

/// How the WHO row is chosen among the products WHO lists under the name.
const WHO_CHOICE: &str = "earliest Date of Prequalification among WHO rows whose INN, dosage form and strength cell names the input; export order on a tie; undated rows last";

pub(crate) fn get(cx: &Ctx<'_>, query: &DrugMarketsGet) -> Outcome {
    let mut out = status::empty(DrugMarkets::into_reply(query.clone(), None));
    let deadline = cx.fetch.deadline(cx.budget);
    let name = query.input();
    let markets = [
        fda(&within(cx, deadline.remaining()), name, &mut out),
        ema(&within(cx, deadline.remaining()), name, &mut out),
        who(&within(cx, deadline.remaining()), name, &mut out),
    ];
    status::get(
        &mut out,
        markets.iter().map(|market| match market.state {
            DrugMarketState::Found => Status::Complete,
            DrugMarketState::NoRecord => Status::Absent,
            DrugMarketState::Failed => Status::Failed,
        }),
    );
    out.body = DrugMarkets {
        markets: markets.into(),
    }
    .with_query(query.clone());
    out
}

fn row(source: DrugSource) -> DrugMarket {
    DrugMarket {
        source,
        state: DrugMarketState::NoRecord,
        record: None,
        name: None,
        holder: None,
        status: Box::default(),
        first_approval: None,
        url: None,
        matches: None,
        choice: None,
        reason: None,
    }
}

fn fda(cx: &Ctx<'_>, name: &Text, out: &mut Outcome) -> DrugMarket {
    let mut row = row(DrugSource::Drugsfda);
    let Ok(get) = DrugGet::named(name.clone(), DrugSource::Drugsfda) else {
        return row;
    };
    let sub = super::get(cx, &get);
    absorb(out, &sub);
    let ReplyData::DrugCard {
        body: Some(card), ..
    } = &sub.body
    else {
        return unanswered(row, &sub, Source::Drugsfda);
    };
    resolution(&mut row, &card.header);
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&drugsfda::SPEC, deadline).fresh(cx.fresh);
    let result = drugsfda::get(&leg, &card.header.id);
    let error = result.as_ref().err().copied();
    let read = status::source(Source::Drugsfda, leg.finish(), error);
    command.finish(deadline.remaining());
    let Some(application) = result.ok().and_then(|page| page.records.into_iter().next()) else {
        row.state = DrugMarketState::Failed;
        row.reason = read.reason;
        out.local = out.local.or(error.and_then(status::local));
        add_source(out, read);
        return row;
    };
    let candidate = application.candidate();
    row.state = DrugMarketState::Found;
    row.record = text(&candidate.id.to_string());
    row.name = candidate.name;
    row.holder = application.sponsor().cloned();
    row.status = distinct(candidate.products.iter().filter_map(|p| p.marketing_status.clone()));
    row.first_approval = candidate
        .original_approval
        .and_then(|first| dated("submission_status_date", first.date));
    row.url = drugsfda::application_url(&candidate.id);
    if let Some(get) = crate::next::checked(
        DrugGet::new(candidate.id, DrugSource::Drugsfda)
            .and_then(|get| get.for_section(DrugSection::Approvals)),
        Source::Drugsfda,
        &mut out.problems,
    ) {
        section_step(out, Source::Drugsfda, Request::GetDrug(get), cx.fresh);
    }
    alternatives(out, &row, name, cx.fresh);
    row
}

fn ema(cx: &Ctx<'_>, name: &Text, out: &mut Outcome) -> DrugMarket {
    let mut row = row(DrugSource::Ema);
    let Ok(get) = DrugGet::named(name.clone(), DrugSource::Ema)
        .and_then(|get| get.for_section(DrugSection::Regulatory))
    else {
        return row;
    };
    let sub = super::get(cx, &get);
    absorb(out, &sub);
    let ReplyData::DrugRegulatory {
        body: Some(body), ..
    } = &sub.body
    else {
        return unanswered(row, &sub, Source::Ema);
    };
    resolution(&mut row, &body.identity);
    row.state = DrugMarketState::Found;
    row.record = text(&body.identity.id.to_string());
    row.name = body.identity.name.clone();
    row.url = body.identity.url.clone();
    if let Section::Data { data, .. } = &body.regulatory
        && let Some(event) = data.records.first()
    {
        row.holder = event.attribution.submitter.clone();
        row.status = event.status.iter().cloned().collect();
        row.first_approval = event
            .dates
            .iter()
            .find(|date| date.role.as_str() == "marketing_authorisation_date")
            .cloned();
    }
    if let Some(get) = crate::next::checked(
        DrugGet::new(body.identity.id.clone(), DrugSource::Ema)
            .and_then(|get| get.for_section(DrugSection::Regulatory)),
        Source::Ema,
        &mut out.problems,
    ) {
        section_step(out, Source::Ema, Request::GetDrug(get), cx.fresh);
    }
    alternatives(out, &row, name, cx.fresh);
    row
}

fn who(cx: &Ctx<'_>, name: &Text, out: &mut Outcome) -> DrugMarket {
    let mut row = row(DrugSource::WhoPq);
    let Ok(search) = DrugSearch::named(name.clone(), DrugSource::WhoPq, 10) else {
        return row;
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&who_pq::SPEC, deadline).fresh(cx.fresh);
    let result = who_pq::search(&leg, &search);
    let error = result.as_ref().err().copied();
    let read = status::source(Source::WhoPq, leg.finish(), error);
    command.finish(deadline.remaining());
    if matches!(read.state, SourceState::Syncing | SourceState::Stale) {
        out.next.push(crate::next::Step::new(
            StepKind::Sync,
            &[Source::WhoPq],
            Action::Sync {
                bundle: Bundle::WHO_PQ,
            },
        ));
    }
    let rows = match result {
        Ok(rows) => rows,
        Err(_) => {
            row.state = DrugMarketState::Failed;
            row.reason = read.reason;
            out.local = out.local.or(error.and_then(status::local));
            add_source(out, read);
            return row;
        }
    };
    add_source(out, read);
    row.matches = Some(rows.len() as u64);
    let Some(chosen) = earliest(&rows) else {
        return row;
    };
    row.state = DrugMarketState::Found;
    row.record = Some(chosen.reference.clone());
    row.name = text(&chosen.inn_dosage_form_strength);
    row.holder = text(&chosen.applicant);
    row.first_approval = chosen
        .prequalified_on
        .and_then(|date| dated("date_of_prequalification", date));
    row.url = who_pq::product_url(&chosen.reference);
    row.choice = Some(WHO_CHOICE.into());
    section_step(
        out,
        Source::WhoPq,
        Request::GetWhoMedicine(DrugWhoGet::new(chosen.reference.clone())),
        cx.fresh,
    );
    alternatives(out, &row, name, cx.fresh);
    row
}

/// The first row with the earliest prequalification date; undated rows only when none is dated.
fn earliest(rows: &[DrugWhoRow]) -> Option<&DrugWhoRow> {
    let key = |date: &PartialDate| (date.year(), date.month(), date.day());
    rows.iter()
        .filter(|row| row.prequalified_on.is_some())
        .min_by_key(|row| row.prequalified_on.as_ref().map(key))
        .or_else(|| rows.first())
}

/// A source the market could not read, or one that answered without a record.
fn unanswered(mut row: DrugMarket, sub: &Outcome, source: Source) -> DrugMarket {
    let failed = sub
        .sources
        .iter()
        .find(|s| matches!(s.state, SourceState::Failed | SourceState::Syncing));
    if sub.status() == Status::Failed && (failed.is_some() || sub.local.is_some()) {
        row.state = DrugMarketState::Failed;
        row.reason = failed
            .filter(|s| s.source == source)
            .or(failed)
            .and_then(|s| s.reason);
    }
    row
}

/// The name lookup's own count and ranking rule.
fn resolution(row: &mut DrugMarket, header: &DrugHeader) {
    if let Some(resolution) = &header.name_resolution {
        row.matches = Some(resolution.total);
        row.choice = Some(resolution.ranking.clone());
    }
}

/// Keep each source read, attribution and problem once, and the sync steps of a market's read.
fn absorb(out: &mut Outcome, sub: &Outcome) {
    for source in &sub.sources {
        add_source(out, source.clone());
    }
    for problem in &sub.problems {
        if !out.problems.contains(problem) {
            out.problems.push(problem.clone());
        }
    }
    out.local = out.local.or(sub.local);
    out.dropped |= sub.dropped;
    for step in sub.next.iter().filter(|step| step.0 == StepKind::Sync) {
        if !out.next.contains(step) {
            out.next.push(step.clone());
        }
    }
}

fn add_source(out: &mut Outcome, source: biomcp_types::SourceOutcome) {
    if !out
        .sources
        .iter()
        .any(|s| s.source == source.source && s.state == source.state)
    {
        out.sources.push(source);
    }
}

fn section_step(out: &mut Outcome, source: Source, request: Request, fresh: bool) {
    out.next.push(crate::next::Step::run(
        StepKind::Section,
        &[source],
        request,
        fresh,
    ));
}

/// When several records match, the call that lists them all.
fn alternatives(out: &mut Outcome, row: &DrugMarket, name: &Text, fresh: bool) {
    if row.matches.is_some_and(|n| n > 1)
        && let Ok(search) = DrugSearch::named(name.clone(), row.source, 10)
    {
        out.next.push(crate::next::Step::run(
            StepKind::Disambiguate,
            &[row.source.source()],
            Request::SearchDrug(search),
            fresh,
        ));
    }
}

fn distinct(values: impl Iterator<Item = Text>) -> Box<[Text]> {
    let mut seen = Vec::new();
    for value in values {
        if !seen.contains(&value) {
            seen.push(value);
        }
    }
    seen.into()
}

fn dated(role: &str, date: PartialDate) -> Option<AuthorizationDate> {
    Some(AuthorizationDate {
        role: role.parse().ok()?,
        date,
    })
}

fn text(value: &str) -> Option<Text> {
    Text::new(value.trim()).ok()
}
