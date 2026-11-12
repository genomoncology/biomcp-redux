//! Resolve one typed drug name to the names its sources state, for trial and article search.
use super::name_fit::capped;
pub(crate) use super::name_fit::{Fit, any, openfda};
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    DrugGet, DrugNameMatch, DrugSource, Reason, ReplyData, SourceNotice, SourceOutcome,
    SourceState, SourceStatus,
};

/// MyChem rows read per page for one name. The trial-name lookup reads one page of its scoped
/// phrase query; a name get pages free text until a page holds an exact name.
pub(crate) const NAME_ROWS: u32 = 50;
/// Lookups may spend one part in this many of what is left, so the search always keeps the rest.
const LOOKUP_SHARE: u32 = 3;
/// Most names sent to a provider that takes one request per name, under its request pacing.
const NAME_REQUESTS: usize = 4;

/// What one lookup settled about a typed name.
#[derive(Debug, Default)]
pub(crate) struct Resolved {
    /// Names to search, most wanted first; one name means the term stays as typed.
    pub names: Vec<Text>,
    /// A lookup failed and none gave names, so the search may be missing names.
    pub failed: bool,
    /// Why the term stayed as typed, as a short phrase, when a lookup did not settle it.
    pub unsettled: Option<&'static str>,
    /// Each stated name left out, as `name (reason)`, and counts past a limit.
    pub left_out: Vec<String>,
}

enum Found {
    Names {
        names: Vec<Text>,
        preferred: Vec<String>,
        official: Vec<String>,
        record: String,
        evidence: Vec<SourceOutcome>,
        problems: Vec<biomcp_types::Problem>,
    },
    Ambiguous(Vec<SourceOutcome>),
    Failed(Vec<SourceOutcome>),
    Nothing,
    /// MyChem holds no row for the name at all.
    Unknown,
}

/// Run lookups under a bounded share of the command, charging only what they spent.
pub(crate) fn bounded<'a, T>(
    deadline: &mut Deadline<'a>,
    run: impl FnOnce(&mut Deadline<'a>) -> T,
) -> T {
    let mut lookup = deadline.child(deadline.remaining() / LOOKUP_SHARE);
    let started = lookup.remaining();
    let value = run(&mut lookup);
    *deadline = deadline.after_stage(started.saturating_sub(lookup.remaining()));
    value
}

/// Walk the sources until one states the drug's names. A source with several structures, or
/// nothing more to give, passes to the next; a failed source is never read as an empty one.
/// ChEMBL comes before the label because a molecule record carries more names than one product.
/// ChEMBL's records are read from MyChem's copy of ChEMBL, in one request of a fraction of a
/// second, where ChEMBL's own API takes two searches and a molecule read of seconds each, and
/// stalls for whole lookups when it is unwell. They are not read for a name MyChem has no row for.
/// `fit` says which names the searching provider can take, and why it cannot take the others.
/// A lookup that does not finish never counts against the search: its sources are listed as
/// skipped, and the notice names the term searched as typed.
pub(crate) fn resolve<'a>(
    cx: &Ctx<'a>,
    input: &Text,
    out: &mut Outcome,
    deadline: &mut Deadline<'a>,
    fit: Fit,
) -> Resolved {
    let mut ambiguous = None;
    let mut failed = Vec::new();
    let mut unknown = false;
    for source in [
        DrugSource::Mychem,
        DrugSource::Chembl,
        DrugSource::OpenfdaLabel,
    ] {
        if cx.fetch.disabled(source.source())
            || (source == DrugSource::Chembl
                && (unknown || cx.fetch.disabled(DrugSource::Mychem.source())))
        {
            continue;
        }
        if deadline.remaining().is_zero() {
            failed.push(biomcp_adapters::registry::outcome(
                source.source(),
                SourceStatus::Failed(Some(Reason::DeadlineExceeded)),
            ));
            continue;
        }
        let stage = cx.fetch.deadline(deadline.remaining());
        let started = stage.remaining();
        let lookup = super::resolve::within(cx, deadline.remaining());
        let found = if source == DrugSource::Mychem {
            mychem(&lookup, input)
        } else if source == DrugSource::Chembl {
            chembl(&lookup, input)
        } else {
            card(&lookup, source, input, out)
        };
        *deadline = deadline.after_stage(started.saturating_sub(stage.remaining()));
        match found {
            Found::Names {
                names,
                preferred,
                official,
                record,
                evidence,
                problems,
            } => {
                let ranked = (preferred.as_slice(), official.as_slice());
                let kept = capped(names, ranked, input, cx.drug_name_limit, fit);
                let message = kept.message(source.source(), &record);
                let mut listed = kept.names.clone();
                listed.sort_by_key(lowered);
                record_evidence(out, evidence, input, &listed, message);
                out.problems.extend(problems);
                return Resolved {
                    left_out: kept.left_out(),
                    names: kept.names,
                    failed: false,
                    unsettled: None,
                };
            }
            Found::Ambiguous(evidence) => ambiguous = ambiguous.or(Some(evidence)),
            Found::Failed(evidence) => failed.extend(evidence),
            Found::Nothing => {}
            Found::Unknown => unknown = true,
        }
    }
    unexpanded(out, input, (failed, ambiguous))
}

