//! Contract consumers distinguish query acceptance from reply guarantees.
use super::{
    compatibility::check_reviewed,
    policy::{ContractKind, Reviews},
};
use serde_json::json;

#[test]
fn required_reply_names_cannot_disappear_within_a_major() {
    let old = json!({"$id":"biodata.example/1","type":"object","properties":{
        "name":{"type":"string"}},"required":["name"]});
    let mut new = old.clone();
    new["required"] = json!([]);
    assert!(check_reviewed(&old, &new, ContractKind::Reply, &Reviews::default()).is_err());
    assert!(check_reviewed(&old, &new, ContractKind::Query, &Reviews::default()).is_ok());
    assert!(check_reviewed(&new, &old, ContractKind::Reply, &Reviews::default()).is_ok());
    assert!(check_reviewed(&new, &old, ContractKind::Query, &Reviews::default()).is_err());
    new["$id"] = json!("biodata.example/2");
    assert!(check_reviewed(&old, &new, ContractKind::Reply, &Reviews::default()).is_ok());
}

#[test]
fn pattern_widening_requires_an_exact_reviewed_transition() {
    let old = json!({"$id":"biodata.example/1","type":"string","pattern":"^a$"});
    let mut new = old.clone();
    new["pattern"] = json!("^[ab]$");
    let reviews: Reviews = serde_json::from_value(json!({"pattern_widenings":[{
        "schema":"biodata.example/1", "path":"", "from":"^a$", "to":"^[ab]$",
        "review":"Accept the additional canonical spelling"
    }]}))
    .unwrap();
    assert!(check_reviewed(&old, &new, ContractKind::Reply, &Reviews::default()).is_err());
    assert!(check_reviewed(&old, &new, ContractKind::Reply, &reviews).is_ok());
    new["pattern"] = json!("^b$");
    assert!(check_reviewed(&old, &new, ContractKind::Reply, &reviews).is_err());
}

#[test]
fn successor_identity_requires_a_review_and_a_new_major() {
    let old = json!({"$id":"biodata.example/1","type":"string"});
    let mut new = json!({"$id":"biodata.renamed/2","type":"string"});
    let reviews: Reviews = serde_json::from_value(json!({"successors":[{
        "from":"biodata.example/1", "to":"biodata.renamed/2", "review":"Rename the contract"
    }]}))
    .unwrap();
    assert!(check_reviewed(&old, &new, ContractKind::Reply, &Reviews::default()).is_err());
    assert!(check_reviewed(&old, &new, ContractKind::Reply, &reviews).is_ok());
    new["$id"] = json!("biodata.renamed/1");
    assert!(check_reviewed(&old, &new, ContractKind::Reply, &reviews).is_err());
}

#[test]
fn growable_enum_contracts_keep_known_values_and_accept_future_names() {
    let old = json!({"$id":"biodata.example/1", "type":"string",
        "x-extensible-enum":["first","second"]});
    let mut new = old.clone();
    new["x-extensible-enum"] = json!(["first", "second", "third"]);
    assert!(check_reviewed(&old, &new, ContractKind::Reply, &Reviews::default()).is_ok());
    new["x-extensible-enum"] = json!(["first"]);
    assert!(check_reviewed(&old, &new, ContractKind::Reply, &Reviews::default()).is_err());
    let mut narrowed = old.clone();
    narrowed["enum"] = json!(["first", "second"]);
    assert!(check_reviewed(&old, &narrowed, ContractKind::Reply, &Reviews::default()).is_err());
    let closed = json!({"$id":"biodata.example/1", "type":"string", "enum":["first","second"]});
    assert!(check_reviewed(&old, &closed, ContractKind::Reply, &Reviews::default()).is_err());
}
