//! Public boundary tables for identifiers, partial dates and coordinate bounds.
use biodata::{HgncId, PartialDate, Position1, Span0};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;
use std::{fmt::Debug, str::FromStr};

#[expect(
    clippy::unwrap_used,
    reason = "Public boundary table uses valid round-trip fixtures"
)]
fn text_roundtrip<T: FromStr + Serialize + DeserializeOwned + Eq + Debug + ToString>(text: &str)
where
    T::Err: Debug,
{
    let value: T = text.parse().unwrap();
    assert_eq!(
        value.to_string(),
        text,
        "Text must preserve its checked roundtrip contract"
    );
    assert_eq!(
        serde_json::to_value(&value).unwrap(),
        json!(text),
        "Text must preserve its checked roundtrip contract"
    );
    assert_eq!(
        serde_json::from_value::<T>(json!(text)).unwrap(),
        value,
        "Text must preserve its checked roundtrip contract"
    );
}

#[test]
fn hgnc_ids_check_numeric_and_canonical_text_boundaries() {
    for (number, text) in [
        (1, "HGNC:1"),
        (1097, "HGNC:1097"),
        (u32::MAX, "HGNC:4294967295"),
    ] {
        assert_eq!(HgncId::new(number).unwrap().get(), number);
        assert_eq!(HgncId::new(number).unwrap().to_string(), text);
        text_roundtrip::<HgncId>(text);
    }
    assert!(HgncId::new(0).is_err());
    for text in [
        "",
        "HGNC:0",
        "HGNC:01",
        "HGNC:+1",
        "HGNC:-1",
        "hgnc:1",
        "HGNC: 1",
        " HGNC:1",
        "HGNC:1\n",
        "HGNC:١",
        "HGNC:4294967296",
        "distinctive-rejected-input",
    ] {
        let error = text.parse::<HgncId>().unwrap_err();
        assert!(!format!("{error:?} {error}").contains("distinctive-rejected-input"));
        let error = serde_json::from_value::<HgncId>(json!(text)).unwrap_err();
        assert!(!error.to_string().contains("distinctive-rejected-input"));
    }
    for value in [json!(1), json!(null), json!(true), json!({})] {
        assert!(serde_json::from_value::<HgncId>(value).is_err());
    }
    let a: HgncId = "HGNC:1".parse().unwrap();
    assert!(std::collections::HashSet::from([a]).contains(&HgncId::new(1).unwrap()));
}

#[test]
fn partial_dates_preserve_precision_and_validate_gregorian_calendar() {
    for (year, month, day, text) in [
        (1, None, None, "0001"),
        (9999, Some(12), Some(31), "9999-12-31"),
        (2000, Some(2), Some(29), "2000-02-29"),
        (2024, Some(2), Some(29), "2024-02-29"),
        (1900, Some(2), Some(28), "1900-02-28"),
        (2023, Some(2), None, "2023-02"),
    ] {
        let value = PartialDate::new(year, month, day).unwrap();
        assert_eq!(
            (value.year(), value.month(), value.day()),
            (year, month, day)
        );
        assert_eq!(value.to_string(), text);
        text_roundtrip::<PartialDate>(text);
    }
    for args in [
        (0, None, None),
        (10000, None, None),
        (2024, None, Some(1)),
        (2024, Some(0), None),
        (2024, Some(13), None),
        (2024, Some(1), Some(0)),
        (2024, Some(4), Some(31)),
        (1900, Some(2), Some(29)),
        (2023, Some(2), Some(29)),
    ] {
        assert!(PartialDate::new(args.0, args.1, args.2).is_err());
    }
    for text in [
        "",
        "1",
        "0000",
        "10000",
        "2024-2",
        "2024-02-1",
        "2024-02-30",
        "2024-01-01x",
        " 2024",
        "2024\n",
        "2024-",
        "2024--01",
        "２０２４",
        "distinctive-rejected-input",
    ] {
        assert!(text.parse::<PartialDate>().is_err(), "{text}");
        let error = serde_json::from_value::<PartialDate>(json!(text)).unwrap_err();
        assert!(!error.to_string().contains("distinctive-rejected-input"));
    }
    assert!(serde_json::from_value::<PartialDate>(json!(2024)).is_err());
}

