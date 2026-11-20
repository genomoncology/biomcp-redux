//! Section bounds on label text shaped like the 1066 review's wrong cases: table rows that look
//! like subsections, headings whose number the PDF drew apart, boxed warnings the contents list
//! leaves out, a label missing its first page, and documents that are not labels; and, for 1082,
//! labels read from their headings alone, where a missed or repeated heading leaves a section
//! unread and never runs it into the next.
use super::label_sections::{Found, find};
use super::label_text::{clean, flatten};
use biomcp_types::LabelHistorySection as S;

const HIGHLIGHTS: &str = "HIGHLIGHTS OF PRESCRIBING INFORMATION These highlights do not include all \
    the information needed to use DRUG safely and effectively. ----INDICATIONS AND USAGE---- DRUG is \
    indicated for melanoma (1.1). ----WARNINGS AND PRECAUTIONS---- Immune-mediated reactions (5.1).";
const OMITTED: &str =
    "*Sections or subsections omitted from the full prescribing information are not listed.";

fn found(text: &str, section: S) -> Found {
    find(&flatten(&clean(text)), section)
}

fn section(text: &str, section: S) -> String {
    match found(text, section) {
        Found::Section(text) => text,
        other => format!("{other:?}"),
    }
}

/// A Keytruda-shaped label: contents, then a body whose adverse reactions hold table rows.
fn keytruda() -> String {
    format!(
        "{HIGHLIGHTS} FULL PRESCRIBING INFORMATION: CONTENTS* 1 INDICATIONS AND USAGE 1.1 Melanoma \
         1.2 Non-Small Cell Lung Cancer 1.3 Malignant Pleural Mesothelioma 2 DOSAGE AND ADMINISTRATION \
         2.1 Patient Selection 4 CONTRAINDICATIONS 5 WARNINGS AND PRECAUTIONS 5.1 Severe and Fatal \
         Immune-Mediated Adverse Reactions 6 ADVERSE REACTIONS 6.1 Clinical Trials Experience 8 USE IN \
         SPECIFIC POPULATIONS 8.1 Pregnancy 11 DESCRIPTION {OMITTED}\n\
         FULL PRESCRIBING INFORMATION\n1 INDICATIONS AND USAGE\n1.1 Melanoma\nKEYTRUDA is indicated \
         for unresectable or metastatic melanoma.\n1.3 Malignant Pleural Mesothelioma\nKEYTRUDA, with \
         pemetrexed and platinum chemotherapy, is indicated for malignant pleural mesothelioma.\n\
         2 DOSAGE AND ADMINISTRATION\n2.1 Patient Selection\nSelect patients by tumor type.\n\
         4 CONTRAINDICATIONS\nNone.\n5 WARNINGS AND PRECAUTIONS\n5.1 Severe and Fatal Immune-Mediated \
         Adverse Reactions\nImmune-mediated reactions may be fatal [see ADVERSE REACTIONS (6.1)].\n\
         6 ADVERSE REACTIONS\n6.1 Clinical Trials Experience\nTable 3: Laboratory Abnormalities\n\
         Increased AST 28 3.1 25 1.9\n1.1 Rash 26 2\n5.1 Neutropenia 35\n4.1 Study 3\n\
         9.1 Peripheral edema 12\nPostmarketing: sclerosing cholangitis.\n8 USE IN SPECIFIC POPULATIONS\n\
         8.1 Pregnancy\nKEYTRUDA can cause fetal harm.\n11 DESCRIPTION\nPembrolizumab is an antibody.\n"
    )
}

