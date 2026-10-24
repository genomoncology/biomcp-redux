//! Variant and trial Markdown shapes reuse the existing isolated command contract.
use super::markdown_shapes::{Shape, check};
use libtest_mimic::Trial;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials: Vec<_> = SHAPES
        .iter()
        .map(|(name, shape)| {
            Trial::test(format!("markdown-view/{name}"), move || {
                check(shape).map_err(Into::into)
            })
        })
        .collect();
    trials.extend(super::markdown_variant_sources::trials());
    trials
}
// Initial bounds deliberately fail so each retained replay measures its rendered shape.
const SHAPES: &[(&str, Shape)] = &[
    (
        "variant-search",
        Shape {
            args: &[
                "search",
                "variant",
                "--gene",
                "EGFR",
                "--protein",
                "L858R",
                "--build",
                "GRCh37",
            ],
            replies: "variant/search/L858R",
            required: &["## Result", "EGFR", "L858R", "|"],
            max_bytes: 1,
        },
    ),
    (
        "variant-card",
        Shape {
            args: &["get", "variant", "rs113488022"],
            replies: "variant/synthetic-components",
            required: &["## Result", "rs113488022", "|"],
            max_bytes: 1,
        },
    ),
    (
        "variant-predict",
        Shape {
            args: &["get", "variant", "rs113488022", "predict"],
            replies: "variant/synthetic-components",
            required: &["## Result", " Predict\n\n", "**state:** data", "data"],
            max_bytes: 1,
        },
    ),
    (
        "variant-conservation",
        Shape {
            args: &["get", "variant", "rs113488022", "conservation"],
            replies: "variant/synthetic-components",
            required: &["## Result", " Conservation\n\n", "**state:** data", "data"],
            max_bytes: 1,
        },
    ),
    (
        "variant-cosmic",
        Shape {
            args: &["get", "variant", "rs113488022", "cosmic"],
            replies: "variant/synthetic-components",
            required: &["## Result", " Cosmic\n\n", "**state:** data", "data"],
            max_bytes: 1,
        },
    ),
    (
        "variant-clinvar",
        Shape {
            args: &["get", "variant", "clinvar:13961", "clinvar"],
            replies: "variant/assertions/domains",
            required: &["## Result", " Clinvar\n\n", "**state:** data", "SCV"],
            max_bytes: 1,
        },
    ),
    (
        "variant-erepo",
        Shape {
            args: &["get", "variant", "CA000434", "erepo"],
            replies: "variant/assertions/erepo-current",
            required: &[
                "## Result",
                " Erepo\n\n",
                "**state:** data",
                "PCER000001171",
            ],
            max_bytes: 1,
        },
    ),
    (
        "variant-population",
        Shape {
            args: &["get", "variant", "CA123643", "population"],
            replies: "variant-population/main",
            required: &["## Result", " Population\n\n", "**state:** data", "gnomad"],
            max_bytes: 1,
        },
    ),
    (
        "variant-gwas",
        Shape {
            args: &["get", "variant", "rs113488022", "gwas"],
            replies: "variant-population/gwas-positive",
            required: &["## Result", " Gwas\n\n", "**state:** data", "gwas"],
            max_bytes: 1,
        },
    ),
    (
        "variant-civic-skipped",
        Shape {
            args: &[
                "get",
                "variant",
                "rs113488022",
                "civic",
                "--disable-sources",
                "civic",
            ],
            replies: "variant/synthetic-components",
            required: &["## Result", "civic", "skipped", "disabled"],
            max_bytes: 1,
        },
    ),
    (
        "variant-hotspots-skipped",
        Shape {
            args: &[
                "get",
                "variant",
                "rs113488022",
                "hotspots",
                "--disable-sources",
                "hotspots",
            ],
            replies: "variant/synthetic-components",
            required: &["## Result", "hotspots", "skipped", "disabled"],
            max_bytes: 1,
        },
    ),
    (
        "variant-oncokb-skipped",
        Shape {
            args: &[
                "get",
                "variant",
                "rs113488022",
                "oncokb",
                "--disable-sources",
                "oncokb",
            ],
            replies: "variant/synthetic-components",
            required: &["## Result", "oncokb", "skipped", "disabled"],
            max_bytes: 1,
        },
    ),
    (
        "variant-normalize",
        Shape {
            args: &[
                "variant",
                "normalize",
                "--source",
                "mutalyzer",
                "--hgvs",
                "NM_004333.4:c.1799T>A",
            ],
            replies: "variant/normalization/mutalyzer",
            required: &["## Result", "mutalyzer", "NM_004333.4"],
            max_bytes: 1,
        },
    ),
    (
        "variant-population-detail",
        Shape {
            args: &["variant", "population", "CA123643", "--detail"],
            replies: "variant-population/main",
            required: &["## Result", "gnomad"],
            max_bytes: 1,
        },
    ),
    (
        "variant-assertions",
        Shape {
            args: &["variant", "assertions", "CA000434", "--source", "erepo"],
            replies: "variant/assertions/erepo-current",
            required: &["## Result", "PCER000001171"],
            max_bytes: 1,
        },
    ),
    (
        "variant-recurrence",
        Shape {
            args: &[
                "variant",
                "recurrence",
                "NC_000007.13:g.140494253G>A",
                "--source",
                "cbioportal",
                "--gene",
                "BRAF",
                "--study",
                "msk_impact_2017",
                "--profile",
                "msk_impact_2017_mutations",
                "--sample-set",
                "msk_impact_2017_all",
                "--limit",
                "3",
            ],
            replies: "variant/recurrence",
            required: &["## Result", "cbioportal"],
            max_bytes: 1,
        },
    ),
    (
        "trial-search",
        Shape {
            args: &["search", "trial", "--condition", "melanoma"],
            replies: "trial/condition",
            required: &["## Result", "NCT", "|"],
            max_bytes: 1,
        },
    ),
    (
        "trial-card",
        Shape {
            args: &["get", "trial", "NCT04303780"],
            replies: "trial/get/NCT04303780",
            required: &["## Result", "NCT04303780", "|"],
            max_bytes: 1,
        },
    ),
    (
        "trial-locations",
        Shape {
            args: &["get", "trial", "NCT04303780", "locations"],
            replies: "trial/get/NCT04303780",
            required: &["## Result", " Locations\n\n", "**state:** data", "data"],
            max_bytes: 1,
        },
    ),
    (
        "trial-contacts",
        Shape {
            args: &["get", "trial", "NCT04303780", "contacts"],
            replies: "trial/get/NCT04303780",
            required: &["## Result", " Contacts\n\n", "**state:** data", "data"],
            max_bytes: 1,
        },
    ),
    (
        "trial-eligibility",
        Shape {
            args: &["get", "trial", "NCT04303780", "eligibility"],
            replies: "trial/get/NCT04303780",
            required: &["## Result", " Eligibility\n\n", "**state:** data", "data"],
            max_bytes: 1,
        },
    ),
    (
        "trial-outcomes",
        Shape {
            args: &["get", "trial", "NCT04303780", "outcomes"],
            replies: "trial/get/NCT04303780",
            required: &["## Result", " Outcomes\n\n", "**state:** data", "data"],
            max_bytes: 1,
        },
    ),
    (
        "trial-arms",
        Shape {
            args: &["get", "trial", "NCT04303780", "arms"],
            replies: "trial/get/NCT04303780",
            required: &["## Result", " Arms\n\n", "**state:** data", "data"],
            max_bytes: 1,
        },
    ),
    (
        "trial-documents",
        Shape {
            args: &["get", "trial", "NCT04303780", "documents"],
            replies: "trial/get/NCT04303780",
            required: &["## Result", " Documents\n\n", "**state:** data", "data"],
            max_bytes: 1,
        },
    ),
    (
        "trial-references",
        Shape {
            args: &["get", "trial", "NCT03511664", "references"],
            replies: "trial/get/NCT03511664",
            required: &["## Result", " References\n\n", "**state:** data", "data"],
            max_bytes: 1,
        },
    ),
];
