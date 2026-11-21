//! FDA's table of withdrawn cancer accelerated approvals.
use crate::{
    decl::{Guidance, Probe, SourceDecl},
    fda_dsc, fda_html,
};
use biodata::Text;
use biomcp_fetch::{Endpoint, Freshness, KeyUse, Leg, RequestError, SourceSpec};
use biomcp_types::{DrugWithdrawal, Source};
use std::time::Duration;

const SOURCE: Source = Source::FdaWithdrawnApprovals;
const PATH: &str = "/drugs/cancer-approvals/withdrawn-cancer-accelerated-approvals";
const LIST: Endpoint = SPEC.get(PATH).bytes(1024 * 1024).fresh(Freshness::Days(1));

/// The FDA list and its public-domain terms.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: fda_dsc::SPEC.user_agent,
    source: SOURCE,
    base: fda_dsc::SITE,
    terms: fda_dsc::SPEC.terms,
    quota: &crate::quotas::FDA_DSC,
    key: KeyUse::None,
    deadline: Duration::from_secs(8),
    command_budget: None,
};

pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &SPEC,
    display: "FDA Withdrawn Cancer Accelerated Approvals",
    discovery_aliases: &["withdrawn oncology accelerated approvals"],
    registry_order: Some(68),
    probe: Probe::Get(PATH, &[]),
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: Some(
            "FDA web lists share a 30-second request interval as requested in FDA's robots.txt.",
        ),
    },
    key: None,
};

/// Rows from the FDA table or the shipped copy when the table cannot be read.
#[derive(Debug)]
pub struct Withdrawals {
    /// One row per FDA indication, in table order.
    pub items: Vec<DrugWithdrawal>,
    /// Why the shipped copy was used.
    pub fallback: Option<RequestError>,
}

/// Collection date of the shipped table.
pub const SEED_DATE: &str = "2026-11-21";

/// Read the current table, with a shipped copy for offline or unsuccessful reads.
pub fn read(leg: &Leg<'_>) -> Result<Withdrawals, RequestError> {
    if leg.disabled_source(SOURCE) {
        return Err(biomcp_fetch::SourceError::disabled(SOURCE).into());
    }
    let result = fda_html::read(leg, &LIST, SOURCE, false).and_then(|body| decode(&body));
    match result {
        Ok(items) => Ok(Withdrawals {
            items,
            fallback: None,
        }),
        Err(error) => Ok(Withdrawals {
            items: decode(include_str!("fda_withdrawals_seed.html"))?,
            fallback: Some(error),
        }),
    }
}

fn decode(body: &str) -> Result<Vec<DrugWithdrawal>, RequestError> {
    let malformed = || biomcp_fetch::bad_reply(SOURCE, None);
    let url = Text::new(&format!("{}{PATH}", SPEC.base)).map_err(|_| malformed())?;
    let rows = fda_html::rows(body, |[drug, indication, approval, withdrawal]| {
        drug == "Drug Name"
            && indication == "Accelerated Approval (AA) Indication"
            && approval == "Accelerated Approval Date"
            && withdrawal.starts_with("Withdrawal Date")
    })
    .filter(|rows| !rows.is_empty())
    .ok_or_else(malformed)?;
    Ok(rows
        .into_iter()
        .map(|[drug, indication, approval, withdrawal]| DrugWithdrawal {
            drug: drug.text.into(),
            indication: indication.text.into(),
            approval_date: approval.text.into(),
            withdrawal_date: withdrawal.text.into(),
            url: url.clone(),
            indication_url: fda_html::anchors(indication.html).find_map(|(href, _)| match href {
                path if path.starts_with("/drugs/") => {
                    Text::new(&format!("{}{path}", SPEC.base)).ok()
                }
                url if url.starts_with("https://") => Text::new(url).ok(),
                _ => None,
            }),
        })
        .collect())
}

/// Match the printed FDA date or its ISO year, month or full date.
pub fn matches_date(needle: &str, published: &str) -> bool {
    if needle == published {
        return true;
    }
    let Ok(query) = needle.parse::<biodata::PartialDate>() else {
        return false;
    };
    fda_dsc::html::date(published).is_some_and(|date| {
        query.year() == date.year()
            && query
                .month()
                .is_none_or(|month| Some(month) == date.month())
            && query.day().is_none_or(|day| Some(day) == date.day())
    })
}
