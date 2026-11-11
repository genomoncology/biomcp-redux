//! Canonical EMA source declaration.
mod map;
mod raw;
mod referrals;
use biodata::DrugId;
use biomcp_fetch::{
    Bundle, Fetch, Find, Freshness, KeyKind, KeyUse, Leg, RequestError, RowFormat, SourceSpec,
    Terms,
};
use biomcp_types::Source;
use biomcp_types::{DrugCard, DrugHeader, DrugRecords, DrugRegulatory, Section};
use std::marker::PhantomData;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::Ema,
    base: "https://www.ema.europa.eu",
    terms: Terms {
        name: crate::drug_sources::EMA_TERMS.name,
        url: crate::drug_sources::EMA_TERMS.url,
        usage: crate::drug_sources::EMA_TERMS.usage,
    },
    quota: &crate::quotas::EMA,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};

const MEDICINES: Bundle<raw::Medicine> = Bundle {
    spec: &SPEC,
    name: "ema",
    path: "/en/documents/report/medicines-output-medicines_json-report_en.json",
    format: RowFormat::Json {
        rows: "data",
        metadata: "meta",
    },
    fresh: Freshness::Days(1),
    row: PhantomData,
};
/// Explicit complete-file synchronization of the medicines and referrals files through the
/// shared atomic bundle store. The reply keeps the medicines file's date and sums both files.
pub fn sync(fetch: &Fetch) -> Result<crate::Mapped<biomcp_types::SyncResult>, RequestError> {
    let mut sync = crate::mapped::bundle_sync(SPEC.source, fetch.sync(&MEDICINES)?);
    let referrals = crate::mapped::bundle_sync(SPEC.source, fetch.sync(&referrals::REFERRALS)?);
    sync.refusals.include(referrals.refusals);
    sync.problems.extend(referrals.problems);
    sync.value.rows += referrals.value.rows;
    sync.value.bytes += referrals.value.bytes;
    sync.value.downloaded_bytes += referrals.value.downloaded_bytes;
    Ok(sync)
}
/// Literal medicine-name candidates in native bundle order; never synchronize on reads.
pub fn search(leg: &Leg<'_>, query: &str) -> Result<crate::Mapped<Vec<DrugHeader>>, RequestError> {
    let (rows, _) = leg.stored_rows(&MEDICINES, Find::Words(query))?;
    Ok(name_headers(&rows, query, false))
}
/// Resolve names from the existing local rows, including snapshots whose older word index omitted generic fields.
/// Native ID lookup remains indexed; reads never synchronize or modify the stored snapshot.
pub fn search_names(
    leg: &Leg<'_>,
    query: &str,
) -> Result<crate::Mapped<Vec<DrugHeader>>, RequestError> {
    let (rows, _) = leg.stored_rows(&MEDICINES, Find::Prefix(KeyKind::Id, ""))?;
    Ok(name_headers(&rows, query, true))
}
fn name_headers(
    rows: &[raw::Medicine],
    query: &str,
    only_matches: bool,
) -> crate::Mapped<Vec<DrugHeader>> {
    let mut problems = fields(None);
    let value = rows
        .iter()
        .filter_map(|row| {
            let exact = |field| {
                row.field(field)
                    .and_then(Result::ok)
                    .is_some_and(|n| biomcp_types::DrugNameMatch::same_name(n, query))
            };
            let fields = [
                "active_substance",
                "international_non_proprietary_name_common_name",
            ];
            let generic_exact = fields.iter().any(|field| exact(field));
            let generic = fields.iter().any(|field| {
                row.field(field)
                    .and_then(Result::ok)
                    .is_some_and(|value| biomcp_types::DrugNameMatch::generic_name(value, query))
            });
            let matched = biomcp_types::DrugNameMatch::from_fields(
                exact("name_of_medicine") && !generic_exact,
                generic,
            );
            if only_matches && matched.is_none() {
                return None;
            }
            problems.record = Some(row.record.clone());
            let mut header = map::header(row, &mut problems);
            header.matched_name = matched;
            Some(header)
        })
        .collect();
    crate::Mapped {
        refusals: crate::Refusal::None,
        value,
        problems: problems.list,
    }
}
fn exact(leg: &Leg<'_>, id: &DrugId) -> Result<Option<raw::Medicine>, RequestError> {
    let (mut rows, _) = leg.stored_rows(&MEDICINES, Find::Exact(KeyKind::Id, &id.to_string()))?;
    // SQLite indexes fold keys; exact get still requires the published identifier itself.
    Ok(rows.pop().filter(|row| row.id == *id))
}
/// Exact native EMA identity card without authorization events.
pub fn card(leg: &Leg<'_>, id: &DrugId) -> Result<crate::Mapped<Option<DrugCard>>, RequestError> {
    let mut problems = fields(Some(id));
    let value = exact(leg, id)?
        .as_ref()
        .map(|row| map::card(row, &mut problems));
    Ok(crate::Mapped {
        refusals: crate::Refusal::None,
        value,
        problems: problems.list,
    })
}
/// Source-native dates and status for only the selected EMA application record, with the
/// Commission decision date from the referrals file when its latest procedure is a referral.
pub fn regulatory(
    leg: &Leg<'_>,
    id: &DrugId,
) -> Result<crate::Mapped<Option<DrugRegulatory>>, RequestError> {
    let mut problems = fields(Some(id));
    let row = exact(leg, id)?;
    let procedure = row.as_ref().and_then(|row| {
        row.field("latest_procedure_affecting_product_information")
            .and_then(Result::ok)
    });
    let decision = referrals::decision(leg, procedure, &mut problems)?;
    let value = row.map(|row| {
        let mut event = map::event(&row, &mut problems);
        let mut date_roles = map::date_roles(&row, &mut problems, &event).into_vec();
        if let Some(decision) = decision {
            let mut dates = std::mem::take(&mut event.dates).into_vec();
            dates.push(decision.date);
            event.dates = dates.into();
            date_roles.push(decision.label);
        }
        DrugRegulatory {
            identity: map::header(&row, &mut problems),
            date_roles: date_roles.into(),
            regulatory: Section::Data {
                source: Source::Ema,
                data: DrugRecords {
                    records: vec![event].into(),
                    offset: 0,
                    total: Some(1),
                    truncated: None,
                    version: problems.field("_bundle_meta.timestamp", row.meta.timestamp),
                    retrieved: None,
                },
            },
        }
    });
    Ok(crate::Mapped {
        refusals: crate::Refusal::None,
        value,
        problems: problems.list,
    })
}
/// Where an EMA regulatory reply's European Commission decision date came from, or, when it has
/// none, why the date EMA's medicines file names `european_commission_decision_date` is not it.
pub fn decision_date_notice(body: &DrugRegulatory) -> Option<Box<str>> {
    let role = |name: &str| {
        body.date_roles
            .iter()
            .find(|label| label.role.as_str() == name)
    };
    if let Some(label) = role(referrals::DECISION_ROLE) {
        let record = label
            .procedure
            .as_ref()
            .map_or("", |record| record.as_str());
        return Some(
            format!(
                "{} is from EMA's referrals file, record {record}.",
                referrals::DECISION_ROLE
            )
            .into(),
        );
    }
    role(map::LATEST_PROCEDURE_ROLE).map(|_| DECISION_DATE_NOTICE.into())
}
const DECISION_DATE_NOTICE: &str = "No European Commission decision date is reported. \
latest_procedure_affecting_product_information_date is EMA's european_commission_decision_date \
field, which EMA's product page shows as the latest procedure's date; it can differ from the \
Commission decision. Read the decision date on the procedure's own EMA page, such as its referral \
page.";
fn fields(id: Option<&DrugId>) -> crate::mapped::Problems {
    crate::mapped::Problems {
        source: Source::Ema,
        record: id.and_then(|id| crate::mapped::text(&id.to_string()).ok()),
        list: Vec::new(),
    }
}

/// Inspect this source's validated local catalog without reading rows or synchronizing.
pub(crate) fn bundle_state(leg: &Leg<'_>) -> Result<biomcp_types::LocalBundleState, RequestError> {
    Ok(leg
        .stored_bundle_state(&MEDICINES)?
        .with(leg.stored_bundle_state(&referrals::REFERRALS)?))
}

/// Complete source policy and command declaration.
pub(crate) mod decl;