#[test]
fn coordinates_enforce_integer_bounds_and_half_open_span_order() {
    for number in [1, u32::MAX] {
        let position = Position1::new(number).unwrap();
        assert_eq!(position.get(), number);
        assert_eq!(serde_json::to_value(position).unwrap(), json!(number));
        assert_eq!(
            serde_json::from_value::<Position1>(json!(number)).unwrap(),
            position
        );
    }
    assert!(Position1::new(0).is_err());
    for value in [
        json!(0),
        json!(-1),
        json!(4294967296_u64),
        json!(1.5),
        json!("1"),
        json!(null),
    ] {
        assert!(serde_json::from_value::<Position1>(value).is_err());
    }
    for (start, end) in [(0, 0), (0, 1), (u32::MAX, u32::MAX), (0, u32::MAX)] {
        let span = Span0::new(start, end).unwrap();
        assert_eq!((span.start(), span.end()), (start, end));
        let value = json!({"start":start,"end":end});
        assert_eq!(serde_json::to_value(span).unwrap(), value);
        assert_eq!(serde_json::from_value::<Span0>(value).unwrap(), span);
    }
    assert!(Span0::new(2, 1).is_err());
    for value in [
        json!({"start":2,"end":1}),
        json!({"start":0}),
        json!({"end":1}),
        json!({"start":-1,"end":1}),
        json!({"start":0,"end":4294967296_u64}),
        json!({"start":"0","end":1}),
        json!([0, 1]),
        json!(null),
    ] {
        assert!(serde_json::from_value::<Span0>(value).is_err());
    }
}

// This inventory covers every public biodata Display + FromStr value, including
// macro-defined vocabularies. Structured records, numeric coordinates/numbers,
// Markdown and native enums lack that pair; their JSON contracts have owners in
// the existing record tests. SourceText has an infallible From boundary below.
// No schema-pattern or unknown-enum policy checks belong to this sweep.
type Check = Result<(), Box<dyn std::error::Error>>;

fn value_contract<T>(valid: &[&str], invalid: &[&str]) -> Check
where
    T: FromStr + Serialize + DeserializeOwned + Eq + Debug + ToString,
    T::Err: std::error::Error + 'static,
{
    for text in valid {
        let value: T = text.parse()?;
        assert_eq!(
            value.to_string().parse::<T>()?,
            value,
            "display must parse to the same value: {text}"
        );
        let wire = serde_json::to_value(&value)?;
        assert_eq!(
            serde_json::from_value::<T>(wire)?,
            value,
            "JSON must preserve the value: {text}"
        );
    }
    for text in invalid {
        assert!(
            text.parse::<T>().is_err(),
            "parser must reject near miss: {text:?}"
        );
        // Some values use structured JSON; their checks are exercised separately.
        assert!(
            serde_json::from_value::<T>(json!(text)).is_err(),
            "JSON must reject near miss: {text:?}"
        );
    }
    Ok(())
}

fn equal_hash<T>(left: &str, right: &str) -> Check
where
    T: FromStr + Eq + Debug + std::hash::Hash,
    T::Err: std::error::Error + 'static,
{
    use std::hash::{DefaultHasher, Hash, Hasher};
    let left: T = left.parse()?;
    let right: T = right.parse()?;
    assert_eq!(left, right, "fixtures must represent equal values");
    let mut a = DefaultHasher::new();
    let mut b = DefaultHasher::new();
    left.hash(&mut a);
    right.hash(&mut b);
    assert_eq!(a.finish(), b.finish(), "equal values must hash equally");
    Ok(())
}

