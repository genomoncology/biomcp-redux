//! Existing mechanical trials use authored synthetic data, never native captures.
pub(crate) const GWAS_SELECTORS: (&str, &[&str]) = (
    "gwas-search/checked-native-selectors-and-aliases",
    &[
        "option search gwas --extended-geneset",
        "option search gwas --gene",
        "option search gwas --include-child-traits",
        "option search gwas --limit",
        "option search gwas --study",
        "option search gwas --trait",
        "option search gwas --trait-id",
    ],
);
pub(crate) const GWAS_PAGES: (&str, &[&str]) = (
    "gwas-search/native-offset-and-continuation",
    &["option search gwas --offset"],
);
pub(crate) const GWAS_POOL: (&str, &[&str]) = (
    "gwas-search/expired-threshold-restarts-original-selectors",
    &["option search gwas --pool"],
);
pub(crate) const GWAS_ANCESTRY: (&str, &[&str]) = (
    "gwas-get/ancestry-walk-ownership-and-source-policy",
    &[
        "section get gwas ancestries",
        "option get gwas --section-limit",
        "option get gwas --section-offset",
    ],
);
pub(crate) const GWAS_ASSOCIATIONS: (&str, &[&str]) = (
    "gwas-get/loci-and-native-study-page-actions",
    &["section get gwas associations"],
);
pub(crate) const STUDY_PAGES: (&str, &[&str]) = (
    "study-get/complete-pages-and-native-counts",
    &[
        "section get study profiles",
        "section get study sample-lists",
        "section get study all",
        "option get study --section-limit",
        "option get study --section-offset",
    ],
);
pub(crate) const STUDY_FILES: (&str, &[&str]) = (
    "study-files/complete-walk-and-notice",
    &["section get study files"],
);
pub(crate) const STUDY_SEARCH: (&str, &[&str]) = (
    "study-catalog/literal-pages-and-selectors",
    &["option search study --offset", "option search study --pool"],
);
pub(crate) const CPIC_VIEW: (&str, &[&str]) = (
    "cpic-pgx/view-network-cache-replay",
    &[
        "section get pgx recommendations",
        "option get pgx --section-limit",
        "option get pgx --section-offset",
    ],
);
pub(crate) const CPIC_TABLE: (&str, &[&str]) = (
    "cpic-pgx/table-network-cache-replay",
    &["option get pgx --drug-id"],
);
pub(crate) const CPIC_GUIDELINES: (&str, &[&str]) = (
    "cpic-pgx/guidelines-network-cache-replay",
    &["section get pgx guidelines"],
);
pub(crate) const CPIC_DIPLOTYPE: (&str, &[&str]) = (
    "cpic-pgx/diplotype-network-cache-replay",
    &["section get pgx diplotype"],
);
pub(crate) const CPIC_PAIRS: (&str, &[&str]) = (
    "cpic-pgx/pairs-network-cache-replay",
    &["option search pgx --offset"],
);
pub(crate) const CLINPGX_CLINICAL: (&str, &[&str]) = (
    "clinpgx-annotations/clinical-network-cache-replay",
    &["section get pgx annotations", "option get pgx --snapshot"],
);
pub(crate) const CLINPGX_KIND: (&str, &[&str]) = (
    "clinpgx-annotations/guideline-network-cache-replay",
    &["option get pgx --kind"],
);
pub(crate) const CLINPGX_ANNOTATION: (&str, &[&str]) = (
    "clinpgx-annotations/annotation-network-cache-replay",
    &["option get pgx --annotation-id"],
);
