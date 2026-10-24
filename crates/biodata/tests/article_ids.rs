//! Public article identifier and command input contracts.
use biodata::{Pmid,Pmcid,Doi,ArticleSearch};
#[test]
fn article_identifiers_check_namespaces_and_doi_equality() {
    for bad in ["0","01","-1","4294967296","PMC1",""] { assert!(bad.parse::<Pmid>().is_err(),"{bad}"); }
    assert_eq!("17611497".parse::<Pmid>().unwrap().get(),17611497);
    assert_eq!("PMC6181777".parse::<Pmcid>().unwrap().to_string(),"PMC6181777");
    assert!("6181777".parse::<Pmcid>().is_err());
    assert_eq!("10.1056/NEJMoa1203421".parse::<Doi>().unwrap(),"10.1056/nejmoa1203421".parse::<Doi>().unwrap());
    for bad in ["10./x","10.a/x","10.1/","10.1/a b"] { assert!(bad.parse::<Doi>().is_err()); }
}
#[test]
fn article_search_checks_selectors_and_uses_the_reviewed_term_order() {
    let valid=serde_json::json!({"gene":"KRAS","variant":"G12C","drug":"sotorasib","disease":"lung"});
    let query:ArticleSearch=serde_json::from_value(valid).unwrap();
    assert_eq!(query.words(),"G12C sotorasib lung");
    for bad in [serde_json::json!({}),serde_json::json!({"text":"x","limit":0}),serde_json::json!({"text":"x","limit":51}),serde_json::json!({"text":"x","source":[]}),serde_json::json!({"text":null}),serde_json::json!({"text":"x","sort":"date"})] {assert!(serde_json::from_value::<ArticleSearch>(bad).is_err());}
}
