//! The monthly CSV omits the regulating center. The advanced search table publishes it.
use biomcp_types::DrugNameMatch;

/// A product listed under CBER in FDA's Purple Book.
#[derive(Clone, Copy, Debug)]
pub struct CberProduct {
    /// FDA's six-digit BLA number.
    pub number: &'static str,
    /// Proprietary name when it identifies this center.
    pub brand: &'static str,
    /// Proper name when it identifies this center.
    pub proper: &'static str,
}

impl CberProduct {
    /// Exact licence page, independent of the available local Purple Book search.
    pub fn url(self) -> String {
        format!(
            "{}/index.cfm?blaNo={}&event=productdetails",
            super::SPEC.base,
            self.number
        )
    }
}

/// Resolve an exact published name to one CBER licence, preferring the proprietary name.
pub fn cber_product(name: &str) -> Option<CberProduct> {
    let rows = include_str!("cber.tsv").lines().filter_map(|line| {
        let (number, names) = line.split_once('\t')?;
        let (brand, proper) = names.split_once('\t')?;
        Some(CberProduct {
            number,
            brand,
            proper,
        })
    });
    for field in [|row: CberProduct| row.brand, |row: CberProduct| row.proper] {
        let mut matches = rows.clone().filter(|&row| {
            let value = field(row);
            !value.is_empty() && DrugNameMatch::same_name(value, name)
        });
        if let Some(first) = matches.next() {
            return matches
                .all(|row| row.number == first.number)
                .then_some(first);
        }
    }
    None
}
