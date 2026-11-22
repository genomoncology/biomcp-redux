//! The named tests that may exercise the private label section finder.
// These checks exercise the private section finder with shaped text or an extracted label PDF.
pub(super) fn allows(path: &str, item: Option<&[String]>) -> bool {
    const CASES: &[&str] = &[
        "table_rows_that_look_like_subsections_are_never_headings",
        "an_omitted_numbered_section_is_unread_and_a_bounded_opening_can_confirm_no_box",
        "a_section_whose_list_line_the_reader_lost_is_implied_by_its_subsections",
        "a_heading_whose_number_is_drawn_apart_is_found_by_its_listed_title",
        "a_boxed_warning_the_contents_list_leaves_out_is_read_from_the_body",
        "a_contents_list_numbered_with_full_stops_bounds_its_sections",
        "a_dotted_body_heading_missing_from_contents_leaves_the_section_unread",
        "a_two_column_contents_list_never_runs_a_section_over_the_ones_it_lost",
        "a_box_drawn_as_an_image_is_read_only_through_its_listed_title",
        "only_a_document_that_names_another_kind_is_not_a_label",
        "a_real_label_reads_into_bounded_sections",
        "a_label_missing_its_contents_is_read_from_its_numbered_headings",
        "older_labels_are_bounded_by_the_heading_that_always_follows",
        "numbered_highlights_do_not_bound_an_unnumbered_body",
    ];
    path == "crates/biomcp-query/src/drug/drugsfda/label_sections_tests.rs"
        && item.is_some_and(|item| {
            item.starts_with(&[
                "#".into(),
                "[".into(),
                "test".into(),
                "]".into(),
                "fn".into(),
            ]) && item
                .get(5)
                .is_some_and(|name| CASES.contains(&name.as_str()))
        })
}
