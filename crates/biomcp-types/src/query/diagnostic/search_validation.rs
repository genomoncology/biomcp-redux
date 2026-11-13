//! Source-owned FDA device search and document access.
use super::{DiagnosticSearchSource, QueryInvalid, SearchFields};
fn device_source(v: &SearchFields) -> bool {
    matches!(
        v.source,
        DiagnosticSearchSource::FdaPma | DiagnosticSearchSource::Fda510k
    )
}

pub(super) fn validate_recall(v: &SearchFields, other: bool) -> Result<(), QueryInvalid> {
    if v.source != DiagnosticSearchSource::FdaDeviceRecall {
        let code = v.product_code.is_some() && !device_source(v);
        if code
            || v.initiated_from.is_some()
            || v.initiated_to.is_some()
            || v.classification.is_some()
        {
            return Err(QueryInvalid::new(&[
                "source",
                "product_code",
                "initiated_from",
                "initiated_to",
                "classification",
            ]));
        }
        return Ok(());
    }
    if v.classification.is_some() && v.product_code.is_some() {
        return Err(QueryInvalid::new(&["product_code", "classification"])
            .with_message(crate::ErrorMessage::DeviceRecallClassificationProductCode));
    }
    if other || v.marker.is_some() || v.bundle_revision.is_some() {
        return Err(QueryInvalid::new(&[
            "source",
            "gene_id",
            "gene",
            "condition_id",
            "type",
            "device_id",
            "drug_application",
            "biomarker",
            "marker",
            "bundle_revision",
        ]));
    }
    for (name, date) in [
        ("initiated_from", v.initiated_from),
        ("initiated_to", v.initiated_to),
    ] {
        if date.is_some_and(|d| d.day().is_none()) {
            return Err(QueryInvalid::new(&[name]));
        }
    }
    if v.initiated_from
        .zip(v.initiated_to)
        .is_some_and(|(a, b)| a.to_string() > b.to_string())
    {
        return Err(QueryInvalid::new(&["initiated_from", "initiated_to"]));
    }
    Ok(())
}

pub(super) fn validate_device(v: &SearchFields, other: bool) -> Result<(), QueryInvalid> {
    if !device_source(v) {
        // GTR matches literal test names; other non-device sources have no name field.
        let name = v.name.is_some() && v.source != DiagnosticSearchSource::Gtr;
        if name || v.decision_from.is_some() || v.decision_to.is_some() {
            return Err(QueryInvalid::new(&[
                "source",
                "name",
                "decision_from",
                "decision_to",
            ]));
        }
        if v.name
            .as_ref()
            .is_some_and(|v| !v.as_str().chars().any(char::is_alphanumeric))
        {
            return Err(QueryInvalid::new(&["name"]));
        }
        return Ok(());
    }
    if other || v.bundle_revision.is_some() {
        return Err(QueryInvalid::new(&[
            "source",
            "gene_id",
            "gene",
            "condition_id",
            "type",
            "device_id",
            "drug_application",
            "biomarker",
            "marker",
            "initiated_from",
            "initiated_to",
            "bundle_revision",
        ]));
    }
    for (name, date) in [
        ("decision_from", v.decision_from),
        ("decision_to", v.decision_to),
    ] {
        if date.is_some_and(|d| d.day().is_none()) {
            return Err(QueryInvalid::new(&[name]));
        }
    }
    if v.decision_from
        .zip(v.decision_to)
        .is_some_and(|(a, b)| a.to_string() > b.to_string())
    {
        return Err(QueryInvalid::new(&["decision_from", "decision_to"]));
    }
    Ok(())
}

/// WHO IVD publishes only a prequalification year, so its window takes whole years.
pub(super) fn validate_who_years(v: &SearchFields) -> Result<(), QueryInvalid> {
    let years = v.prequalified_from.is_some() || v.prequalified_to.is_some();
    if years && v.source != DiagnosticSearchSource::WhoIvd {
        return Err(QueryInvalid::new(&[
            "source",
            "prequalified_from",
            "prequalified_to",
        ]));
    }
    if [v.prequalified_from, v.prequalified_to]
        .into_iter()
        .flatten()
        .any(|d| d.month().is_some())
    {
        return Err(QueryInvalid::new(&["prequalified_from", "prequalified_to"])
            .with_message(crate::ErrorMessage::WhoIvdYears));
    }
    if v.prequalified_from
        .zip(v.prequalified_to)
        .is_some_and(|(a, b)| a.year() > b.year())
    {
        return Err(QueryInvalid::new(&["prequalified_from", "prequalified_to"]));
    }
    Ok(())
}
