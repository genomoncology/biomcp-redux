//! Section parsing when the label gives no usable contents list.
use super::{Found, S, found, section};

#[test]
fn a_label_missing_its_contents_is_read_from_its_numbered_headings() {
    // Olumiant's 2018 original as the reader gives it: page 1, with the box and contents, and page
    // 3, with the box and section 1, are images; the list's last lines remain on page 2.
    let label = "This label may not be the latest approved by FDA. 2 14 CLINICAL STUDIES 16 HOW SUPPLIED \
        17 PATIENT COUNSELING INFORMATION * Sections or subsections omitted from the full prescribing \
        information are not listed. 4 CONTRAINDICATIONS None. 5 WARNINGS AND PRECAUTIONS 5.1 Serious \
        Infections Serious and sometimes fatal infections. 5.3 Thrombosis Thrombosis has occurred \
        [see Adverse Reactions (6.1)]. 6 ADVERSE REACTIONS The most common reactions were nausea. \
        7 DRUG INTERACTIONS Avoid probenecid. 8 USE IN SPECIFIC POPULATIONS Pregnancy data.";
    let warnings = section(label, S::WarningsAndCautions);
    assert!(
        warnings.starts_with("5 WARNINGS AND PRECAUTIONS") && warnings.ends_with("(6.1)]."),
        "{warnings}"
    );
    assert_eq!(
        section(label, S::Contraindications),
        "4 CONTRAINDICATIONS None."
    );
    // The box and section 1 are not in the text, and section 8 has no heading after it.
    for each in [
        S::BoxedWarning,
        S::IndicationsAndUsage,
        S::UseInSpecificPopulations,
    ] {
        assert_eq!(found(label, each), Found::Unbounded, "{each}");
    }
    // A section whose next heading the reader lost is unread, never run into the one after.
    let lost = label.replace("6 ADVERSE REACTIONS ", "");
    assert_eq!(found(&lost, S::WarningsAndCautions), Found::Unbounded);
    // A stretch of a contents list, whose body headings were lost, is no section.
    let listed = "HIGHLIGHTS OF PRESCRIBING INFORMATION 4 CONTRAINDICATIONS 5 WARNINGS AND PRECAUTIONS \
        5.1 Serious Infections 6 ADVERSE REACTIONS 6.1 Clinical Trials Experience Serious infections.";
    assert_eq!(found(listed, S::WarningsAndCautions), Found::Unbounded);
}

/// Lipitor's 2007 label as the reader gives it: headings run into their paragraphs, one behind
/// a page number, with cross-references in capitals.
const LIPITOR: &str = "DESCRIPTION Atorvastatin calcium is a synthetic agent (see CLINICAL PHARMACOLOGY). \
    INDICATIONS AND USAGE LIPITOR is indicated as an adjunct to diet. CONTRAINDICATIONS Active liver \
    disease. WARNINGS Liver Dysfunction Persistent elevations occurred (see DOSAGE AND ADMINISTRATION). \
    Myopathy, see ADVERSE REACTIONS and PRECAUTIONS, Drug Interactions. PRECAUTIONS General Rule out \
    secondary causes (see WARNINGS, Skeletal Muscle). 22 ADVERSE REACTIONS LIPITOR is generally \
    well-tolerated. Musculoskeletal System: leg cramps, myasthenia. OVERDOSAGE There is no specific \
    treatment. DOSAGE AND ADMINISTRATION Take once daily. HOW SUPPLIED Tablets.";

#[test]
fn older_labels_are_bounded_by_the_heading_that_always_follows() {
    let adverse = section(LIPITOR, S::AdverseReactions);
    assert!(
        adverse.starts_with("ADVERSE REACTIONS LIPITOR") && adverse.ends_with("myasthenia."),
        "{adverse}"
    );
    let warnings = section(LIPITOR, S::WarningsAndCautions);
    assert!(
        warnings.starts_with("WARNINGS Liver") && warnings.ends_with("Skeletal Muscle). 22"),
        "warnings run through precautions to adverse reactions: {warnings}"
    );
    assert_eq!(
        section(LIPITOR, S::Contraindications),
        "CONTRAINDICATIONS Active liver disease."
    );
    assert_eq!(found(LIPITOR, S::BoxedWarning), Found::Older);
    // A heading the reader lost leaves the section before it unread, never run into the next.
    let lost = LIPITOR.replace("22 ADVERSE REACTIONS LIPITOR", "LIPITOR");
    assert_eq!(found(&lost, S::WarningsAndCautions), Found::Unbounded);
    // A heading printed twice leaves its section unread.
    let twice = LIPITOR.replace("treatment. DOSAGE", "treatment. ADVERSE REACTIONS DOSAGE");
    assert_eq!(found(&twice, S::AdverseReactions), Found::Unbounded);
}
