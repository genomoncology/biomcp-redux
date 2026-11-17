use biomcp_adapters::ctgov::criteria::split;

#[test]
fn partial_lines_use_their_actual_source_heading() {
    let text = concat!(
        "Inclusion Criteria:\nExclusion Criteria are mentioned\n",
        "* Item mentions Exclusion Criteria: here\nExclusion Criteria: continued item"
    );
    let first = "Inclusion Criteria:\nExclusion Criteria";
    let page = split(text, 0, first).expect("split");
    assert!(page.exclusion.is_none());
    let suffix = "Exclusion Criteria: here\n";
    let suffix_start = text.find(suffix).expect("suffix in fixture");
    let page = split(text, suffix_start, suffix).expect("split");
    assert_eq!(
        page.inclusion.as_deref(),
        Some([Box::from("Exclusion Criteria: here")].as_slice())
    );
    assert!(page.exclusion.is_none());
    let heading_end = text.len() - "continued item".len();
    let page = split(text, heading_end, "continued item").expect("split");
    assert_eq!(
        page.exclusion.as_deref(),
        Some([Box::from("continued item")].as_slice())
    );
}
