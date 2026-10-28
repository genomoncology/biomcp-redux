//! Public variant cards use one identifier spelling at every reference boundary.
use biomcp_types::{VariantCard, VariantInput};
use serde_json::json;

#[test]
fn card_ids_requested_and_statement_subjects_share_published_text()
-> Result<(), Box<dyn std::error::Error>> {
    for id in [
        "rs113488022",
        "CA123643",
        "PA123",
        "VCV000013961",
        "VCV000013961.98",
        "clinvar:13961",
        "COSV55386424",
        "COSM476",
        "COSN123",
        "ga4gh:VA.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    ] {
        let golden = json!({
            "resolution":"unresolved", "source":"erepo",
            "candidates":[{
                "identity":{"ids":[id]}, "requested":id,
                "source":"erepo", "record":{"id":"synthetic-record"},
                "sections":{"erepo":{"state":"data", "source":"erepo", "data":{
                    "assertions":[{"kind":"document", "domain":"germline", "statement":{
                        "subject":{"level":"allele", "id":id},
                        "proposition":"pathogenicity", "classification":{"label":"synthetic"},
                        "attribution":{"source":"erepo", "record":"synthetic-record"}
                    }}], "page":{"offset":0,"returned":1,"total":1}
                }}}
            }]
        });
        let card: VariantCard = serde_json::from_value(golden.clone())?;
        assert_eq!(serde_json::to_value(card)?, golden);
        assert_eq!(VariantInput::new(id)?.as_str(), id);
    }
    Ok(())
}