#[test]
fn table_rows_that_look_like_subsections_are_never_headings() {
    let label = keytruda();
    let indications = section(&label, S::IndicationsAndUsage);
    assert!(
        indications.starts_with("1 INDICATIONS AND USAGE"),
        "{indications}"
    );
    assert!(
        indications.contains("malignant pleural mesothelioma"),
        "{indications}"
    );
    assert!(
        indications
            .ends_with("platinum chemotherapy, is indicated for malignant pleural mesothelioma.")
    );
    assert_eq!(
        section(&label, S::Contraindications),
        "4 CONTRAINDICATIONS None."
    );
    let adverse = section(&label, S::AdverseReactions);
    assert!(adverse.starts_with("6 ADVERSE REACTIONS"), "{adverse}");
    assert!(adverse.contains("9.1 Peripheral edema 12 Postmarketing: sclerosing cholangitis."));
    assert!(!adverse.contains("Pregnancy"), "{adverse}");
    let warnings = section(&label, S::WarningsAndCautions);
    assert!(
        warnings.ends_with("[see ADVERSE REACTIONS (6.1)]."),
        "a cross-reference is no heading: {warnings}"
    );
}

#[test]
fn a_section_the_contents_list_leaves_out_is_absent_and_the_box_too() {
    let label = keytruda();
    assert_eq!(found(&label, S::DrugInteractions), Found::Absent);
    assert_eq!(found(&label, S::BoxedWarning), Found::Absent);
}

#[test]
fn a_section_whose_list_line_the_reader_lost_is_implied_by_its_subsections() {
    // Keytruda's 2021 label (S102): the reader lost the list's `1 INDICATIONS AND USAGE` line.
    let label = keytruda().replace(
        "CONTENTS* 1 INDICATIONS AND USAGE 1.1 Melanoma",
        "CONTENTS* 1.1 Melanoma",
    );
    let indications = section(&label, S::IndicationsAndUsage);
    assert!(
        indications.contains("malignant pleural mesothelioma"),
        "{indications}"
    );
    // A section the list lacks but the body heads is unread, never absent.
    let lost = keytruda().replace(" 4 CONTRAINDICATIONS 5 WARNINGS", " 5 WARNINGS");
    assert_eq!(found(&lost, S::Contraindications), Found::Unbounded);
}

#[test]
fn a_heading_whose_number_is_drawn_apart_is_found_by_its_listed_title() {
    // Livalo's 2024 label: the body draws its section numbers apart from their titles.
    let label = format!(
        "{HIGHLIGHTS} FULL PRESCRIBING INFORMATION: CONTENTS* 1 INDICATIONS AND USAGE 2 DOSAGE AND \
         ADMINISTRATION 5 WARNINGS AND PRECAUTIONS 5.1 Myopathy and Rhabdomyolysis 6 ADVERSE REACTIONS \
         6.1 Clinical Studies Experience 6.2 Postmarketing Experience 7 DRUG INTERACTIONS 8 USE IN \
         SPECIFIC POPULATIONS 8.1 Pregnancy {OMITTED} 1\nFULL PRESCRIBING INFORMATION\nINDICATIONS AND \
         USAGE\nLIVALO is indicated as an adjunct to diet.\n2 DOSAGE AND ADMINISTRATION\nTake orally.\n\
         5 WARNINGS AND PRECAUTIONS\n5.1 Myopathy and Rhabdomyolysis\nLIVALO may cause myopathy.\n\
         ADVERSE REACTIONS\nThe following serious adverse reactions are discussed elsewhere.\n2.8 3.1 \
         Constipation 1.9 3.6\n6.2 Postmarketing Experience\nrare reports of myasthenia gravis.\n\
         DRUG INTERACTIONS\n7\nTable 2 lists interacting drugs.\nUSE IN SPECIFIC POPULATIONS\n8.1 \
         Pregnancy\nDiscontinue LIVALO.\n"
    );
    let adverse = section(&label, S::AdverseReactions);
    assert!(adverse.starts_with("ADVERSE REACTIONS"), "{adverse}");
    assert!(
        adverse.contains("myasthenia gravis"),
        "the table row does not end it: {adverse}"
    );
    assert!(!adverse.contains("Table 2"), "{adverse}");
    let indications = section(&label, S::IndicationsAndUsage);
    assert!(
        indications.starts_with("INDICATIONS AND USAGE LIVALO"),
        "{indications}"
    );
}

