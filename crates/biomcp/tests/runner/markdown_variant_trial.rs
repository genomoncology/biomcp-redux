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
// Ceilings are ceil(measured comparable Markdown bytes * 1.15), from j11380.
// Card and ClinVar ceilings use j16521; other shapes retain j11380.
// The existing checker subtracts only the validated new source terms columns.
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
            required: &["## Result", "EGFR", "L858R", "chr7:g.55259515T&gt;G"],
            max_bytes: 1306,
        },
    ),
    (
        "variant-card",
        Shape {
            args: &["get", "variant", "rs113488022"],
            replies: "variant/synthetic-components",
            required: &["## Result", "rs113488022", "chr7:g.140453136A&gt;G", "BRAF"],
            max_bytes: 2608,
        },
    ),
    (
        "variant-predict",
        Shape {
            args: &["get", "variant", "rs113488022", "predict"],
            replies: "variant/synthetic-components",
            required: &[
                "cadd&#46;phred",
                "0.25",
                "## Result",
                " Predict\n\n",
                "**state:** data",
                "data",
            ],
            max_bytes: 9861,
        },
    ),
    (
        "variant-conservation",
        Shape {
            args: &["get", "variant", "rs113488022", "conservation"],
            replies: "variant/synthetic-components",
            required: &[
                "dbnsfp&#46;gerp++&#46;nr",
                "1.5",
                "## Result",
                " Conservation\n\n",
                "**state:** data",
                "data",
            ],
            max_bytes: 11070,
        },
    ),
    (
        "variant-cosmic",
        Shape {
            args: &["get", "variant", "rs113488022", "cosmic"],
            replies: "variant/synthetic-components",
            required: &[
                "COSM999999",
                "unverified",
                "## Result",
                " Cosmic\n\n",
                "**state:** data",
                "data",
            ],
            max_bytes: 5713,
        },
    ),
    (
        "variant-clinvar",
        Shape {
            args: &["get", "variant", "clinvar:13961", "clinvar"],
            replies: "variant/assertions/domains",
            required: &[
                "## Result",
                " Clinvar\n\n",
                "**state:** data",
                "VCV000013961",
            ],
            max_bytes: 1691,
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
            max_bytes: 1185,
        },
    ),
    (
        "variant-population",
        Shape {
            args: &["get", "variant", "CA123643", "population"],
            replies: "variant-population/main",
            required: &[
                "1460618",
                "7-140753336-A-T",
                "## Result",
                " Population\n\n",
                "**state:** data",
                "gnomad",
            ],
            max_bytes: 1301,
        },
    ),
    (
        "variant-gwas",
        Shape {
            args: &["get", "variant", "rs113488022", "gwas"],
            replies: "variant-population/gwas-positive",
            required: &[
                "GCST90996374",
                "40210729",
                "## Result",
                " Gwas\n\n",
                "**state:** data",
                "gwas",
            ],
            max_bytes: 1996,
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
            max_bytes: 601,
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
            max_bytes: 618,
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
                "--build",
                "GRCh38",
                "--tumor-type",
                "Synthetic tumor",
                "--disable-sources",
                "oncokb",
            ],
            replies: "variant/synthetic-components",
            required: &["## Result", "oncokb", "skipped", "disabled"],
            max_bytes: 667,
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
            required: &["## Result", "mutalyzer", "NM\\_004333.4"],
            max_bytes: 845,
        },
    ),
    (
        "variant-population-detail",
        Shape {
            args: &["variant", "population", "CA123643", "--detail"],
            replies: "variant-population/main",
            required: &["1460618", "7-140753336-A-T", "## Result", "gnomad"],
            max_bytes: 1777,
        },
    ),
    (
        "variant-assertions",
        Shape {
            args: &["variant", "assertions", "CA000434", "--source", "erepo"],
            replies: "variant/assertions/erepo-current",
            required: &["## Result", "PCER000001171"],
            max_bytes: 832,
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
            required: &["msk\\_impact\\_2017", "## Result", "cbioportal"],
            max_bytes: 1875,
        },
    ),
    (
        "trial-search",
        Shape {
            args: &["search", "trial", "--condition", "melanoma"],
            replies: "trial/condition",
            required: &["## Result", "NCT04631731", "NCT01874119", "|"],
            max_bytes: 4527,
        },
    ),
    (
        "trial-card",
        Shape {
            args: &["get", "trial", "NCT04303780"],
            replies: "trial/get/NCT04303780",
            required: &["## Result", "NCT04303780", "CodeBreak 200", "|"],
            max_bytes: 1821,
        },
    ),
    (
        "trial-locations",
        Shape {
            args: &["get", "trial", "NCT04303780", "locations"],
            replies: "trial/get/NCT04303780",
            required: &[
                "Pacific Cancer Medical Center Inc",
                "Anaheim",
                "## Result",
                " Locations\n\n",
                "**state:** data",
                "data",
            ],
            max_bytes: 3962,
        },
    ),
    (
        "trial-contacts",
        Shape {
            args: &["get", "trial", "NCT04303780", "contacts"],
            replies: "trial/get/NCT04303780",
            required: &[
                "Amgen",
                "MD",
                "## Result",
                " Contacts\n\n",
                "**state:** data",
                "data",
            ],
            max_bytes: 2107,
        },
    ),
    (
        "trial-eligibility",
        Shape {
            args: &["get", "trial", "NCT04303780", "eligibility"],
            replies: "trial/get/NCT04303780",
            required: &[
                "Inclusion Criteria:",
                "ECOG ≤ 1",
                "## Result",
                " Eligibility\n\n",
                "**state:** data",
                "data",
            ],
            max_bytes: 2805,
        },
    ),
    (
        "trial-outcomes",
        Shape {
            args: &["get", "trial", "NCT04303780", "outcomes"],
            replies: "trial/get/NCT04303780",
            required: &[
                "Progression-free Survival (PFS)",
                "## Result",
                " Outcomes\n\n",
                "**state:** data",
                "data",
            ],
            max_bytes: 2919,
        },
    ),
    (
        "trial-arms",
        Shape {
            args: &["get", "trial", "NCT04303780", "arms"],
            replies: "trial/get/NCT04303780",
            required: &[
                "AMG 510",
                "Docetaxel",
                "## Result",
                " Arms\n\n",
                "**state:** data",
                "data",
            ],
            max_bytes: 2621,
        },
    ),
    (
        "trial-documents",
        Shape {
            args: &["get", "trial", "NCT04303780", "documents"],
            replies: "trial/get/NCT04303780",
            required: &[
                "SAP\\_001.pdf",
                "557905",
                "## Result",
                " Documents\n\n",
                "**state:** data",
                "data",
            ],
            max_bytes: 2619,
        },
    ),
    (
        "trial-references",
        Shape {
            args: &["get", "trial", "NCT03511664", "references"],
            replies: "trial/get/NCT03511664",
            required: &[
                "34161051",
                "RESULT",
                "## Result",
                " References\n\n",
                "**state:** data",
                "data",
            ],
            max_bytes: 7650,
        },
    ),
    (
        "trial-document-save",
        Shape {
            args: &[
                "trial",
                "document",
                "NCT04303780",
                "SAP_001.pdf",
                "--save",
                "protocol.pdf",
                "--replace",
            ],
            replies: "trial/documents/synthetic",
            required: &[
                "## Result",
                "SAP\\_001.pdf",
                "**bytes:** 67",
                "**state:** data",
            ],
            max_bytes: 858,
        },
    ),
];