#[test]
fn public_text_values_preserve_identity_1() -> Check {
    value_contract::<biodata::HgncId>(&["HGNC:1", "HGNC:4294967295"], &["HGNC:01"])?;
    value_contract::<biodata::PartialDate>(&["2024", "2024-02", "2024-02-29"], &["2023-02-29"])?;
    value_contract::<biodata::Text>(&[" checked α ", "a\\u{0001}"], &["\u{0001}"])?;
    value_contract::<biodata::NcbiGeneId>(&["1", "4294967295"], &["01"])?;
    value_contract::<biodata::GeneSymbol>(&["BRAF", "bRaF"], &["1BRAF"])?;
    value_contract::<biodata::EnsemblGeneId>(
        &["ENSG00000157764", "ENSG00000157764.2"],
        &["ENSG00000157764.0"],
    )?;
    value_contract::<biodata::OmimId>(&["001234"], &["12345"])?;
    value_contract::<biodata::UniProtAcc>(&["P15056", "A0A024RBG1"], &["P15056-1"])?;
    value_contract::<biodata::Cytoband>(&["7q34", "Xp22.33"], &["7q3"])?;
    value_contract::<biodata::Pmid>(&["1"], &["01"])?;
    value_contract::<biodata::Pmcid>(&["PMC1"], &["PMC01"])?;
    value_contract::<biodata::Doi>(&["10.1000/ABC", "10.1000/abc"], &["10.100/abc"])?;
    value_contract::<biodata::EuropePmcId>(&["MED:1", "PMC:PMC1"], &["MED:01"])?;
    value_contract::<biodata::SemanticScholarId>(
        &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
        &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    )?;
    value_contract::<biodata::Accession>(
        &[
            "NM_004333.6",
            "NC_000007.14",
            "ENST00000288602.11",
            "LRG_1t1",
        ],
        &["NM_004333.0"],
    )?;
    value_contract::<biodata::AccessionBase>(&["NM_004333"], &["NM_004333.6"])?;
    value_contract::<biodata::Chromosome>(&["1", "X", "MT"], &["23"])?;
    value_contract::<biodata::Contig>(&["chr7", "NC_000007.14"], &["chr 7"])?;
    value_contract::<biodata::Base>(&["A", "N"], &["a"])?;
    value_contract::<biodata::Sequence>(&["ACGTN"], &["ACGT!"])?;
    value_contract::<biodata::HgvsText>(
        &["NM_004333.6:c.1799T>A", "NP_004324.2:p.Val600Glu"],
        &["NM_004333.6:c."],
    )?;
    value_contract::<biodata::RsId>(&["rs1"], &["rs01"])?;
    value_contract::<biodata::CaId>(&["CA000001"], &["CA1"])?;
    value_contract::<biodata::PaId>(&["PA1"], &["PA01"])?;
    value_contract::<biodata::ClinVarVariationId>(&["clinvar:1"], &["clinvar:01"])?;
    value_contract::<biodata::ClinVarAlleleId>(&["1"], &["01"])?;
    value_contract::<biodata::VcvId>(&["VCV000000001", "VCV000000001.2"], &["VCV000000000"])?;
    value_contract::<biodata::RcvId>(&["RCV000000001.1"], &["RCV000000001.0"])?;
    value_contract::<biodata::ScvId>(&["SCV000000001.1"], &["SCV000000001.0"])?;
    value_contract::<biodata::CosmicId>(&["COSV1", "COSM1", "COSN1"], &["COSV01"])?;
    value_contract::<biodata::VrsId>(
        &["ga4gh:VA.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
        &["ga4gh:VA.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    )?;
    value_contract::<biodata::VariantId>(
        &[
            "rs1",
            "CA000001",
            "PA1",
            "VCV000000001",
            "clinvar:1",
            "COSV1",
            "ga4gh:VA.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ],
        &["rs01"],
    )?;
    value_contract::<biodata::Curie>(&["HGNC:1097"], &["HGNC:"])?;
    value_contract::<biodata::NctId>(&["NCT00000001"], &["NCT0000001"])?;
    value_contract::<biodata::TrialUploadTime>(&["2024-02-29T23:59"], &["2023-02-29T23:59"])?;
    Ok(())
}

#[test]
fn public_text_values_preserve_identity_2() -> Check {
    value_contract::<biodata::DrugId>(&["CHEMBL25", "NCIT:C1"], &["NCIT:"])?;
    value_contract::<biodata::ReportId>(&["faers:1"], &["faers:"])?;
    value_contract::<biodata::ClinPgxAccession>(&["PA1"], &["PA01"])?;
    value_contract::<biodata::PgxGeneKey>(&["CYP2D6"], &["1CYP"])?;
    value_contract::<biodata::PgxDiplotypeExpression>(&[" *1/*2 "], &["  "])?;
    value_contract::<biodata::MondoId>(&["MONDO:0000001"], &["MONDO:000001"])?;
    value_contract::<biodata::NcitId>(&["C123"], &["C"])?;
    value_contract::<biodata::GwasStudyId>(&["GCST000001"], &["GCST000000"])?;
    value_contract::<biodata::GwasAssociationId>(&["association:1"], &["association:01"])?;
    value_contract::<biodata::GwasTraitId>(&["EFO_0000001", "HP:0000001"], &["EFO_"])?;
    value_contract::<biodata::ScientificPValue>(&["1.50e-8", "1.5e-8", "1e0"], &["1e1", "0e-8"])?;
    value_contract::<biodata::HpId>(&["HP:0000001"], &["HP:000001"])?;
    value_contract::<biodata::ReactomeId>(&["R-HSA-1"], &["R-HSA-01"])?;
    value_contract::<biodata::WikiPathwaysId>(&["WP1"], &["WP01"])?;
    value_contract::<biodata::PathwayId>(&["R-HSA-1", "WP1"], &["WP01"])?;
    value_contract::<biodata::UniProtIsoform>(&["P15056", "P15056-2"], &["P15056-0"])?;
    value_contract::<biodata::AminoAcids>(&["ACDEXUO"], &["ACDE!"])?;
    value_contract::<biodata::InterProId>(&["IPR000001"], &["IPR00001"])?;
    value_contract::<biodata::ComplexPortalId>(&["CPX-1"], &["CPX-01"])?;
    value_contract::<biodata::StudyId>(&["study_1"], &["study/1"])?;
    value_contract::<biodata::StudyProfileId>(&["study_mutations"], &["study/mutations"])?;
    value_contract::<biodata::StudySampleListId>(&["study_all"], &["study/all"])?;
    value_contract::<biodata::CbioportalCancerTypeId>(&["brca"], &["brca/"])?;
    value_contract::<biodata::StudyResourceId>(&["resource_1"], &["resource/1"])?;
    value_contract::<biodata::StudyBlobSha>(
        &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
        &["AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"],
    )?;
    value_contract::<biodata::StudyImportTime>(&["2024-02-29 23:59:59"], &["2023-02-29 23:59:59"])?;
    value_contract::<biodata::GtrTestId>(&["GTR000000001.1"], &["GTR000000001.0"])?;
    value_contract::<biodata::WhoProductId>(&["1234-123-12"], &["1234-12-12"])?;
    value_contract::<biodata::PmaNumber>(&["P000001"], &["P00001"])?;
    value_contract::<biodata::PmaSupplement>(&["S001"], &["S000"])?;
    value_contract::<biodata::FdaDrugApplication>(&["NDA000001", "BLA000001"], &["NDA00001"])?;
    value_contract::<biodata::DiagnosticDeviceId>(
        &["P000001/S001", "K000001", "DEN000001", "H000001"],
        &["P000001/S000"],
    )?;
    value_contract::<biodata::PmaSupplementCell>(&["", "S001"], &["S000"])?;
    value_contract::<biodata::DeviceRecallNumber>(&["Z-0001-2024"], &["Z-001-2024"])?;
    value_contract::<biodata::DrugRecallNumber>(&["D-0001-2024"], &["D-001-2024"])?;
    Ok(())
}

#[test]
fn public_text_values_preserve_identity_3() -> Check {
    value_contract::<biodata::CellosaurusAccession>(&["CVCL_0030"], &["CVCL_030"])?;
    value_contract::<biodata::CellLineSourceId>(&["ATCC:HTB-26"], &["ATCC:"])?;
    value_contract::<biodata::ChemblCellId>(&["CHEMBL1"], &["CHEMBL"])?;
    value_contract::<biodata::SemanticScholarAuthorId>(&["1"], &["01"])?;
    value_contract::<biodata::OrcidId>(&["0000-0002-1825-0097"], &["0000-0002-1825-0098"])?;
    value_contract::<biodata::AuthorId>(
        &["orcid:0000-0002-1825-0097", "semanticscholar:1"],
        &["orcid:0000-0002-1825-0098"],
    )?;
    value_contract::<biodata::OrcidClientId>(&["APP-0123456789ABCDEF"], &["APP-0123456789ABCDE"])?;
    value_contract::<biodata::OrcidPublicVisibility>(&["public", "PUBLIC"], &["private"])?;
    value_contract::<biodata::License>(&["cc_by"], &["cc_by!"])?;
    value_contract::<biodata::TrialOutcomeKind>(&["primary"], &["primary!"])?;
    value_contract::<biodata::TrialDateKind>(&["ACTUAL"], &["ACTUAL!"])?;
    value_contract::<biodata::TrialArmKind>(&["EXPERIMENTAL"], &["EXPERIMENTAL!"])?;
    value_contract::<biodata::TrialInterventionKind>(&["BEHAVIORAL"], &["BEHAVIORAL!"])?;
    value_contract::<biodata::TrialReferenceKind>(&["BACKGROUND"], &["BACKGROUND!"])?;
    value_contract::<biodata::TrialPhase>(&["EARLY_PHASE1"], &["EARLY_PHASE1!"])?;
    value_contract::<biodata::TrialStatus>(
        &["ACTIVE_NOT_RECRUITING"],
        &["ACTIVE_NOT_RECRUITING!"],
    )?;
    value_contract::<biodata::StudyType>(&["INTERVENTIONAL"], &["INTERVENTIONAL!"])?;
    value_contract::<biodata::TrialSex>(&["ALL"], &["ALL!"])?;
    value_contract::<biodata::TrialSiteStatus>(&["NOT_YET_RECRUITING"], &["NOT_YET_RECRUITING!"])?;
    value_contract::<biodata::TrialContactRole>(&["CONTACT"], &["CONTACT!"])?;
    value_contract::<biodata::TrialOfficialRole>(&["SUB_INVESTIGATOR"], &["SUB_INVESTIGATOR!"])?;
    value_contract::<biodata::PhenotypeSynonymClass>(&["exact"], &["exact!"])?;
    value_contract::<biodata::PhenotypeHierarchyRelation>(&["super_classes"], &["super_classes!"])?;
    value_contract::<biodata::Build>(&["GRCh37"], &["GRCh37!"])?;
    value_contract::<biodata::Strand>(&["+"], &["+!"])?;
    value_contract::<biodata::PathwaySpecies>(&["Homo sapiens"], &["Homo sapiens!"])?;
    value_contract::<biodata::GtrTestType>(&["Clinical"], &["Clinical!"])?;
    value_contract::<biodata::GtrMethodList>(&["method_categories"], &["method_categories!"])?;
    value_contract::<biodata::DrugIdentityLevel>(&["ingredient"], &["ingredient!"])?;
    value_contract::<biodata::StudyFileKind>(&["file"], &["file!"])?;
    value_contract::<biodata::AuthorCountScope>(&["provider_record"], &["provider_record!"])?;
    value_contract::<biodata::ManeStatus>(&["select"], &["select!"])?;
    value_contract::<biodata::GeneStatus>(&["approved"], &["approved!"])?;
    value_contract::<biodata::LocusGroup>(&["protein_coding"], &["protein_coding!"])?;
    value_contract::<biodata::MatchKind>(&["symbol"], &["symbol!"])?;
    Ok(())
}

#[test]
fn public_text_values_preserve_identity_4() -> Check {
    value_contract::<biodata::SummarySource>(&["refseq"], &["refseq!"])?;
    value_contract::<biodata::HgncStatus>(&["entry_withdrawn"], &["entry_withdrawn!"])?;
    value_contract::<biodata::LocusType>(
        &["gene_with_protein_product"],
        &["gene_with_protein_product!"],
    )?;
    value_contract::<biodata::FaersAgeUnit>(&["800"], &["800!"])?;
    value_contract::<biodata::ExpressionSyntax>(&["hgvs.g"], &["hgvs.g!"])?;
    value_contract::<biodata::RefSeqPrefix>(&["NC_"], &["NC_!"])?;
    value_contract::<biodata::EnsemblFeature>(&["ENSG"], &["ENSG!"])?;
    value_contract::<biodata::PopulationDataType>(&["exomes"], &["exomes!"])?;
    value_contract::<biodata::PopulationSex>(&["xx"], &["xx!"])?;
    value_contract::<biodata::GenotypeFilter>(&["raw"], &["raw!"])?;
    value_contract::<biodata::ProteinCoordinateConvention>(
        &["one_based_inclusive"],
        &["one_based_inclusive!"],
    )?;
    value_contract::<biodata::ProteinIsoformStatus>(&["Displayed"], &["Displayed!"])?;
    value_contract::<biodata::ProteinSequenceAvailability>(&["available"], &["available!"])?;
    value_contract::<biodata::ClinvarReviewStatusCode>(
        &["practice_guideline"],
        &["practice_guideline!"],
    )?;
    value_contract::<biodata::ClinvarAlleleOriginCode>(&["germline"], &["germline!"])?;
    value_contract::<biodata::AcmgAmp2015Code>(&["pathogenic"], &["pathogenic!"])?;
    value_contract::<biodata::ClingenLowPenetrance2024Code>(
        &["pathogenic_low_penetrance"],
        &["pathogenic_low_penetrance!"],
    )?;
    value_contract::<biodata::AmpAscoCap2017TierCode>(&["tier_i"], &["tier_i!"])?;
    value_contract::<biodata::AmpAscoCap2017LevelCode>(&["level_a"], &["level_a!"])?;
    value_contract::<biodata::ClingenCgcVicc2022Code>(&["oncogenic"], &["oncogenic!"])?;
    value_contract::<biodata::CivicEvidenceLevelCode>(&["a"], &["a!"])?;
    value_contract::<biodata::OncokbOncogenicityCode>(&["oncogenic"], &["oncogenic!"])?;
    value_contract::<biodata::OncokbLevelCode>(&["level_1"], &["level_1!"])?;
    value_contract::<biodata::Proposition>(&["pathogenicity"], &["pathogenicity!"])?;
    value_contract::<biodata::Direction>(&["supports"], &["supports!"])?;
    value_contract::<biodata::Predicate>(&["is_causal_for"], &["is_causal_for!"])?;
    Ok(())
}

#[test]
fn equal_identity_uses_equal_hashes() -> Check {
    equal_hash::<biodata::GeneSymbol>("BRAF", "bRaF")?;
    equal_hash::<biodata::Doi>("10.1000/ABC", "10.1000/abc")?;
    equal_hash::<biodata::ScientificPValue>("1.50e-8", "1.5e-8")?;
    equal_hash::<biodata::VcvId>("VCV000000001.1", "VCV000000001.2")?;
    equal_hash::<biodata::RcvId>("RCV000000001.1", "RCV000000001.2")?;
    equal_hash::<biodata::ScvId>("SCV000000001.1", "SCV000000001.2")?;
    Ok(())
}

proptest::proptest! {
    #![proptest_config(proptest::test_runner::Config {
        cases: 128, failure_persistence: None, ..Default::default()
    })]
    #[test]
    fn public_variant_parsers_never_panic_and_roundtrip(
        arbitrary in proptest::collection::vec(proptest::char::any(), 0..2200),
        accession in "(NM_|NC_)[0-9]{6}\\.[1-9][0-9]{0,3}",
        suffix in "[1-9][0-9]{0,6}[ACGT]>[ACGT]",
    ) {
        let arbitrary: String = arbitrary.into_iter().collect();
        for text in [arbitrary, accession.clone(), format!("{accession}:c.{suffix}")] {
            property_roundtrip::<biodata::Accession>(&text)?;
            property_roundtrip::<biodata::HgvsText>(&text)?;
            property_roundtrip::<biomcp_types::VariantInput>(&text)?;
        }
    }
}

fn property_roundtrip<T>(text: &str) -> Result<(), proptest::test_runner::TestCaseError>
where
    T: FromStr + Eq + Debug + ToString,
    T::Err: Debug,
{
    if let Ok(value) = text.parse::<T>() {
        proptest::prop_assert_eq!(
            value.to_string().parse::<T>().as_ref(),
            Ok(&value),
            "display must roundtrip"
        );
    }
    Ok(())
}
