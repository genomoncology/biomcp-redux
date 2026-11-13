//! Made-up date cells: the private row mapper reads WHO dates without a source export.
use super::Native;
use biomcp_types::{DrugWhoRow, WhoDateStatus};

fn row(date: &str) -> Result<DrugWhoRow, biodata::Error> {
    Ok(Native {
        reference: biodata::Text::new("SYN-0001")?,
        inn: "Synthetic Tablet 1mg".into(),
        product_type: "".into(),
        therapeutic_area: "".into(),
        applicant: "".into(),
        dosage_form: "".into(),
        basis_of_listing: "".into(),
        date: date.into(),
    }
    .into())
}

#[test]
fn date_cells_read_as_iso_or_say_why_not() -> Result<(), biodata::Error> {
    for (cell, iso, status) in [
        ("14  Oct,  2014", Some("2014-10-14"), None),
        ("", None, Some(WhoDateStatus::NotPublished)),
        ("  ", None, Some(WhoDateStatus::NotPublished)),
        ("2014-10-14", None, Some(WhoDateStatus::Unreadable)),
        ("31  Feb,  2014", None, Some(WhoDateStatus::Unreadable)),
    ] {
        let row = row(cell)?;
        assert_eq!(row.date_of_prequalification.as_ref(), cell, "{cell:?}");
        assert_eq!(
            row.prequalified_on.map(|date| date.to_string()).as_deref(),
            iso,
            "{cell:?}"
        );
        assert_eq!(row.prequalified_on_status, status, "{cell:?}");
    }
    Ok(())
}
