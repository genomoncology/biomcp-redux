//! Check every shipped presentation against its own FDA search keys.
use super::decode::decode;
use biomcp_fetch::Row;
use std::collections::HashMap;

#[test]
fn every_shipped_product_resolves_by_its_own_names_and_bla() {
    let bytes = include_bytes!("seed.csv");
    let release = decode(bytes).expect("complete shipped release");
    let by_product: HashMap<_, _> = release
        .rows
        .iter()
        .map(|row| ((row.row.bla.as_str(), row.row.product_number.as_ref()), row))
        .collect();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(bytes.as_slice());
    let mut headings = 0;
    let mut checked = 0;
    for record in reader.records() {
        let record = record.expect("valid FDA CSV");
        if record.get(0) == Some("N/R/U") {
            headings += 1;
        } else if headings == 2 && record.iter().any(|cell| !cell.is_empty()) {
            let bla = format!("BLA{}", &record[1]);
            let product = &record[12];
            let row = by_product
                .get(&(bla.as_str(), product))
                .unwrap_or_else(|| panic!("missing {bla} product {product}"));
            let keys = row.keys();
            for name in [bla.as_str(), &record[2], &record[3]] {
                if !name.is_empty() {
                    assert!(row.matches(name), "{bla} product {product} missing {name}");
                    assert!(
                        keys.exact.iter().any(|(_, key)| key.eq_ignore_ascii_case(name)),
                        "{bla} product {product} did not index {name}"
                    );
                }
            }
            checked += 1;
        }
    }
    assert!(checked > 0);
    assert_eq!(checked, release.rows.len());
}
