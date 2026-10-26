//! Public author contracts before adapters exist; rows below are synthetic.
use biodata::*;
use biomcp_types::*;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
fn roundtrip<T: DeserializeOwned + Serialize>(v: Value) -> Value {
    serde_json::to_value(serde_json::from_value::<T>(v).unwrap()).unwrap()
}
#[test]
fn source_qualified_identity_checks_checksum_and_aliases() {
    for (input, canonical) in [
        ("semantic-scholar:2060978", "semantic-scholar:2060978"),
        ("semanticscholar:2060978", "semantic-scholar:2060978"),
        ("orcid:0000-0002-4299-3018", "orcid:0000-0002-4299-3018"),
        ("orcid:0000-0002-1694-233X", "orcid:0000-0002-1694-233X"),
    ] {
        let id: AuthorId = input.parse().unwrap();
        assert_eq!(id.to_string(), canonical);
        assert_eq!(roundtrip::<AuthorId>(json!(input)), json!(canonical));
        assert_eq!(canonical.parse::<AuthorId>().unwrap(), id);
    }
    for bad in ["2060978", "0000-0002-4299-3018", "orcid:0000-0002-4299-3019", "semantic-scholar:0", "semantic-scholar:12 3"] {
        assert!(bad.parse::<AuthorId>().is_err(), "{bad}");
    }
}
#[test]
fn requests_validate_before_io_and_preserve_selection() {
    let q: AuthorSearch = serde_json::from_value(json!({"query":"Example name","source":"semanticscholar"})).unwrap();
    assert_eq!(q.limit(), 5);
    assert_eq!(q.offset(), 0);
    assert_eq!(q.source(), AuthorSearchSource::SemanticScholar);
    assert_eq!(roundtrip::<AuthorSearch>(json!({"query":"Example name","source":"semanticscholar"}))["source"], "semantic-scholar");
    for value in [json!({"query":"x","limit":0}), json!({"query":"x","limit":26}), json!({"query":"x","offset":-1}), json!({"query":" "}), json!({"query":"x","source":"orcid"})] {
        assert!(serde_json::from_value::<AuthorSearch>(value).is_err());
    }
    assert!(serde_json::from_value::<AuthorGet>(json!({"ids":[]})).is_err());
    assert!(serde_json::from_value::<AuthorGet>(json!({"ids":["2060978"]})).is_err());
    let ids = json!({"ids":["semantic-scholar:2060978","orcid:0000-0002-4299-3018"]});
    assert_eq!(roundtrip::<AuthorGet>(ids.clone()), ids);
    let p: AuthorPapersQuery = serde_json::from_value(json!({"id":"semantic-scholar:2060978","full":true,"limit":25,"offset":5,"pool":"native-pool"})).unwrap();
    assert_eq!(p.section(), Some(AuthorPaperSection::Details));
    assert_eq!(p.limit(),25);
    assert_eq!(p.offset(),5);
    for value in [json!({"id":"semantic-scholar:1","limit":0}),json!({"id":"semantic-scholar:1","limit":26}),json!({"id":"semantic-scholar:1","offset":-1}),json!({"id":"semantic-scholar:1","section":"unknown"}),json!({"id":"semantic-scholar:1","section":null})] {
        assert!(serde_json::from_value::<AuthorPapersQuery>(value).is_err());
    }
}
