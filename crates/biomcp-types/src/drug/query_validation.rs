//! Validate the source and section selectors of a drug read.
use super::{DrugSection, DrugSource, label_find, query::GetFields};
use crate::{CivicClaimKind, QueryInvalid};

pub(super) fn validate_sections(fields: &GetFields) -> Result<(), QueryInvalid> {
    if fields.section.contains(&DrugSection::Markets)
        || fields.source.is_some_and(|source| !source.supports_get())
    {
        return Err(QueryInvalid::new(&["section", "source"]));
    }
    if fields.document_id.is_some()
        && (fields.source != Some(DrugSource::Drugsfda)
            || fields.section.as_ref() != [DrugSection::Approvals])
    {
        return Err(QueryInvalid::new(&["document_id", "source", "section"]));
    }
    if fields
        .document_id
        .as_ref()
        .is_some_and(|id| id.as_str().contains(','))
    {
        return Err(QueryInvalid::new(&["document_id"])
            .with_message(crate::ErrorMessage::DrugApprovalDocumentList));
    }
    let approvals = fields.source == Some(DrugSource::Drugsfda)
        && fields.section.as_ref() == [DrugSection::Approvals];
    let held = !fields.section.is_empty()
        && fields.section.iter().all(|section| {
            matches!(
                section,
                DrugSection::Approvals
                    | DrugSection::Interactions
                    | DrugSection::Label
                    | DrugSection::Regulatory
                    | DrugSection::Xrefs
                    | DrugSection::EssentialMedicines
            )
        });
    let most = if held { crate::HELD_PAGE_MAX } else { 50 };
    if fields
        .section_limit
        .is_some_and(|n| !(1..=most).contains(&n))
    {
        return Err(QueryInvalid::new(&["section_limit"]));
    }
    validate_window(fields, approvals)?;
    if fields.section.is_empty()
        && (fields.section_limit.is_some() || fields.section_offset.is_some())
    {
        return Err(QueryInvalid::new(&[
            "section",
            "section_limit",
            "section_offset",
        ]));
    }
    validate_label_options(fields)?;
    if fields.product_number.is_some() && !fields.section.contains(&DrugSection::Approvals) {
        return Err(QueryInvalid::new(&["product_number", "section"]));
    }
    if fields.partner.is_some()
        && (fields.source != Some(DrugSource::Ddinter)
            || fields.section.as_ref() != [DrugSection::Interactions])
    {
        return Err(QueryInvalid::new(&["partner", "source", "section"]));
    }
    // The WHO eEML answers only its own section, which a who-eml get reads with or without the word.
    let eml = fields.section.contains(&DrugSection::EssentialMedicines);
    if (fields.source == Some(DrugSource::WhoEml)
        && !matches!(
            fields.section.as_ref(),
            [] | [DrugSection::EssentialMedicines]
        ))
        || (eml && fields.source != Some(DrugSource::WhoEml))
    {
        return Err(QueryInvalid::new(&["source", "section"]));
    }
    if fields.source == Some(DrugSource::FdaDrugRecall) && !fields.section.is_empty() {
        return Err(QueryInvalid::new(&["source", "section"]));
    }
    Ok(())
}
fn validate_label_options(fields: &GetFields) -> Result<(), QueryInvalid> {
    let options = (
        &*fields.find,
        fields.every_version,
        fields.label_section.as_ref(),
    );
    let paging = (fields.section_limit, fields.section_offset);
    if label_find::requested(&fields.section, &fields.find) {
        label_find::check(fields.source, paging, options)
    } else {
        super::history::checked(
            &fields.section,
            fields.source == Some(DrugSource::Drugsfda),
            paging,
            options,
        )
    }
}
fn validate_window(fields: &GetFields, approvals: bool) -> Result<(), QueryInvalid> {
    let window = [
        ("decision_from", fields.decision_from),
        ("decision_to", fields.decision_to),
    ];
    if window.iter().any(|(_, date)| date.is_some()) && (!approvals || fields.document_id.is_some())
    {
        return Err(QueryInvalid::new(&[
            "decision_from",
            "decision_to",
            "source",
            "section",
            "document_id",
        ]));
    }
    for (name, date) in window {
        if date.is_some_and(|d| d.day().is_none()) {
            return Err(QueryInvalid::new(&[name]));
        }
    }
    if fields
        .decision_from
        .zip(fields.decision_to)
        .is_some_and(|(a, b)| a.to_string() > b.to_string())
    {
        return Err(QueryInvalid::new(&["decision_from", "decision_to"]));
    }
    Ok(())
}
pub(super) fn validate_civic(fields: &GetFields) -> Result<(), QueryInvalid> {
    let filter = &fields.evidence_filter;
    if (!filter.evidence_level.is_empty() || filter.evidence_significance.is_some())
        && fields.civic_kind != Some(CivicClaimKind::Evidence)
    {
        return Err(QueryInvalid::new(&[
            "evidence_level",
            "evidence_significance",
            "civic_kind",
        ]));
    }
    if fields
        .civic_profile
        .is_some_and(|id| id == 0 || id > i32::MAX as u64)
    {
        return Err(QueryInvalid::new(&["civic_profile"]));
    }
    if fields.civic_profile.is_some() && fields.civic_kind.is_none() {
        return Err(QueryInvalid::new(&["civic_profile", "civic_kind"]));
    }
    if fields.civic_cursor.is_some() && fields.civic_kind.is_none() {
        return Err(QueryInvalid::new(&["civic_cursor", "civic_kind"]));
    }
    if fields.source != Some(DrugSource::Civic) || !fields.section.contains(&DrugSection::Civic) {
        if fields.civic_kind.is_some() {
            return Err(QueryInvalid::new(&["civic_kind", "source", "section"]));
        }
        if filter.evidence_status.is_some() {
            return Err(QueryInvalid::new(&["evidence_status", "source", "section"]));
        }
    }
    Ok(())
}
