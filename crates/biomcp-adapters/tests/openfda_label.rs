//! Native label parser contracts use synthetic prose, never licensed clinical answers.
use biomcp_adapters::openfda_label::Record;
use serde_json::json;

#[test]
fn native_label_parts_keep_every_character_and_missing_fields_stay_missing() {
    let record: Record = serde_json::from_value(json!({
        "id":"2b30c65b-bf6e-4a30-9754-585d500d59f3",
        "set_id":"211ef2da-2868-4a77-8055-1cb2cd78e24b",
        "version":"41", "effective_time":"20260713",
        "openfda":{"brand_name":["Synthetic brand"],"spl_set_id":["211ef2da-2868-4a77-8055-1cb2cd78e24b"]},
        "dosage_and_administration":["  α\nβ  ","\tSecond part.\n"]
    })).unwrap();
    assert_eq!(
        record.parts("dosage_and_administration").unwrap(),
        ["  α\nβ  ", "\tSecond part.\n"]
    );
    assert!(record.parts("boxed_warning").is_none());
    assert!(record.parts("not_a_heading").is_none());
    assert_eq!(
        record.header().unwrap().identity_level,
        biodata::DrugIdentityLevel::SplSet
    );
    assert_eq!(
        record.header().unwrap().id.to_string(),
        "211ef2da-2868-4a77-8055-1cb2cd78e24b"
    );
    assert_eq!(
        record.reference().unwrap().revised.unwrap().to_string(),
        "2026-07-13"
    );
}

#[test]
fn bounded_label_pages_reassemble_native_arrays_with_empty_parts_and_unicode() {
    let originals = [
        String::new(),
        " α😀\\\"\n\t\u{1f} ".repeat(2000),
        String::new(),
        " final β\n ".to_owned(),
        String::new(),
    ];
    let record: Record = serde_json::from_value(json!({
        "id":"2b30c65b-bf6e-4a30-9754-585d500d59f3",
        "set_id":"211ef2da-2868-4a77-8055-1cb2cd78e24b",
        "version":"41", "effective_time":"20260713",
        "openfda":{"brand_name":["Synthetic brand"]},
        "dosage_and_administration": originals
    }))
    .unwrap();
    for limit in [1, 3, 50] {
        let mut rebuilt = vec![String::new(); originals.len()];
        let mut seen = vec![false; originals.len()];
        let mut offset = 0;
        loop {
            let page = record
                .label("dosage_and_administration", offset, limit)
                .unwrap()
                .unwrap();
            assert_eq!(page.offset, offset);
            assert!(!page.fragments.is_empty());
            assert!(page.fragments.len() <= limit as usize);
            assert!(serde_json::to_string(&page.text).unwrap().len() - 2 <= 6144);
            let mut text = page.text.chars();
            for fragment in &page.fragments {
                let row = fragment.source_part as usize;
                seen[row] = true;
                assert_eq!(
                    u64::from(fragment.source_offset),
                    rebuilt[row].chars().count() as u64
                );
                rebuilt[row].extend(text.by_ref().take(fragment.characters as usize));
            }
            assert!(text.next().is_none());
            offset += page.fragments.len() as u32;
            if u64::from(offset) == page.total.unwrap() {
                break;
            }
        }
        assert_eq!(rebuilt, originals);
        assert!(seen.into_iter().all(|seen| seen));
    }
    assert!(record.label("boxed_warning", 0, 1).unwrap().is_none());
    assert!(record.label("dosage_and_administration", 0, 0).is_err());
}
