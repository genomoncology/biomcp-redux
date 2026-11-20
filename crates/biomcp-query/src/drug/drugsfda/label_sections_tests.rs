//! Section bounds on label text shaped like the 1066 review's wrong cases: table rows that look
//! like subsections, headings whose number the PDF drew apart, boxed warnings the contents list
//! leaves out, a label missing its first page, and documents that are not labels.
use super::label_sections::{Found, find};
use super::label_text::{clean, flatten};
use biomcp_types::LabelHistorySection as S;

const HIGHLIGHTS: &str = "HIGHLIGHTS OF PRESCRIBING INFORMATION These highlights do not include all \
    the information needed to use DRUG safely and effectively. ----INDICATIONS AND USAGE---- DRUG is \
    indicated for melanoma (1.1). ----WARNINGS AND PRECAUTIONS---- Immune-mediated reactions (5.1).";
const OMITTED: &str =
    "*Sections or subsections omitted from the full prescribing information are not listed.";

fn found(text: &str, section: S) -> Found {
    let cleaned = clean(text);
    find(&cleaned, &flatten(&cleaned), section)
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
    let missing = label.replace("FULL PRESCRIBING INFORMATION WARNING: SERIOUS INFECTIONS AND MALIGNANCY Patients treated with XELJANZ are at increased risk for serious infections.", "FULL PRESCRIBING INFORMATION");
    assert_eq!(found(&missing, S::BoxedWarning), Found::Unbounded);
}

#[test]
fn a_label_missing_its_contents_is_unread_never_absent() {
    // Olumiant's 2018 original as the reader gives it: page 1, with the box and contents, is gone.
    let label = "This label may not be the latest approved by FDA. 2 14 CLINICAL STUDIES 16 HOW SUPPLIED \
        17 PATIENT COUNSELING INFORMATION * Sections or subsections omitted from the full prescribing \
        information are not listed. 4 CONTRAINDICATIONS None. 5 WARNINGS AND PRECAUTIONS 5.1 Serious \
        Infections Serious and sometimes fatal infections.";
    for each in [
        S::BoxedWarning,
        S::Contraindications,
        S::WarningsAndCautions,
    ] {
        assert_eq!(found(label, each), Found::Unbounded, "{each}");
    }
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

#[test]
fn older_labels_count_headings_only_as_whole_lines() {
    let whole = "DESCRIPTION\nAtorvastatin calcium is a synthetic agent.\nWARNINGS\nLiver dysfunction \
        has been reported.\nPRECAUTIONS\nGeneral: rule out secondary causes.\nADVERSE REACTIONS\n\
        Myalgia was reported.\nOVERDOSAGE\nThere is no specific treatment.\n";
    assert_eq!(
        section(whole, S::AdverseReactions),
        "ADVERSE REACTIONS Myalgia was reported."
    );
    let warnings = section(whole, S::WarningsAndCautions);
    assert!(warnings.starts_with("WARNINGS Liver") && warnings.contains("PRECAUTIONS General"));
    assert_eq!(found(whole, S::BoxedWarning), Found::Older);
    // Lipitor's 2005 label as the reader gives it: headings run into their paragraphs.
    let inline = "DESCRIPTION Atorvastatin calcium.\n  CONTRAINDICATIONS  Active liver disease.\n";
    assert_eq!(found(inline, S::Contraindications), Found::Older);
}