/// Say why the term stayed as typed, when a source failed or stated several structures.
fn unexpanded(
    out: &mut Outcome,
    input: &Text,
    (mut failed, ambiguous): (Vec<SourceOutcome>, Option<Vec<SourceOutcome>>),
) -> Resolved {
    let typed = vec![input.clone()];
    if !failed.is_empty() {
        for source in &mut failed {
            if source.state == SourceState::Failed {
                source.set_status(SourceStatus::Skipped(source.reason));
            }
        }
        let message = "Drug-name lookups did not finish, so names were not expanded; \
                       searched only the typed term. Records under the drug's other names \
                       may be missing; repeat the search to retry the lookup."
            .to_owned();
        record_evidence(out, failed, input, &typed, message);
        return Resolved {
            names: Vec::new(),
            failed: true,
            unsettled: Some("name lookup unfinished"),
            left_out: Vec::new(),
        };
    }
    if let Some(evidence) = ambiguous {
        let message = "The name matches more than one drug structure; \
                       searched the typed term without expansion."
            .to_owned();
        record_evidence(out, evidence, input, &typed, message);
        return Resolved {
            names: typed,
            failed: false,
            unsettled: Some("name fits several drugs"),
            left_out: Vec::new(),
        };
    }
    Resolved::default()
}

/// Read the rows stating the name. Product rows and stereo forms are not separate drugs, and
/// product rows give no names when the drug has rows of its own.
fn mychem(cx: &Ctx<'_>, input: &Text) -> Found {
    let command = cx.fetch.command_deadline(cx.budget);
    let leg = cx
        .fetch
        .leg(&biomcp_adapters::mychem::SPEC, command.deadline())
        .fresh(cx.fresh);
    let result = biomcp_adapters::mychem::name_records(&leg, input.as_str(), NAME_ROWS);
    let error = result.as_ref().err().copied();
    let remaining = leg.remaining();
    let parent = status::source(DrugSource::Mychem.source(), leg.finish(), error);
    command.finish(remaining);
    let mapped = match result {
        Ok(mapped) => mapped,
        Err(_) => return failure(error, vec![parent]),
    };
    let carried = super::components(cx, &parent, false);
    let evidence: Vec<_> = std::iter::once(parent).chain(carried).collect();
    if mapped.refusals == biomcp_adapters::Refusal::Refused
        || u64::from(mapped.value.received) != mapped.value.total
    {
        return Found::Failed(evidence);
    }
    let structures: std::collections::BTreeSet<_> = mapped
        .value
        .records
        .iter()
        .filter_map(|row| row.structure.as_deref())
        .collect();
    if structures.len() > 1 {
        return Found::Ambiguous(evidence);
    }
    if mapped.value.total == 0 {
        return Found::Unknown;
    }
    let Some(record) = mapped
        .value
        .records
        .iter()
        .find(|row| row.structure.is_some())
    else {
        return Found::Nothing;
    };
    let record = record.id.clone();
    // A product row with no structure of its own never describes this drug.
    let drug_rows = || {
        mapped
            .value
            .records
            .iter()
            .filter(|row| row.structure.is_some())
    };
    let names = drug_rows()
        .flat_map(|row| row.names.iter().cloned())
        .collect();
    let preferred = drug_rows()
        .flat_map(|row| row.preferred.iter().cloned())
        .collect();
    let official = drug_rows()
        .flat_map(|row| row.official.iter().cloned())
        .collect();
    with_other_names(
        (names, preferred, official),
        input,
        record,
        evidence,
        mapped.problems,
    )
}