#[test]
fn a_boxed_warning_the_contents_list_leaves_out_is_read_from_the_body() {
    // Xeljanz's 2012 label lists no warning in its contents but opens its body with one.
    let label = format!(
        "XELJANZ tablets WARNING: SERIOUS INFECTIONS AND MALIGNANCY See full prescribing information \
         for complete boxed warning. FULL PRESCRIBING INFORMATION: CONTENTS* 1 INDICATIONS AND USAGE \
         1.1 Rheumatoid Arthritis 4 CONTRAINDICATIONS {OMITTED} FULL PRESCRIBING INFORMATION WARNING: \
         SERIOUS INFECTIONS AND MALIGNANCY Patients treated with XELJANZ are at increased risk for \
         serious infections. 1 INDICATIONS AND USAGE 1.1 Rheumatoid Arthritis XELJANZ is indicated. \
         4 CONTRAINDICATIONS None."
    );
    let boxed = section(&label, S::BoxedWarning);
    assert!(
        boxed.starts_with("WARNING: SERIOUS INFECTIONS AND MALIGNANCY Patients"),
        "{boxed}"
    );
    // The highlights name a box the body does not show: the box is unread, never absent.
    let shown = "FULL PRESCRIBING INFORMATION WARNING: SERIOUS INFECTIONS AND MALIGNANCY \
                 Patients treated with XELJANZ are at increased risk for serious infections.";
    let missing = label.replace(shown, "FULL PRESCRIBING INFORMATION");
    assert_eq!(found(&missing, S::BoxedWarning), Found::Unbounded);
}

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

/// Opzelura's 2025 contents list and body opening, numbered `1.` and `1.1.`.
const DOTTED: &str = "FULL PRESCRIBING INFORMATION: CONTENTS* WARNING: SERIOUS INFECTIONS, MAJOR ADVERSE \
    CARDIOVASCULAR EVENTS, AND THROMBOSIS 1. INDICATIONS AND USAGE 1.1. Atopic Dermatitis 2. DOSAGE AND \
    ADMINISTRATION 4. CONTRAINDICATIONS 5. WARNINGS AND PRECAUTIONS 5.1. Serious Infections 6. ADVERSE \
    REACTIONS 6.1. Clinical Trials Experience *Sections or subsections omitted from the full prescribing \
    information are not listed. Reference ID: 5661016 2 FULL PRESCRIBING INFORMATION WARNING: SERIOUS \
    INFECTIONS, MAJOR ADVERSE CARDIOVASCULAR EVENTS, AND THROMBOSIS Patients treated with oral Janus \
    kinase inhibitors are at risk. 1. INDICATIONS AND USAGE 1.1. Atopic Dermatitis OPZELURA is indicated \
    for atopic dermatitis. 2. DOSAGE AND ADMINISTRATION Apply twice daily. 4. CONTRAINDICATIONS None. \
    5. WARNINGS AND PRECAUTIONS 5.1. Serious Infections Avoid use in active infection. 6. ADVERSE \
    REACTIONS Nasopharyngitis was reported.";
/// The box and the body's opening heading in [`DOTTED`].
const DOTTED_BOX: &str = "2 FULL PRESCRIBING INFORMATION WARNING: SERIOUS INFECTIONS, MAJOR ADVERSE \
    CARDIOVASCULAR EVENTS, AND THROMBOSIS Patients treated with oral Janus kinase inhibitors are at risk. ";

#[test]
fn a_contents_list_numbered_with_full_stops_bounds_its_sections() {
    let boxed = section(DOTTED, S::BoxedWarning);
    assert!(
        boxed.starts_with("WARNING: SERIOUS") && boxed.ends_with("are at risk."),
        "{boxed}"
    );
    assert_eq!(
        section(DOTTED, S::WarningsAndCautions),
        "5. WARNINGS AND PRECAUTIONS 5.1. Serious Infections Avoid use in active infection."
    );
}

