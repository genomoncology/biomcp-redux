//! Drug public API contracts, before any command implementation exists.
use biodata::{DrugId, DrugIdentityLevel};
use biomcp_types::{DrugGet, DrugSearch};
use serde_json::json;

#[test]
fn published_ids_roundtrip_without_source_prefixes() {
    for value in [
        "CHEMBL941",
        "EMEA/H/C/000406",
        "NDA021335",
        "741ff3e3-dc1a-45a6-84e5-2481b27131aa",
        "DDInter1744",
        "5",
        "KTUFNOKKBVMGRW-UHFFFAOYSA-N",
    ] {
        let id: DrugId = value.parse().expect("published ID");
        assert_eq!(id.to_string(), value);
        assert_eq!(
            serde_json::from_value::<DrugId>(json!(value)).expect("serde ID"),
            id
        );
    }
    for value in [
        "chembl:CHEMBL941",
        "ema:EMEA/H/C/000406",
        "openfda-label:741ff3e3-dc1a-45a6-84e5-2481b27131aa",
        "",
        "Gleevec tablets",
        " CHEMBL941",
    ] {
        assert!(value.parse::<DrugId>().is_err(), "{value}");
        assert!(serde_json::from_value::<DrugId>(json!(value)).is_err());
    }
}

#[test]
fn search_defaults_and_required_get_source_are_explicit() {
    let query: DrugSearch = serde_json::from_value(json!({"query":"Gleevec"})).expect("search");
    assert_eq!(
        serde_json::to_value(query).expect("search JSON")["source"],
        "mychem"
    );
    assert!(serde_json::from_value::<DrugGet>(json!({"input":"CHEMBL941"})).is_err());
    assert!(
        serde_json::from_value::<DrugGet>(json!({"input":"chembl:CHEMBL941", "source":"chembl"}))
            .is_err()
    );
}

#[test]
fn selected_source_checks_its_native_identifier_grammar() {
    use biomcp_types::DrugSource;
    for (source, id) in [
        ("mychem", "KTUFNOKKBVMGRW-UHFFFAOYSA-N"),
        ("chembl", "CHEMBL941"),
        ("opentargets", "CHEMBL941"),
        ("ema", "EMEA/H/C/000406"),
        ("drugsfda", "NDA021335"),
        ("openfda-label", "741ff3e3-dc1a-45a6-84e5-2481b27131aa"),
        ("ddinter", "DDInter1744"),
        ("civic", "5"),
    ] {
        let selected: DrugSource = source.parse().expect("source");
        let id: DrugId = id.parse().expect("published ID");
        assert!(selected.accepts(&id));
        let request = DrugGet::new(id, selected).expect("native request");
        let json = serde_json::to_value(&request).expect("request JSON");
        assert_eq!(
            serde_json::from_value::<DrugGet>(json).expect("request decode"),
            request
        );
    }
    for (source, id) in [
        ("chembl", "Gleevec"),
        ("ema", "CHEMBL941"),
        ("drugsfda", "NDA1"),
        ("ddinter", "1744"),
        ("civic", "0"),
        ("openfda-label", "not-a-uuid"),
    ] {
        assert!(serde_json::from_value::<DrugGet>(json!({"source":source,"input":id})).is_err());
    }
}

#[test]
fn closed_vocabularies_roundtrip_and_refuse_other_sections() {
    use biomcp_types::{DrugSection, DrugSource};
    for name in [
        "ingredient",
        "salt",
        "clinical_formulation",
        "branded_product",
        "spl_set",
        "spl_version",
        "ndc_package",
        "fda_application",
        "fda_product",
        "ema_authorization",
        "ema_application",
        "orphan_designation",
        "unknown",
    ] {
        let value: DrugIdentityLevel = name.parse().expect("identity level");
        assert_eq!(value.to_string(), name);
        assert_eq!(
            serde_json::from_value::<DrugIdentityLevel>(json!(name)).expect("JSON level"),
            value
        );
    }
    for name in [
        "targets",
        "indications",
        "civic",
        "label",
        "approvals",
        "regulatory",
        "interactions",
    ] {
        let value: DrugSection = name.parse().expect("section");
        assert_eq!(value.to_string(), name);
        assert_eq!(
            serde_json::from_value::<DrugSection>(json!(name)).expect("JSON section"),
            value
        );
    }
    for name in [
        "mychem",
        "chembl",
        "opentargets",
        "civic",
        "openfda-label",
        "drugsfda",
        "ema",
        "ddinter",
    ] {
        let value: DrugSource = name.parse().expect("source");
        assert_eq!(value.to_string(), name);
        assert_eq!(
            serde_json::from_value::<DrugSource>(json!(name)).expect("JSON source"),
            value
        );
    }
    for name in ["all", "shortage", "safety", "cell_lines", "unknown"] {
        assert!(name.parse::<DrugSection>().is_err());
    }
    assert!("who".parse::<DrugSource>().is_err());
}

