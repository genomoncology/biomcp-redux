//! Feature gene-sections: malformed native metadata preserves source submission identity.
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![super::super::transcript::trial(
        "gene-sections/gencc-invalid-date-keeps-submission",
        || gencc().map_err(|error| error.to_string().into()),
    )]
}
fn gencc() -> Result<(), Box<dyn std::error::Error>> {
    let value = super::super::response_cases::checked_steps(
        "gene/gencc",
        |source, recording| {
            if source != "gencc" {
                return;
            }
            let body = recording.pointer_mut("/exchanges/1/body/value").unwrap();
            *body = json!(body.as_str().unwrap().replacen("2021-08-04 12:08:26", "   ", 1));
        },
        json!([
            {"run":["sync","gencc"],"exit":0,"expect":{"/result/rows":3}},
            {"run":["get","gene","ODC1","gencc","--disable-sources","oncokb"],"exit":0,
             "expect":{
                 "/result/items/0/sections/gencc/data/submissions":{"count":3},
                 "/result/items/0/sections/gencc/data/submissions/0/id":"SGC-113621",
                 "/result/items/0/sections/gencc/data/submissions/0":{"omits":["evaluated_date"]},
                 "/problems":{"count":1},
                 "/problems/0/field":"submitted_as_date"
             }}
        ]),
    )?;
    if value.pointer("/problems/0/record") != Some(&json!("SGC-113621")) {
        return Err("GenCC field problem lost its source record".into());
    }
    Ok(())
}
