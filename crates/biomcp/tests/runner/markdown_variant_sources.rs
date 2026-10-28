//! Positive section views reuse the existing synthetic source responders.
// Ceilings are ceil(j11380 measured comparable Markdown bytes * 1.15).
use super::{
    launch::Environment,
    markdown_shapes::{Shape, check_environment},
    network_stub::Stub,
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::json;
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    ["civic", "hotspots", "oncokb"]
        .into_iter()
        .map(|source| {
            transcript_trial(format!("markdown-view/variant-{source}-data"), move || {
                positive(source).map_err(Into::into)
            })
        })
        .collect()
}
type Reply = fn(&str) -> Result<String, String>;
fn positive(source: &str) -> Result<(), String> {
    let (hosts, reply): (&[&str], Reply) = match source {
        "civic" => (&["civicdb.org"], super::variant_civic::civic_body),
        "hotspots" => (
            &["myvariant.info", "www.cancerhotspots.org"],
            super::variant_hotspots::body,
        ),
        _ => (&["myvariant.info", "www.oncokb.org"], super::oncokb::reply),
    };
    let stub = Stub::for_sources(hosts, |_| "application/json", reply)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("synthetic-ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca,
        "keys":{"oncokb_api_key":"synthetic-markdown-key"}});
    let shape = match source {
        "civic" => Shape {
            args: &[
                "get",
                "variant",
                "CA123643",
                "civic",
                "--disable-sources",
                "myvariant",
            ],
            replies: "",
            required: &[
                "## Result",
                " Civic\n\n",
                "**state:** data",
                "BRAF V600E",
                "dabrafenib",
            ],
            max_bytes: 4699,
        },
        "hotspots" => Shape {
            args: &[
                "get",
                "variant",
                "chr7:g.140753336A>T",
                "--build",
                "GRCh38",
                "hotspots",
            ],
            replies: "",
            required: &[
                "## Result",
                " Hotspots\n\n",
                "**state:** data",
                "ENST00000288602",
                "V600",
                "candidate",
            ],
            max_bytes: 2034,
        },
        _ => Shape {
            args: &[
                "get",
                "variant",
                "chr7:g.140753336A>T",
                "oncokb",
                "--build",
                "GRCh38",
                "--tumor-type",
                "Synthetic tumor",
                "--section-limit",
                "50",
            ],
            replies: "",
            required: &[
                "## Result",
                " Oncokb\n\n",
                "**state:** data",
                "Oncogenic",
                "Synthetic mutation effect&#46;",
            ],
            max_bytes: 2470,
        },
    };
    check_environment(&shape, &env, &[], config)
}
