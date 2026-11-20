//! Replays where the holder search and the later SPL read disagree.
use crate::runner::{response_cases, transcript};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> impl Iterator<Item = Trial> {
    [
        ("drug-name-get/holder-read-missing", true),
        ("drug-name-get/holder-read-wrong-application", false),
    ]
    .into_iter()
    .map(|(name, missing)| transcript::trial(name, move || fallback(missing)))
}

fn fallback(missing: bool) -> Result<(), Box<dyn std::error::Error>> {
    let args = [
        "get",
        "drug",
        "Hadlima",
        "label",
        "--source",
        "openfda-label",
        "--label-section",
        "how_supplied",
    ];
    response_cases::checked_steps(
        "drug/label-holder-recovery",
        |_, recording| {
            if !recording
                .get("url")
                .and_then(Value::as_str)
                .is_some_and(|url| {
                    url.contains("openfda.spl_set_id%3A%22c24655e5-796e-4127-b716-92f08d649c83")
                })
            {
                return;
            }
            let Some(reply) = recording.pointer_mut("/exchanges/0/body/value") else {
                return;
            };
            if missing {
                reply["meta"]["results"]["total"] = json!(0);
                reply["results"] = json!([]);
            } else {
                reply["results"][0]["openfda"]["application_number"] = json!(["BLA761219"]);
            }
        },
        json!([{
            "run": args, "mcp": args,
            "expect": {
                "/status": "complete",
                "/result/identity/id": "ef25af7e-6267-409a-8d68-de51eb48812a",
                "/result/identity/application_numbers/0": "BLA761059",
                "/result/identity/name_resolution/total": 2,
                "/result/identity/name_resolution/alternatives/0/args/2": "f7f9da92-ff5d-4260-b4bd-e3f6993a36ae",
                "/result/label/data/text": "Synthetic packaging for HADLIMA.",
                "/sources/0/notices/*/message": {"includes":
                    "The holder label for BLA761059 could not be read. The reply uses a repackager's copy."}
            }
        }]),
    )?;
    Ok(())
}
