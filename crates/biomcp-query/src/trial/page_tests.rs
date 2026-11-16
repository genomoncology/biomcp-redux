//! Shape check: an earlier refusal survives a clean ClinicalTrials.gov page (synthetic page).
use super::{apply_page, ctgov, empty};
use biomcp_types::TrialSearch;

type Check = Result<(), Box<dyn std::error::Error>>;

#[test]
fn earlier_refusal_survives_a_clean_page() -> Check {
    let query: TrialSearch = serde_json::from_str(r#"{"query":"melanoma"}"#)?;
    let mut outcome = empty(&query);
    outcome.dropped = true;
    let page = ctgov::SearchPage {
        refusals: biomcp_adapters::Refusal::None,
        received: 0,
        items: Vec::new(),
        problems: Vec::new(),
        condition_word_mismatch: false,
        gene_contexts: Vec::new(),
        codon_only: Vec::new(),
        codon_error: None,
        total: Some(0),
        count_error: None,
        count_request: None,
        next: None,
    };
    apply_page(&query, &query, false, None, &mut outcome, page);
    assert!(outcome.dropped, "a clean page cleared an earlier refusal");
    Ok(())
}
