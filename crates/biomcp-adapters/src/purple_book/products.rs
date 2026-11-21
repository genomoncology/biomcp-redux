//! Group matching FDA presentations by application without losing their distinctions.
use super::dates::Dates;
use biomcp_types::{
    PurpleBookDesignationCount, PurpleBookPresentation, PurpleBookProduct, PurpleBookReference,
};
use std::collections::{BTreeMap, BTreeSet};

/// Summarize the presentations of each application in BLA order.
pub fn summarize(rows: &[PurpleBookPresentation]) -> Box<[PurpleBookProduct]> {
    rows.chunk_by(|left, right| left.bla == right.bla)
        .filter_map(product)
        .collect()
}

fn product(rows: &[PurpleBookPresentation]) -> Option<PurpleBookProduct> {
    let first = rows.first()?;
    let approval = Dates::read(rows.iter().map(|row| row.approval_date.as_deref()));
    let interchangeability = Dates::read(
        rows.iter()
            .map(|row| row.interchangeability_date.as_deref()),
    );
    let expiration = Dates::read(
        rows.iter()
            .map(|row| row.first_interchangeable_exclusivity_expiration.as_deref()),
    );
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.license_type).or_insert(0) += 1;
    }
    let applicant = rows.iter().find_map(|row| row.applicant.as_deref());
    Some(PurpleBookProduct {
        bla: first.bla.clone(),
        name: first.name.clone(),
        other_names: other(rows.iter().map(|row| row.name.as_ref()), Some(&first.name)),
        proper_name: first.proper_name.clone(),
        other_proper_names: other(
            rows.iter().map(|row| row.proper_name.as_ref()),
            Some(&first.proper_name),
        ),
        applicant: applicant.map(Into::into),
        other_applicants: other(
            rows.iter().filter_map(|row| row.applicant.as_deref()),
            applicant,
        ),
        references: references(rows),
        presentation_count: rows.len() as u32,
        designations: counts
            .into_iter()
            .map(|(license_type, count)| PurpleBookDesignationCount {
                license_type,
                count,
            })
            .collect(),
        earliest_approval_date: approval.earliest.clone(),
        approval_date_unreported: approval.unreported,
        approval_dates: approval.differing(),
        earliest_interchangeability_date: interchangeability.earliest.clone(),
        interchangeability_date_unreported: interchangeability.unreported,
        interchangeability_dates: interchangeability.differing(),
        first_interchangeable_exclusivity_expirations: expiration.values,
        first_interchangeable_exclusivity_expiration_unreported: expiration.unreported,
    })
}

fn other<'a>(values: impl Iterator<Item = &'a str>, first: Option<&str>) -> Box<[Box<str>]> {
    values
        .filter(|value| Some(*value) != first)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(Into::into)
        .collect()
}

fn references(rows: &[PurpleBookPresentation]) -> Box<[PurpleBookReference]> {
    let mut by_names = BTreeMap::new();
    for row in rows {
        by_names
            .entry((&row.reference_name, &row.reference_proper_name))
            .or_insert(row);
    }
    by_names
        .into_values()
        .map(|row| PurpleBookReference {
            name: row.reference_name.clone(),
            proper_name: row.reference_proper_name.clone(),
            bla: row.reference_bla.clone(),
            bla_candidates: row.reference_bla_candidates.clone(),
        })
        .collect()
}