#[test]
fn native_text_and_regulator_date_roles_roundtrip_without_interpretation() {
    use biodata::{AuthorizationEvent, DrugLabelSection};
    let event = json!({"record":"EMEA/H/C/000406","jurisdiction":"native jurisdiction","status":"source status","dates":[{"role":"original authorization", "date":"2001-11-07"},{"role":"other date", "date":"2001-11-06"}],"attribution":{"source":"ema","record":"EMEA/H/C/000406"}});
    let decoded: AuthorizationEvent = serde_json::from_value(event.clone()).expect("event");
    assert_eq!(serde_json::to_value(decoded).expect("event JSON"), event);
    let label = json!({"label":{"id":"741ff3e3-dc1a-45a6-84e5-2481b27131aa","attribution":{"source":"openfda-label"}},"heading":"drug_interactions","text":"  original text\nsecond line  "});
    let decoded: DrugLabelSection = serde_json::from_value(label.clone()).expect("label");
    assert_eq!(serde_json::to_value(decoded).expect("label JSON"), label);
}

#[test]
fn section_options_use_existing_continuation_names_and_reject_card_paging() {
    let query = json!({"source":"openfda-label","input":"741ff3e3-dc1a-45a6-84e5-2481b27131aa", "section":["label"], "label_section":"drug_interactions", "section_offset":10, "section_limit":20});
    let parsed: DrugGet = serde_json::from_value(query.clone()).expect("label continuation");
    assert_eq!(serde_json::to_value(parsed).expect("query JSON"), query);
    for extra in [
        json!({"section_limit":0,"section":["targets"]}),
        json!({"section_limit":51,"section":["targets"]}),
        json!({"section_offset":0}),
        json!({"label_section":"drug_interactions"}),
        json!({"section":["label"]}),
        json!({"product_number":"001"}),
        json!({"text_offset":1}),
    ] {
        let mut value = json!({"source":"chembl", "input":"CHEMBL941"});
        value
            .as_object_mut()
            .expect("query object")
            .extend(extra.as_object().expect("extra object").clone());
        assert!(serde_json::from_value::<DrugGet>(value).is_err());
    }
    for limit in [0, 51] {
        assert!(
            serde_json::from_value::<DrugSearch>(json!({"query":"Gleevec", "limit":limit}))
                .is_err()
        );
    }
}