/// Read the ChEMBL molecule stating the name from MyChem's copy of ChEMBL. Like ChEMBL's own
/// exact-name filters, a name two molecules state gives no names.
fn chembl(cx: &Ctx<'_>, input: &Text) -> Found {
    let command = cx.fetch.command_deadline(cx.budget);
    let leg = cx
        .fetch
        .leg(&biomcp_adapters::mychem::SPEC, command.deadline())
        .fresh(cx.fresh);
    let result = biomcp_adapters::mychem::chembl_names(&leg, input.as_str(), NAME_ROWS);
    let error = result.as_ref().err().copied();
    let remaining = leg.remaining();
    let parent = status::source(DrugSource::Mychem.source(), leg.finish(), error);
    command.finish(remaining);
    let copied = biomcp_adapters::registry::outcome(DrugSource::Chembl.source(), parent.status());
    let evidence = vec![parent, copied];
    let mapped = match result {
        Ok(mapped) => mapped,
        Err(_) => return failure(error, evidence),
    };
    if mapped.refusals == biomcp_adapters::Refusal::Refused
        || u64::from(mapped.value.received) != mapped.value.total
    {
        return Found::Failed(evidence);
    }
    let mut molecules = mapped.value.molecules;
    if molecules.len() > 1 {
        return Found::Ambiguous(evidence);
    }
    let Some(molecule) = molecules.pop() else {
        return Found::Nothing;
    };
    with_other_names(
        (molecule.names, molecule.preferred, molecule.official),
        input,
        molecule.id,
        evidence,
        mapped.problems,
    )
}

/// Read one exact label record through the get drug path; label rows are products of one drug.
fn card(cx: &Ctx<'_>, source: DrugSource, input: &Text, out: &mut Outcome) -> Found {
    let Some(query) = crate::next::checked(
        DrugGet::named(input.clone(), source),
        source.source(),
        &mut out.problems,
    ) else {
        return Found::Nothing;
    };
    let found = super::resolve::get(cx, &query);
    if found
        .sources
        .iter()
        .any(|s| s.source == source.source() && s.state != SourceState::Ok)
    {
        let error = found.local;
        return failure(error, found.sources);
    }
    if found.dropped {
        return Found::Failed(found.sources);
    }
    let ReplyData::DrugCard {
        body: Some(card), ..
    } = &found.body
    else {
        return Found::Nothing;
    };
    let names = super::aliases::names(card);
    if !names
        .iter()
        .any(|name| DrugNameMatch::same_identity(name.as_str(), input.as_str()))
    {
        return Found::Nothing;
    }
    let preferred = names
        .first()
        .map(|name| name.to_string())
        .into_iter()
        .collect();
    let names = names.into_iter().cloned().collect();
    let official = card.official_names.iter().map(Text::to_string).collect();
    let record = card.header.id.to_string();
    with_other_names(
        (names, preferred, official),
        input,
        record,
        found.sources,
        found.problems,
    )
}

fn with_other_names(
    (names, preferred, official): (Vec<Text>, Vec<String>, Vec<String>),
    input: &Text,
    record: String,
    evidence: Vec<SourceOutcome>,
    problems: Vec<biomcp_types::Problem>,
) -> Found {
    if names
        .iter()
        .all(|name| DrugNameMatch::same_name(name.as_str(), input.as_str()))
    {
        return Found::Nothing;
    }
    Found::Names {
        names,
        preferred,
        official,
        record,
        evidence,
        problems,
    }
}

/// A lookup with no recording says nothing about its source; only replay reaches this.
fn failure(error: Option<RequestError>, evidence: Vec<SourceOutcome>) -> Found {
    if error.and_then(status::reason) == Some(Reason::ReplayMiss)
        || evidence
            .iter()
            .any(|s| s.reason == Some(Reason::ReplayMiss))
    {
        return Found::Nothing;
    }
    Found::Failed(evidence)
}

fn lowered(name: &Text) -> String {
    name.as_str().to_lowercase()
}

fn record_evidence(
    out: &mut Outcome,
    evidence: Vec<SourceOutcome>,
    input: &Text,
    names: &[Text],
    message: String,
) {
    let mut noted = false;
    for mut source in evidence {
        if !noted {
            noted = true;
            source.notices.push(SourceNotice::DrugNameSearch {
                input: input.clone(),
                names: names.to_vec().into(),
                message: message.clone().into(),
            });
        }
        if let Some(existing) = out.sources.iter_mut().find(|s| s.source == source.source) {
            status::retain_source_state(existing, &source);
            existing.notices.extend(source.notices);
        } else {
            out.sources.push(source);
        }
    }
}

