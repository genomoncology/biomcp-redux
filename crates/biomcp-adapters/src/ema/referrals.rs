//! EMA's referrals file, read only to date the European Commission decision on a referral.
//!
//! EMA's "Download website data in JSON data format" page publishes it as the "Referrals JSON
//! data file", beside the medicines file and under the same legal notice. That page names its
//! `european_commission_decision_date` key "European Commission decision date".
use super::raw;
use biodata::{AuthorizationDate, Text};
use biomcp_fetch::{Bundle, Find, Freshness, KeyKind, Leg, RequestError, RowFormat};
use biomcp_types::{DateRoleLabel, LocalBundleState};
use std::marker::PhantomData;

pub(super) const REFERRALS: Bundle<raw::Referral> = Bundle {
    spec: &super::SPEC,
    name: "ema-referrals",
    path: "/en/documents/report/referrals-output-json-report_en.json",
    format: RowFormat::Json {
        rows: "data",
        metadata: "meta",
    },
    fresh: Freshness::Days(1),
    row: PhantomData,
};
/// The referrals file's key for the Commission decision, kept as the reported role.
pub(super) const DECISION_ROLE: &str = "european_commission_decision_date";
/// A referral's Commission decision date and the referral reference it came from.
pub(super) struct Decision {
    pub date: AuthorizationDate,
    pub label: DateRoleLabel,
}
/// The Commission decision on the referral that is a product's latest procedure.
///
/// The medicines file gives a referral as `REF/0000287477`; the referrals file gives the same
/// procedure as `EMA/REF/0000287477`. Any other procedure, a referrals file not yet synchronized,
/// or a reference that is not exactly one referral row yields no decision.
pub(super) fn decision(
    leg: &Leg<'_>,
    procedure: Option<&str>,
    problems: &mut crate::mapped::Problems,
) -> Result<Option<Decision>, RequestError> {
    let Some(procedure) = procedure.filter(|procedure| procedure.starts_with("REF/")) else {
        return Ok(None);
    };
    if matches!(
        leg.stored_bundle_state(&REFERRALS)?,
        LocalBundleState::Missing
    ) {
        return Ok(None);
    }
    let reference = format!("EMA/{procedure}");
    let (rows, _) = leg.stored_rows(&REFERRALS, Find::Exact(KeyKind::Xref, &reference))?;
    // SQLite indexes fold keys; the join still requires the published reference itself.
    let mut rows = rows.into_iter().filter(|row| row.reference == reference);
    let (Some(row), None) = (rows.next(), rows.next()) else {
        return Ok(None);
    };
    let mut found = crate::mapped::Problems {
        source: problems.source,
        record: None,
        list: Vec::new(),
    };
    found.record = found.optional("reference_number", Some(Text::new(&row.reference)));
    let date = found.optional(
        DECISION_ROLE,
        row.field(DECISION_ROLE)
            .filter(|value| !matches!(value, Ok("")))
            .map(|value| value.and_then(raw::date)),
    );
    problems.list.extend(found.list);
    Ok(date.map(|date| Decision {
        date: AuthorizationDate {
            role: biodata::text!("european_commission_decision_date"),
            date,
        },
        label: DateRoleLabel {
            role: biodata::text!("european_commission_decision_date"),
            label: Some(biodata::text!("European Commission decision date")),
            procedure: found.record,
        },
    }))
}