#[test]
#[cfg(feature = "schema")]
fn header_and_card_exclude_deeper_disclosures_from_the_public_contract() {
    use biomcp_types::{DrugCard, DrugHeader};
    let header = json!({"id":"CHEMBL941", "source":"chembl", "name":"native name", "identity_level":"unknown"});
    let decoded: DrugHeader = serde_json::from_value(header.clone()).expect("header");
    assert_eq!(serde_json::to_value(decoded).expect("header JSON"), header);
    for extra in [
        "approval",
        "approval_date",
        "authorization_events",
        "label_text",
        "interactions",
        "targets",
    ] {
        let mut value = header.clone();
        value[extra] = json!([]);
        assert!(serde_json::from_value::<DrugHeader>(value).is_err());
        let value = json!({"header":header, extra:[]});
        assert!(serde_json::from_value::<DrugCard>(value).is_err());
    }
    let schema = serde_json::to_value(schemars::schema_for!(DrugHeader)).expect("header schema");
    let keys: Vec<_> = schema["properties"]
        .as_object()
        .expect("header properties")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        keys,
        ["id", "identity_level", "name", "source", "type", "url"]
    );
    let schema = serde_json::to_value(schemars::schema_for!(DrugCard)).expect("card schema");
    let text = serde_json::to_string(&schema).expect("schema JSON");
    for forbidden in [
        "AuthorizationEvent",
        "DrugLabelSection",
        "DrugInteractionClaim",
        "DrugTargetRelation",
    ] {
        assert!(!text.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn civic_claim_streams_continue_the_selected_drug_without_a_variant_query() {
    use biomcp_types::{CivicClaimKind, CivicProfile, DrugCivic};
    let selected = json!({"source":"civic", "input":"5", "section":["civic"], "section_offset":7, "section_limit":2});
    let selected: DrugGet = serde_json::from_value(selected).expect("selected drug");
    let evidence = selected
        .with_civic_page(
            12,
            CivicClaimKind::Evidence,
            Some("native-evidence-cursor".parse().expect("cursor")),
        )
        .expect("evidence continuation");
    let assertions = selected
        .with_civic_page(
            12,
            CivicClaimKind::Assertions,
            Some("native-assertion-cursor".parse().expect("cursor")),
        )
        .expect("assertion continuation");
    let profile = json!({"record":{"id":"12"}, "label":"synthetic source profile", "compound":false, "membership":"candidate", "members":[], "evidence_count":3, "assertion_count":1, "statements":[], "evidence_page":{"returned":2,"total":3,"next":evidence}, "assertion_page":{"returned":0,"total":1,"next":assertions}});
    let decoded: CivicProfile<DrugGet> =
        serde_json::from_value(profile.clone()).expect("drug profile");
    assert_eq!(
        serde_json::to_value(&decoded).expect("profile JSON"),
        profile
    );
    for (next, kind, cursor) in [
        (
            decoded.evidence_page.next.expect("evidence next"),
            CivicClaimKind::Evidence,
            "native-evidence-cursor",
        ),
        (
            decoded.assertion_page.next.expect("assertion next"),
            CivicClaimKind::Assertions,
            "native-assertion-cursor",
        ),
    ] {
        assert_eq!(next.input().to_string(), "5");
        assert_eq!(next.source().to_string(), "civic");
        assert_eq!(next.civic_profile(), Some(12));
        assert_eq!(next.civic_kind(), Some(kind));
        assert_eq!(next.civic_cursor().expect("native cursor").as_str(), cursor);
        assert_eq!(next.section_offset(), Some(7));
        assert_eq!(next.section_limit(), Some(2));
    }
    let response = json!({"identity":{"id":"5","source":"civic","name":"synthetic therapy","identity_level":"unknown"}, "civic":{"state":"data","source":"civic","data":{"records":[profile], "offset":0}}});
    let decoded: DrugCivic = serde_json::from_value(response.clone()).expect("drug CIViC response");
    assert_eq!(
        serde_json::to_value(decoded).expect("response JSON"),
        response
    );
}

#[test]
fn civic_continuation_refuses_incomplete_or_cross_source_claim_selectors() {
    for selectors in [
        json!({"civic_profile":12}),
        json!({"civic_kind":"evidence"}),
        json!({"civic_cursor":"native cursor"}),
        json!({"civic_profile":0,"civic_kind":"evidence"}),
        json!({"civic_profile":2147483648_u64,"civic_kind":"evidence"}),
        json!({"civic_profile":12,"civic_kind":"unknown"}),
        json!({"source":"chembl","input":"CHEMBL941","civic_profile":12,"civic_kind":"evidence"}),
        json!({"section":[],"civic_profile":12,"civic_kind":"evidence"}),
    ] {
        let mut query = json!({"source":"civic","input":"5","section":["civic"]});
        query
            .as_object_mut()
            .expect("query")
            .extend(selectors.as_object().expect("selectors").clone());
        assert!(serde_json::from_value::<DrugGet>(query).is_err());
    }
}

#[test]
fn label_candidate_versions_are_native_optional_identity_metadata() {
    let mut value = json!({"id":"211ef2da-2868-4a77-8055-1cb2cd78e24b", "source":"openfda-label", "name":"Synthetic brand", "identity_level":"spl_set"});
    let unversioned: biomcp_types::DrugHeader =
        serde_json::from_value(value.clone()).expect("unversioned");
    assert!(
        serde_json::to_value(unversioned)
            .expect("JSON")
            .get("version")
            .is_none()
    );
    value["version"] = json!("41");
    let versioned: biomcp_types::DrugHeader =
        serde_json::from_value(value.clone()).expect("versioned");
    assert_eq!(serde_json::to_value(versioned).expect("JSON"), value);
    value["text"] = json!("not an identity field");
    assert!(serde_json::from_value::<biomcp_types::DrugHeader>(value).is_err());
}
