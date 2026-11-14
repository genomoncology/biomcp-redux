//! The first three names begin as in NCBI's GTR test_version.gz (2026-11-05); the others are made up.
use super::{contains, needle, whole_word};

#[test]
fn query_spaces_and_hyphens_are_optional_and_name_hyphens_are_ignored() {
    for name in [
        "Nonsmall cell lung cancer, somatic (BRAF gene)",
        "NonSmall Cell Lung Cancer Genetic Panel",
        "Non-small Cell Lung Carcinoma Panel",
    ] {
        assert!(contains(name, "non-small cell"), "{name}");
    }
    assert!(contains("Non-small Cell Lung Carcinoma Panel", "nonsmall cell"));
    assert!(contains("BRCA1 Hereditary Cancer Panel", "BRCA 1"));
    assert!(contains("T-Cell Receptor Gene Rearrangement", "T cell"));
}

#[test]
fn a_name_space_is_never_skipped_where_the_query_had_none() {
    assert!(!contains("Amniotic Fluid Culture", "CF"));
    assert!(!contains("Hepatic failure panel", "CF"));
    assert!(!contains("BRCA 1 synthetic panel", "BRCA1"));
    assert!(contains("CF Mutation Panel", "CF"));
}

#[test]
fn whole_words_are_judged_on_the_source_name() {
    let query = needle("non-small cell").unwrap_or_default();
    assert!(whole_word("NonSmall Cell Lung Cancer Genetic Panel", &query));
    let query = needle("small cell").unwrap_or_default();
    assert!(!whole_word("Nonsmall cell lung cancer", &query));
    assert!(whole_word("Non-small cell lung cancer", &query));
}

#[test]
fn text_of_only_spaces_and_hyphens_matches_nothing() {
    assert!(needle(" - ").is_none());
    assert!(!contains("Any test name", "-"));
}
