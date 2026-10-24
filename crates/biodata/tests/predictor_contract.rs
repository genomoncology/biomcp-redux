//! The shared predictor contract preserves source numbers and labels without interpretation.
use biodata::{PredictorScore, VariantConservation};
use serde_json::json;
#[test]
fn source_predictor_contract_keeps_zero_and_native_labels() {
    let source = json!({"subject":{"level":"allele","hgvs":"NC_000007.13:g.140453136A>T"},"method":"cadd.sift.val","score":0.0,"label":"deleterious","attribution":{"source":"myvariant","record":"chr7:g.140453136A>T"}});
    let score: PredictorScore = serde_json::from_value(source.clone()).expect("valid source score");
    assert_eq!(serde_json::to_value(score).expect("serialize"), source);
    assert!(serde_json::from_value::<PredictorScore>(json!({"score":null})).is_err());
    let empty: VariantConservation = serde_json::from_value(json!({"scores":[],"components":[]}))
        .expect("an absent record field has no fabricated score");
    assert!(empty.scores.is_empty());
}
