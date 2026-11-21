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
    source: Source::FdaWithdrawnApprovals,
    base: fda_dsc::SITE,
    terms: fda_dsc::SPEC.terms,
    quota: &crate::quotas::FDA_DSC,
    key: KeyUse::None,
    deadline: Duration::from_secs(8),
    command_budget: None,
};

pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &crate::fda_withdrawals::SPEC,
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
    .map_err(|_| malformed())?;
    if rows.is_empty() {
        return Err(malformed());
    }
    rows.into_iter()
        .map(|[drug, indication, approval, withdrawal]| {
            let indication_url = fda_html::anchors(indication.html)
                .next()
                .map(|(href, _)| approval_url(href))
                .transpose()?;
            Ok(DrugWithdrawal {
                drug: drug.text.into(),
                indication: indication.text.into(),
                approval_date: approval.text.into(),
                withdrawal_date: withdrawal.text.into(),
                url: url.clone(),
                indication_url,
            })
        })
        .collect()
}

fn approval_url(href: &str) -> Result<Text, RequestError> {
    let url = if href.starts_with("/drugs/") {
        Text::new(&format!("{}{href}", SPEC.base))
    } else if href.starts_with("https://") {
        Text::new(href)
    } else {
        return Err(biomcp_fetch::bad_reply(SOURCE, None));
    };
    url.map_err(|_| biomcp_fetch::bad_reply(SOURCE, None))
}

/// Match the printed FDA date or its ISO year, month or full date.
pub fn matches_date(needle: &str, published: &str) -> bool {
    if needle == published {
        return true;
    }
    needle.parse::<biodata::PartialDate>().is_ok_and(|query| {
        fda_dsc::html::date(published).is_some_and(|date| {
            query.year() == date.year()
                && query
                    .month()
                    .is_none_or(|month| Some(month) == date.month())
                && query.day().is_none_or(|day| Some(day) == date.day())
        })
    })
}