#[test]
fn a_two_column_contents_list_never_runs_a_section_over_the_ones_it_lost() {
    // Opzelura's 2026 list, read row by row across its two columns: sections 2 to 5 come after
    // section 6, so the list in order gives section 1 and then section 6.
    let listed = "1. INDICATIONS AND USAGE 1.1. Atopic Dermatitis 2. DOSAGE AND ADMINISTRATION 4. \
        CONTRAINDICATIONS 5. WARNINGS AND PRECAUTIONS 5.1. Serious Infections 6. ADVERSE REACTIONS 6.1. \
        Clinical Trials Experience *Sections";
    let rows = "5.4. Major Adverse Cardiovascular Events (MACE) 1. INDICATIONS AND USAGE 6. ADVERSE \
        REACTIONS 1.1. Atopic Dermatitis 6.1. Clinical Trials Experience 2. DOSAGE AND ADMINISTRATION 4. \
        CONTRAINDICATIONS 5. WARNINGS AND PRECAUTIONS *Sections";
    let interleaved = DOTTED.replace(listed, rows);
    assert_ne!(interleaved, DOTTED, "the list was replaced");
    let boxed = section(&interleaved, S::BoxedWarning);
    assert!(boxed.ends_with("are at risk."), "{boxed}");
    assert_eq!(
        found(&interleaved, S::IndicationsAndUsage),
        Found::Unbounded
    );
}

#[test]
fn a_box_drawn_as_an_image_is_read_only_through_its_listed_title() {
    // Opzelura's 2021 original: the box and the body's own heading are on a page drawn as an image.
    let label = DOTTED.replace(DOTTED_BOX, "");
    assert_ne!(label, DOTTED, "the box was taken out");
    let title = "WARNING: SERIOUS INFECTIONS, MAJOR ADVERSE CARDIOVASCULAR EVENTS, AND THROMBOSIS";
    assert_eq!(
        found(&label, S::BoxedWarning),
        Found::Title(title.to_owned())
    );
    // A title the list's other column runs into is no title.
    let mixed = label.replace(
        "MAJOR ADVERSE CARDIOVASCULAR EVENTS, AND",
        "5.4. Major Adverse Cardiovascular Events MAJOR ADVERSE CARDIOVASCULAR EVENTS, AND",
    );
    assert_eq!(found(&mixed, S::BoxedWarning), Found::Unbounded);
}

#[test]
fn only_a_document_that_names_another_kind_is_not_a_label() {
    let ifu = "INSTRUCTIONS FOR USE AIMOVIG (erenumab-aooe) injection, for subcutaneous use. This \
        Instructions for Use contains information on how to inject AIMOVIG.";
    assert_eq!(found(ifu, S::WarningsAndCautions), Found::NotPrescribing);
    // Tecentriq's 2016 original reads as cross-references alone: unread, not "not a label".
    let sparse = "o o o www.fda.gov/medwatch. [see Clinical Studies (14.1)] [see Warnings and \
        Precautions (5.1)] [see Dosage and Administration (2.2) and Adverse Reactions (6.1)].";
    assert_eq!(found(sparse, S::IndicationsAndUsage), Found::Older);
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

/// Xeljanz's 2012 original label, unchanged: its contents list leaves out the boxed warning.
const XELJANZ_2012: &[u8] =
    include_bytes!("../../../../biomcp-adapters/tests/fixtures/pdf/label-xeljanz-2012.pdf");

#[test]
fn a_real_label_reads_into_bounded_sections() -> Result<(), String> {
    let read = biomcp_adapters::pdf::embedded(XELJANZ_2012, 400, &|| false)
        .map_err(|reason| format!("{reason:?}"))?;
    assert_eq!(read.state, biomcp_types::PdfTextState::Text);
    let text = |each| section(&read.text, each);
    assert!(text(S::BoxedWarning).starts_with("WARNING: SERIOUS INFECTIONS AND MALIGNANCY"));
    assert!(text(S::IndicationsAndUsage).starts_with("1 INDICATIONS AND USAGE"));
    let adverse = text(S::AdverseReactions);
    assert!(
        adverse.starts_with("6 ADVERSE REACTIONS Because clinical studies"),
        "{adverse}"
    );
    let interactions = text(S::DrugInteractions);
    assert!(
        interactions.starts_with("7 DRUG INTERACTIONS 7.1 Potent CYP3A4 Inhibitors"),
        "{interactions}"
    );
    assert!(
        !adverse.contains("Potent CYP3A4 Inhibitors"),
        "adverse reactions end at section 7"
    );
    Ok(())
}
