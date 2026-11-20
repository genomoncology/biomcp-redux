//! Extracted text fixtures for the label section bounds tests.
pub(super) const HIGHLIGHTS: &str = "HIGHLIGHTS OF PRESCRIBING INFORMATION These highlights do not include all \
    the information needed to use DRUG safely and effectively. ----INDICATIONS AND USAGE---- DRUG is \
    indicated for melanoma (1.1). ----WARNINGS AND PRECAUTIONS---- Immune-mediated reactions (5.1).";
pub(super) const OMITTED: &str =
    "*Sections or subsections omitted from the full prescribing information are not listed.";

/// A Keytruda-shaped label: contents, then a body whose adverse reactions hold table rows.
pub(super) fn keytruda() -> String {
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

/// Opzelura's 2025 contents list and body opening, numbered `1.` and `1.1.`.
pub(super) const DOTTED: &str = "FULL PRESCRIBING INFORMATION: CONTENTS* WARNING: SERIOUS INFECTIONS, MAJOR ADVERSE \
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
pub(super) const DOTTED_BOX: &str = "2 FULL PRESCRIBING INFORMATION WARNING: SERIOUS INFECTIONS, MAJOR ADVERSE \
    CARDIOVASCULAR EVENTS, AND THROMBOSIS Patients treated with oral Janus kinase inhibitors are at risk. ";

/// Lipitor's 2007 label as the reader gives it: headings run into their paragraphs, one behind
/// a page number, with cross-references in capitals.
pub(super) const LIPITOR: &str = "DESCRIPTION Atorvastatin calcium is a synthetic agent (see CLINICAL PHARMACOLOGY). \
    INDICATIONS AND USAGE LIPITOR is indicated as an adjunct to diet. CONTRAINDICATIONS Active liver \
    disease. WARNINGS Liver Dysfunction Persistent elevations occurred (see DOSAGE AND ADMINISTRATION). \
    Myopathy, see ADVERSE REACTIONS and PRECAUTIONS, Drug Interactions. PRECAUTIONS General Rule out \
    secondary causes (see WARNINGS, Skeletal Muscle). 22 ADVERSE REACTIONS LIPITOR is generally \
    well-tolerated. Musculoskeletal System: leg cramps, myasthenia. OVERDOSAGE There is no specific \
    treatment. DOSAGE AND ADMINISTRATION Take once daily. HOW SUPPLIED Tablets.";
