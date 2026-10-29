//! Original registry site walks and response-only attachment/status boundaries.
#![expect(
    clippy::indexing_slicing,
    reason = "Checked captures and command envelopes fail directly on shape drift"
)]
use super::{
    launch::Environment,
    response_cases::{checked_steps, copy_capture},
    trial_get::{command, original},
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial(
            "trial/sites/whole-source-location-and-contact-walks",
            || walks().map_err(|e| e.to_string().into()),
        ),
        transcript_trial("trial/sites/contact-attachment-and-source-status", || {
            attachments().map_err(|e| e.to_string().into())
        }),
        transcript_trial("trial/sites/checked-geographic-coordinates", || {
            coordinates().map_err(|e| e.to_string().into())
        }),
        transcript_trial("trial/sites/old-snapshot-owner-expires", || {
            old_owner().map_err(|e| e.to_string().into())
        }),
    ]
}
fn walks() -> Check {
    for (id, count) in [
        ("NCT03734029", 208),
        ("NCT02987543", 204),
        ("NCT02578680", 0),
    ] {
        let raw = original(id)?;
        for section in ["locations", "contacts"] {
            walk(id, section, &raw, count)?;
        }
    }
    Ok(())
}
fn walk(id: &str, section: &str, raw: &Value, count: usize) -> Check {
    let env = Environment::new()?;
    let replay = super::launch::fixtures().join("replies/trial/get").join(id);
    let mut args = vec!["get".into(), "trial".into(), id.into(), section.into()];
    let mut globals = env.globals(Some(&replay), false);
    let mut rows = Vec::new();
    let mut finished = false;
    let mut max_bytes = 0;
    for page in 0..40 {
        let out = command(&env, &args, &globals, 0)?;
        max_bytes = max_bytes.max(super::source_terms::size(&out, "")?);
        if page == 0 {
            if super::mcp::call_with_globals(&env, &args, false, &globals)? != out {
                return Err("site CLI/MCP parity differs".into());
            }
            // Every continuation must work without recordings or a network source.
            globals = env.globals(None, true);
        }
        let source_card = &out["result"]["items"][0]["card"];
        if id == "NCT02987543"
            && (source_card["status"] != "COMPLETED"
                || source_card["updated"]["date"]
                    != raw["statusModule"]["lastUpdatePostDateStruct"]["date"])
        {
            return Err("trial overall status/update changed".into());
        }
        let result = &out["result"]["items"][0]["sections"][section];
        if result["state"] == "data" {
            let data = &result["data"];
            if data["offset"] != json!(rows.len()) {
                return Err("site walk skipped or repeated a record".into());
            }
            rows.extend(
                data["items"]
                    .as_array()
                    .ok_or("missing records")?
                    .iter()
                    .cloned(),
            );
        } else if result["state"] != "empty" {
            return Err("absent sites did not return empty".into());
        }
        match super::action(&out, "page") {
            Ok(next) => args = next,
            Err(_) => {
                finished = true;
                break;
            }
        }
    }
    if !finished {
        return Err("bounded sites walk did not finish".into());
    }
    let ceiling = match (id, section) {
        ("NCT03734029", "locations") => 5040,
        ("NCT03734029", _) => 2206,
        ("NCT02987543", "locations") => 4645,
        ("NCT02987543", _) => 2511,
        (_, "locations") => 3411,
        _ => 3561,
    };
    if max_bytes + 1 > ceiling {
        return Err(format!(
            "{id} {section} page exceeded {ceiling} byte budget: {}",
            max_bytes + 1
        )
        .into());
    }
    let expected = expected_records(raw, section);
    if rows != expected || section == "locations" && rows.len() != count {
        return Err(format!("{id} {section} lost original fields or order").into());
    }
    Ok(())
}
fn contact(v: &Value) -> Value {
    json!({"name":v["name"],"role":v["role"],"phone":v["phone"],"phone_ext":v["phoneExt"],"email":v["email"]})
}
fn expected_records(raw: &Value, section: &str) -> Vec<Value> {
    let module = &raw["contactsLocationsModule"];
    let mut rows = Vec::new();
    if section == "locations" {
        for row in module["locations"].as_array().into_iter().flatten() {
            let coordinates = row
                .get("geoPoint")
                .map(|p| json!({"latitude":p["lat"],"longitude":p["lon"]}));
            let contacts: Vec<_> = row["contacts"]
                .as_array()
                .into_iter()
                .flatten()
                .map(contact)
                .collect();
            rows.push(json!(
                {"facility":row["facility"],"city":row["city"],"state":row["state"],"zip":row["zip"],"country":row["country"],
                "status":row["status"],"coordinates":coordinates,"contacts":contacts}
            ));
        }
    } else {
        for row in module["centralContacts"].as_array().into_iter().flatten() {
            rows.push(json!({"record":"central","data":contact(row)}))
        }
        for row in module["overallOfficials"].as_array().into_iter().flatten() {
            rows.push(json!({"record":"official","data":{"name":row["name"],"affiliation":row["affiliation"],"role":row["role"]}}))
        }
    }
    for row in &mut rows {
        super::assertions::omit_empty_lists(row);
        super::assertions::omit_null_fields(
            row,
            &[
                "facility",
                "city",
                "state",
                "zip",
                "country",
                "status",
                "coordinates",
                "name",
                "role",
                "phone",
                "phone_ext",
                "email",
                "affiliation",
            ],
        );
    }
    rows
}
fn attachments() -> Check {
    // Synthetic response-only fields use the retained publisher metadata/enums.
    // Original records omit site status and contacts; those absences remain in walks.
    checked_steps(
        "trial/get/NCT02987543",
        |_, record| {
            if !record["url"]
                .as_str()
                .is_some_and(|u| u.ends_with("NCT02987543"))
            {
                return;
            }
            record["exchanges"][0]["body"]["value"]["protocolSection"]["contactsLocationsModule"] = json!(
                { "centralContacts":[{"name":"Central Contact","role":"CONTACT","phone":"555-0100","phoneExt":"123","email":"central@example.org"}],
                "overallOfficials":[{"name":"Trial Chair","role":"STUDY_CHAIR","affiliation":"Registry Organization"}],
                "locations":[{"facility":"First Facility","city":"First City","country":"United States","status":"RECRUITING","geoPoint":{"lat":90,
                "lon":180},"contacts":[{"name":"Site One","role":"PRINCIPAL_INVESTIGATOR","email":"one@example.org"}]},
                {"facility":"Second Facility","status":"SUSPENDED","geoPoint":{"lat":-90,"lon":-180},"contacts":[{"name":"Site Two",
                "role":"SUB_INVESTIGATOR","phone":"555-0102"}]}] }
            );
        },
        json!(
            [ {"run":["get","trial","NCT02987543","locations","--limit","1"],"mcp":["get","trial","NCT02987543","locations","--limit","1"],
            "expect":{ "/result/items/0/card/status":"COMPLETED","/result/items/0/sections/locations/data/total":2,
            "/result/items/0/sections/locations/data/items/0/status":"RECRUITING",
            "/result/items/0/sections/locations/data/items/0/contacts/0/name":"Site One",
            "/result/items/0/sections/locations/data/items/0/coordinates":{"equals":{"latitude":90.0,"longitude":180.0}}}},{"follow":"page",
            "offline":true,"expect":{"/result/items/0/sections/locations/data/offset":1,
            "/result/items/0/sections/locations/data/items/0/facility":"Second Facility",
            "/result/items/0/sections/locations/data/items/0/status":"SUSPENDED",
            "/result/items/0/sections/locations/data/items/0/contacts/0/name":"Site Two"}},{"run":["get","trial","NCT02987543","contacts"],
            "expect":{"/result/items/0/sections/contacts/data/items":[{"record":"central","data":{"name":"Central Contact","role":"CONTACT",
            "phone":"555-0100","phone_ext":"123","email":"central@example.org"}},{"record":"official","data":{"name":"Trial Chair",
            "affiliation":"Registry Organization","role":"STUDY_CHAIR"}}]}} ]
        ),
    )?;
    Ok(())
}
fn coordinates() -> Check {
    for point in [
        json!({"lat":90.01,"lon":0}),
        json!({"lat":0,"lon":-180.01}),
        json!({"lat":"NaN","lon":0}),
        json!({"lat":0}),
    ] {
        checked_steps(
            "trial/get/NCT03734029",
            |_, record| {
                if record["url"]
                    .as_str()
                    .is_some_and(|u| u.ends_with("NCT03734029"))
                {
                    record["exchanges"][0]["body"]["value"]["protocolSection"]["contactsLocationsModule"]
                        ["locations"][0]["geoPoint"] = point.clone();
                }
            },
            json!([
                {"run":["get","trial","NCT03734029","locations"],"expect":{
                    "/status":"partial","/result/items/0/card/nct_id":"NCT03734029",
                    "/result/items/0/sections/locations/data/items/0":{"omits":["coordinates"]},
                    "/result/items/0/sections/locations/data/items/0/facility":"Ironwood Cancer & Research Centers - Chandler II",
                    "/result/items/0/sections/locations/data/items/1/facility":"Banner MD Anderson Cancer Center",
                    "/problems/0/field":"geoPoint"}},
                {"run":["get","trial","NCT03734029","locations"],"offline":true,"expect":{
                    "/status":"partial","/result/items/0/sections/locations/data/items/0":{"omits":["coordinates"]},
                    "/problems/0/field":"geoPoint"}}
            ]),
        )?;
    }
    Ok(())
}
fn old_owner() -> Check {
    for version in [1, 2, 3] {
        old_version(version)?;
    }
    Ok(())
}
fn old_version(version: u32) -> Check {
    let env = Environment::new()?;
    let replay = env.root().join("recordings");
    copy_capture("trial/get/NCT03734029", &replay, |_, _| {})?;
    let args = ["get", "trial", "NCT03734029", "locations"].map(str::to_owned);
    let first = command(&env, &args, &env.globals(Some(&replay), false), 0)?;
    let next = super::action(&first, "page")?;
    // Reclassify this row as an old owner's incompatible payload. The key lookup
    // must reject it before decode, even though the bytes are intentionally invalid.
    let database = env.root().join("case-cache").join("replay.sqlite");
    let fetch = biomcp_fetch::Fetch::open(biomcp_fetch::FetchConfig {
        cache: Some(database),
        cache_owners: vec![biomcp_query::store::OWNER],
        mode: biomcp_fetch::Mode::Offline,
        ..Default::default()
    })?;
    let db = fetch.cache_access()?.connect()?;
    use sha2::{Digest, Sha256};
    let old_key =
        Sha256::digest(format!("biomcp.trial_snapshot/{version}\0NCT03734029").as_bytes()).to_vec();
    db.execute(
        "UPDATE trial_snapshot SET request_key=?1,payload='[\"old positional snapshot\"]'",
        [old_key],
    )?;
    let expired = command(&env, &next, &env.globals(None, true), 3)?;
    if expired["status"] != "expired" || super::action(&expired, "restart").is_err() {
        return Err("old snapshot did not expire with restart".into());
    }
    Ok(())
}
