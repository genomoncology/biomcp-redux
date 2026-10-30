//! The query's collection boundary preserves accepted-row pages without sharing collection facts.
use biomcp_adapters::{Refusal, opentargets};
use libtest_mimic::Trial;
use serde_json::json;
#[path = "../../../../biomcp-query/src/drug/native/collection_page.rs"]
mod projection;
pub(super) fn trials() -> Vec<Trial> {
    vec![super::trial(
        "drug-native-sources/collection-refusals-stay-with-accepted-collection",
        || {
            opentargets_collection_owns_refusals_across_pages()
                .map_err(|error| error.to_string().into())
        },
    )]
}

fn opentargets_collection_owns_refusals_across_pages() -> Result<(), Box<dyn std::error::Error>> {
    let drug = serde_json::from_value(json!({
        "id":"CHEMBL1", "namespace":"opentargets", "identity_level":"unknown"
    }))?;
    for offset in 0..3 {
        let mut expected_targets = None;
        let mut expected_indications = None;
        for refused in [false, true] {
            let mut targets = vec![
                json!({"id":"ENSG00000141510"}),
                json!({"id":"ENSG00000146648"}),
            ];
            let mut indications = vec![
                json!({"disease":{"id":"EFO_0001"}}),
                json!({"disease":{"id":"EFO_0002"}}),
            ];
            if refused {
                targets.insert(1, json!({"id":"invalid"}));
                indications.insert(1, json!({"disease":{"id":" "}}));
            }
            let raw = serde_json::from_value(json!({"rows":[{"targets":targets}]}))?;
            let mut collection = opentargets::map_targets(&drug, raw)?;
            let raw = serde_json::from_value(json!({"rows":indications}))?;
            let mut other = opentargets::map_indications(&drug, raw)?;
            let expected = if refused {
                Refusal::Refused
            } else {
                Refusal::None
            };
            let page = projection::page(&mut collection, offset, 1);
            let other_page = projection::page(&mut other, offset, 1);
            assert_eq!(
                collection.refusals, expected,
                "Open Targets target collection lost actual refusal"
            );
            assert_eq!(
                other.refusals, expected,
                "Open Targets indication collection lost actual refusal"
            );
            assert_eq!(
                page.refusals(),
                Refusal::None,
                "collection refusal leaked into target page"
            );
            assert_eq!(
                other_page.refusals(),
                Refusal::None,
                "collection refusal leaked into indication page"
            );
            check_position(page.total(), page.received(), page.next_offset(), offset);
            check_position(
                other_page.total(),
                other_page.received(),
                other_page.next_offset(),
                offset,
            );
            let target_reply = serde_json::to_vec(&page.rows)?;
            let indication_reply = serde_json::to_vec(&other_page.rows)?;
            if refused {
                assert_eq!(
                    Some(target_reply),
                    expected_targets,
                    "accepted target page changed"
                );
                assert_eq!(
                    Some(indication_reply),
                    expected_indications,
                    "accepted indication page changed"
                );
            } else {
                expected_targets = Some(target_reply);
                expected_indications = Some(indication_reply);
            }
        }
    }
    Ok(())
}

fn check_position(total: Option<u64>, received: u32, next: Option<u32>, offset: usize) {
    assert_eq!(total, Some(2), "accepted collection total changed");
    assert_eq!(received, u32::from(offset < 2), "accepted slots changed");
    assert_eq!(
        next,
        if offset == 0 { Some(1) } else { None },
        "accepted continuation changed"
    );
}
