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
    // openFDA answers HTTP 400 to a letter outside ASCII, so the name is refused before the request.
    if v.name.as_ref().is_some_and(|v| !v.as_str().is_ascii()) {
        return Err(
            QueryInvalid::new(&["name"]).with_message(crate::ErrorMessage::DeviceRecallNameAscii)
        );
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
    // openFDA publishes intended-use text (ao_statement) for PMA only; its 510(k) records have none (live, 2026-11-14).
    if let Some(text) = &v.intended_use {
        if v.source != DiagnosticSearchSource::FdaPma {
            return Err(QueryInvalid::new(&["source", "intended_use"])
                .with_message(crate::ErrorMessage::DiagnosticIntendedUsePma));
        }
        // openFDA answers HTTP 400 to a letter outside ASCII inside a phrase.
        if !text.as_str().is_ascii() || !text.as_str().chars().any(char::is_alphanumeric) {
            return Err(QueryInvalid::new(&["intended_use"])
                .with_message(crate::ErrorMessage::DiagnosticIntendedUseAscii));
        }
    }
    if !device_source(v) {
        // GTR matches literal test names and device recalls their product description;
        // other non-device sources have no name field.
        let name = v.name.is_some()
            && !matches!(
                v.source,
                DiagnosticSearchSource::Gtr | DiagnosticSearchSource::FdaDeviceRecall
            );
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