/// One compact disclosure for a search that widened its names: how the names were searched, why
/// the term stayed as typed, then each stated name left out with its short reason.
pub(crate) fn compact(
    searched: &str,
    (widened, rejected): (bool, bool),
    unsettled: &[String],
    left_out: &[String],
) -> String {
    let mut parts: Vec<String> = Vec::new();
    if rejected {
        parts.push("provider rejected the other names; typed only".into());
    } else if widened {
        parts.push(searched.into());
    } else if unsettled.is_empty() {
        parts.push("no other names".into());
    }
    if !rejected {
        parts.extend(unsettled.iter().map(|note| format!("{note}; typed only")));
        parts.extend(left_out.iter().cloned());
    }
    parts.join("; ")
}

/// Drop the lookups' name notices from their sources, so the searched source can carry the one
/// notice that says what was searched.
pub(crate) fn drop_notices(sources: &mut [SourceOutcome]) {
    for source in sources {
        source
            .notices
            .retain(|notice| !matches!(notice, SourceNotice::DrugNameSearch { .. }));
    }
}

/// Keep disabled article providers and exact terms outside drug-name resolution.
pub(crate) fn article<'a>(
    cx: &Ctx<'a>,
    query: &biomcp_types::ArticleSearch,
    out: &mut Outcome,
    deadline: &mut Deadline<'a>,
) -> Resolved {
    if query.exact_term()
        || crate::article::next::search_sources(query)
            .iter()
            .all(|source| cx.fetch.disabled(*source))
    {
        return Resolved::default();
    }
    query
        .drug_name_input()
        .map_or_else(Resolved::default, |input| {
            bounded(deadline, |lookup| resolve(cx, input, out, lookup, any))
        })
}

/// Whether the provider refused the request itself, as an oversized name group can make it do.
pub(crate) fn rejected(error: Option<&RequestError>) -> bool {
    matches!(
        error,
        Some(RequestError::Source(error)) if matches!(error.status, Some(400 | 413 | 414))
    )
}

/// Whether the searched provider refused the request itself, read from its reply.
pub(crate) fn refused(out: &Outcome, source: biomcp_types::Source) -> bool {
    out.sources.iter().any(|s| {
        s.source == source && s.state == SourceState::Failed && s.reason == Some(Reason::Rejected)
    })
}

/// The notice for a provider that searched only the typed term after refusing the names.
pub(crate) fn typed_only(input: &Text, source: biomcp_types::Source) -> SourceNotice {
    SourceNotice::DrugNameSearch {
        input: input.clone(),
        names: vec![input.clone()].into(),
        message: format!("{source} rejected the expanded request; searched the typed term.").into(),
    }
}

/// Stop claiming names no provider searched.
pub(crate) fn retract(out: &mut Outcome) {
    retract_sources(&mut out.sources);
}

/// Stop claiming names no provider searched, in name-lookup evidence kept apart from a reply.
pub(crate) fn retract_sources(sources: &mut [SourceOutcome]) {
    for notice in sources.iter_mut().flat_map(|s| &mut s.notices) {
        if let SourceNotice::DrugNameSearch {
            input,
            names,
            message,
        } = notice
        {
            *names = vec![input.clone()].into();
            *message =
                "The provider rejected the expanded request; searched the typed term.".into();
        }
    }
}

/// Providers taking one request per name get the typed name, then the others in `names` order.
pub(crate) fn one_request_each(input: Option<&Text>, names: &[Text]) -> Vec<Text> {
    let mut picked: Vec<_> = names.iter().collect();
    picked.sort_by_key(|name| Some(*name) != input);
    picked.truncate(NAME_REQUESTS);
    // Request order sets result order, so it must not depend on which name was typed.
    picked.sort_by_key(|name| name.as_str().to_lowercase());
    picked.into_iter().cloned().collect()
}

/// Source names are literal terms inside a native Boolean group.
pub(crate) fn alternatives(names: &[Text], input: &Text) -> Text {
    let mut names = names.to_vec();
    // A provider's reply is cached by its request, so the order must not depend on the typed name.
    names.sort_by_key(lowered);
    let terms: Vec<_> = names
        .iter()
        .map(|name| {
            format!(
                "\"{}\"",
                name.as_str().replace('\\', "\\\\").replace('"', "\\\"")
            )
        })
        .collect();
    Text::new(&format!("({})", terms.join(" OR "))).unwrap_or_else(|_| input.clone())
}
