//! pgx-types: checked public requests and neutral records, with no I/O.
use biodata::{ClinPgxAccession, PaId};
use biomcp_types::{PgxGet, PgxSearch};
use serde_json::json;
#[test]
fn explicit_sources_and_section_ownership_are_checked_before_io() -> Result<(), Box<dyn std::error::Error>> {
    let search: PgxSearch = serde_json::from_value(json!({"gene":"DPYD","source":"cpic","cpic_level":"B/C","limit":2}))?;
    assert_eq!(search.at_offset(2).gene(), search.gene());
    assert_eq!(search.at_offset(2).cpic_level(), search.cpic_level());
    for value in [json!({"gene":"DPYD"}),json!({"gene":"DPYD","source":"clinpgx"}),json!({"gene":"DPYD","source":"cpic","limit":0}),json!({"gene":"DPYD","source":"cpic","drug":"x"})] {
        assert!(serde_json::from_value::<PgxSearch>(value).is_err());
    }
    for source in ["cpic","clinpgx"] {
        for section in [None,Some("recommendations"),Some("guidelines"),Some("diplotype"),Some("annotations")] {
            let mut q=json!({"gene":"TPMT","source":source});
            if let Some(s)=section { q["section"]=json!(s); }
            if section==Some("diplotype") { q["diplotype"]=json!("*1/*3A"); }
            let valid=if source=="cpic" { section!=Some("annotations") } else { section==Some("annotations") };
            assert_eq!(serde_json::from_value::<PgxGet>(q.clone()).is_ok(),valid,"{q}");
        }
    }
    for value in [json!({"gene":"TPMT","source":"cpic","section":"diplotype"}),json!({"gene":"TPMT","source":"cpic","diplotype":"*1/*3A"}),json!({"gene":"TPMT","source":"cpic","section":"guidelines","drug_id":"RxNorm:10324"}),json!({"gene":"TPMT","source":"cpic","kind":"clinical"}),json!({"gene":"TPMT","source":"cpic","section_limit":null})] {
        assert!(serde_json::from_value::<PgxGet>(value).is_err());
    }
    let a:ClinPgxAccession="PA166134421".parse()?;
    assert_eq!(a.to_string(),"PA166134421");
    assert!("PA166134421".parse::<PaId>().is_err());
    for invalid in ["PA","PA0","PA-1","pa166134421"," PA166134421","clinpgx:PA166134421"] { assert!(invalid.parse::<ClinPgxAccession>().is_err()); }
    Ok(())
}
