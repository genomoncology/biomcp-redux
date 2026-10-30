//! Existing routine owners retain runtime pool IDs and assert source rows.
//! Protein feature/member owners and oversized trial text use authored synthetic data.
pub(crate) const PATHWAY_GENES: (&str, &[&str]) = (
    "pathway-reactome/native-reference-walk",
    &[
        "option pathway genes --limit",
        "option pathway genes --offset",
        "option pathway genes --pool",
    ],
);
pub(crate) const PATHWAY_SEARCH: (&str, &[&str]) = (
    "pathway-reactome/pool-bindings-before-source-io",
    &["option search pathway --offset"],
);
pub(crate) const ORGANIZATION: (&str, &[&str]) = (
    "organization-nci/native-local-pages",
    &[
        "option search organization --offset",
        "option search organization --pool",
    ],
);
pub(crate) const BIOMARKER: (&str, &[&str]) = (
    "biomarker-nci/native-snapshot-pages",
    &[
        "option search biomarker --limit",
        "option search biomarker --offset",
        "option search biomarker --pool",
    ],
);
pub(crate) const FEATURES: (&str, &[&str]) = (
    "protein-sections/displayed-bounds-and-identity",
    &[
        "option protein features --offset",
        "option protein features --pool",
    ],
);
pub(crate) const MEMBERS: (&str, &[&str]) = (
    "protein-domains-complexes/exact-members-and-retained-pages",
    &[
        "option protein complex-members --offset",
        "option protein complex-members --pool",
    ],
);
pub(crate) const TRIAL: (&str, &[&str]) = (
    "trial-get/get/checked-source-continuations",
    &[
        "option get trial --offset",
        "option get trial --pool",
        "failure trial expired-pool",
    ],
);
