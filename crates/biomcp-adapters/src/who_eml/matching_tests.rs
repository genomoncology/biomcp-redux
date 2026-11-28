//! Resolve every name from the pinned eEML source snapshot.
use super::{header, matching_medicine};

#[test]
fn every_who_medicine_name_resolves_to_its_own_entry() {
    let medicines: Vec<_> = include_str!("medicine-names.tsv")
        .lines()
        .map(|line| line.split_once('\t').expect("eEML number and name"))
        .collect();
    let headers: Vec<_> = medicines
        .iter()
        .map(|(number, name)| {
            header(number, crate::mapped::record(*name)).expect("valid eEML number")
        })
        .collect();
    assert_eq!(medicines.len(), 650);
    for (number, name) in medicines {
        let matched = matching_medicine(&headers, name).expect(name);
        assert_eq!(matched.id.to_string(), number, "{name}");
    }
    assert!(matching_medicine(&headers, "insulin").is_none());
    assert!(matching_medicine(&headers, "seasonal").is_none());
}
