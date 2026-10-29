//! Public gene entities keep source identity, ambiguity and complete prose.
use biodata::*;
use biomcp_types::*;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;
use std::{collections::HashSet, fmt::Debug, hash::Hash, str::FromStr};
type Check = Result<(), Box<dyn std::error::Error>>;
fn round_trip<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: &T) -> Check {
    assert_eq!(
        serde_json::from_value::<T>(serde_json::to_value(value)?)?,
        *value,
        "Gene values must preserve their checked serialization contract"
    );
    Ok(())
}
fn header() -> Result<GeneHeader, Error> {
    Ok(GeneHeader {
        hgnc_id: HgncId::new(1097)?,
        symbol: GeneSymbol::new("BRAF")?,
        name: Text::new("BRAF name")?,
        locus_group: LocusGroup::ProteinCoding,
        location: None,
        status: GeneStatus::Approved,
        r#match: None,
    })
}
fn gene() -> Result<Gene, Error> {
    Ok(Gene {
        hgnc_id: HgncId::new(1097)?,
        symbol: GeneSymbol::new("BRAF")?,
        entrez_id: None,
        ensembl_id: None,
        name: Text::new("BRAF name")?,
        status: GeneStatus::Approved,
        locus_group: LocusGroup::ProteinCoding,
        locus_type: None,
        location: None,
        previous_symbols: Box::new([]),
        aliases: Box::new([]),
        uniprot: Box::new([]),
        omim: Box::new([]),
        mane: Box::default(),
        spans: Box::new([]),
        summary: None,
    })
}
#[test]
fn gene_resolutions_preserve_every_candidate_and_withdrawn_identity() -> Check {
    let matching = Match {
        kind: MatchKind::Symbol,
        text: Text::new("BRAF")?,
    };
    let mut entity = gene()?;
    entity.entrez_id = Some(NcbiGeneId::new(673)?);
    entity.ensembl_id = Some("ENSG00000157764.12".parse()?);
    entity.locus_type = Some(LocusType::GeneWithProteinProduct);
    entity.previous_symbols = vec![GeneSymbol::new("BRAF1")?].into_boxed_slice();
    entity.aliases = vec![Text::new("alias with spaces / symbols")?].into_boxed_slice();
    entity.uniprot = vec!["P15056".parse()?].into_boxed_slice();
    entity.omim = vec!["164757".parse()?].into_boxed_slice();
    entity.mane = vec![Mane::new(
        ManeStatus::Select,
        "NM_004333.6".parse()?,
        "ENST00000646891.2".parse()?,
    )?]
    .into();
    let prose = "Complete source summary. ".repeat(1000);
    entity.summary = Some(Summary {
        text: Text::new(&prose)?,
        source: SummarySource::RefSeq,
        dated: Some("2026-10".parse()?),
    });
    entity.spans = vec![GenomicInterval {
        build: Some(Build::GRCh38),
        chromosome: "7".parse()?,
        span: Span0::new(140719327, 140924929)?,
        strand: Some(Strand::Minus),
    }]
    .into_boxed_slice();
    let value = serde_json::to_value(&entity)?;
    assert_eq!(value["summary"]["text"], prose.trim());
    round_trip(&entity)?;
    for resolution in [
        GeneResolution::Resolved {
            gene: Box::new(entity),
            r#match: matching,
        },
        GeneResolution::Ambiguous {
            candidates: vec![header()?, header()?].into_boxed_slice(),
        },
        GeneResolution::Withdrawn {
            hgnc_id: HgncId::new(18617)?,
            symbol: GeneSymbol::new("BRCA3")?,
            hgnc_status: HgncStatus::EntryWithdrawn,
            merged_into: vec![HgncId::new(1)?, HgncId::new(2)?].into_boxed_slice(),
        },
        GeneResolution::NotFound {},
    ] {
        round_trip(&resolution)?;
    }
    let absent = serde_json::to_value(gene()?)?;
    for field in [
        "locus_type",
        "location",
        "entrez_id",
        "ensembl_id",
        "mane",
        "summary",
    ] {
        assert!(absent.get(field).is_none());
    }
    assert_eq!(
        serde_json::to_value(GeneResolution::NotFound {})?,
        json!({"resolution":"not_found"})
    );
    let extended: GeneResolution =
        serde_json::from_value(json!({"resolution":"not_found","gene":{}}))?;
    assert_eq!(
        serde_json::to_value(extended)?,
        json!({"resolution":"not_found"})
    );
    assert!(serde_json::from_value::<Gene>(json!({})).is_err());
    round_trip(&header()?)?;
    Ok(())
}
#[test]
fn mane_requires_curated_versioned_transcripts_at_its_boundary() -> Check {
    let valid = Mane::new(
        ManeStatus::Select,
        "NM_004333.6".parse()?,
        "ENST00000646891.2".parse()?,
    )?;
    assert_eq!(valid.refseq().to_string(), "NM_004333.6");
    round_trip(&valid)?;
    for (refseq, ensembl) in [
        ("NM_004333", "ENST00000646891.2"),
        ("NM_004333.6", "ENST00000646891"),
        ("XM_004333.6", "ENST00000646891.2"),
    ] {
        assert!(
            serde_json::from_value::<Mane>(
                json!({"status":"select", "refseq":refseq,"ensembl":ensembl})
            )
            .is_err()
        );
    }
    assert!(
        serde_json::from_value::<Mane>(
            json!({"status":"plus_clinical", "refseq":"NR_123456.1","ensembl":"ENST00000646891.2"})
        )
        .is_ok()
    );
    assert!(serde_json::from_value::<Mane>(json!({"status":"select", "refseq":"NM_004333.6","ensembl":"ENST00000646891.2","extra":true})).is_ok());
    Ok(())
}
#[test]
fn genomic_intervals_preserve_assembly_strand_and_scaffold_names() -> Check {
    for name in ["1", "22", "X", "Y", "MT", "NW_015148969.2"] {
        let contig = Contig::new(name)?;
        assert_eq!(contig.as_str(), name);
        round_trip(&contig)?;
    }
    for name in ["", " ", "chr 7", "7\n", "é"] {
        assert!(Contig::new(name).is_err());
    }
    assert!(Contig::new(&"x".repeat(Contig::MAX_LEN)).is_ok());
    assert!(Contig::new(&"x".repeat(Contig::MAX_LEN + 1)).is_err());
    for build in [Some(Build::GRCh37), Some(Build::GRCh38), None] {
        for strand in [None, Some(Strand::Plus), Some(Strand::Minus)] {
            let interval = GenomicInterval {
                build,
                strand,
                chromosome: "7".parse()?,
                span: Span0::new(0, 0)?,
            };
            round_trip(&interval)?;
            let json = serde_json::to_value(interval)?;
            assert_eq!(json.get("strand").is_none(), strand.is_none());
        }
    }
    for value in [
        json!({"chromosome":"chr7","span":{"start":0,"end":1}}),
        json!({"build":"GRCh38","chromosome":"7","span":{"start":2,"end":1}}),
    ] {
        assert!(serde_json::from_value::<GenomicInterval>(value).is_err());
    }
    let summary = Summary {
        text: Text::new("full")?,
        source: SummarySource::RefSeq,
        dated: None,
    };
    round_trip(&summary)?;
    assert!(serde_json::to_value(summary)?.get("dated").is_none());
    Ok(())
}

#[test]
fn gene_locations_parse_publisher_forms_without_changing_text() -> Check {
    for (text, kind) in [
        ("Xp22.33 and Yp11.32", "bands"),
        ("Xp22.3 and Yp11.3", "bands"),
        ("7q34", "bands"),
        ("Xp21.2-p21.1", "range"),
        ("9p21.2-9p21.1", "range"),
        ("mitochondria", "mitochondrial"),
        ("publisher location text", "unparsed"),
        ("7q34 and invalid", "unparsed"),
        ("Xp21.2-invalid", "unparsed"),
        (" 7q34 ", "unparsed"),
    ] {
        let location = GeneLocation::from(text);
        assert_eq!(location.as_str(), text);
        assert_eq!(serde_json::to_value(&location)?, text);
        round_trip(&location)?;
        let parsed = match location.kind() {
            GeneLocationKind::Cytobands { first, additional } => {
                assert_eq!(
                    first.as_str(),
                    text.split(" and ").next().ok_or("no first band")?
                );
                assert_eq!(additional.len(), text.matches(" and ").count());
                "bands"
            }
            GeneLocationKind::Range { start, end } => {
                assert_eq!(start.as_str(), text.split('-').next().ok_or("no start")?);
                assert_eq!(
                    end.as_str(),
                    if text.starts_with('X') {
                        "Xp21.1"
                    } else {
                        "9p21.1"
                    }
                );
                "range"
            }
            GeneLocationKind::Mitochondrial => "mitochondrial",
            GeneLocationKind::Unparsed => "unparsed",
        };
        assert_eq!(parsed, kind);
    }
    Ok(())
}

#[test]
fn variant_consumer_reads_coding_representations_statements_and_frequencies() -> Check {
    // A notation example only; this test asserts no molecular interpretation.
    let notation = "NM_005228.5:c.2573T>G";
    let variant = Variant {
        ids: vec!["VCV000013961.3".parse()?].into(),
        representations: vec![Representation {
            source: Text::new("synthetic")?,
            expression: Expression::new(
                ExpressionSyntax::HgvsC,
                Text::new(notation)?,
                Some(Text::new("21.1")?),
            )?,
            build: None,
            allele: None,
        }]
        .into(),
        genes: vec![GeneRef::try_from(GeneSymbol::new("EGFR")?)?].into(),
    };
    let coding = variant
        .representations
        .iter()
        .find(|v| v.expression.syntax() == ExpressionSyntax::HgvsC)
        .expect("coding representation");
    assert_eq!(coding.expression.value().as_str(), notation);
    assert_eq!(
        coding.expression.syntax_version().map(Text::as_str),
        Some("21.1")
    );
    let subject = VariantRef::allele_hgvs(notation.parse()?);
    let attribution = Attribution {
        source: Text::new("synthetic")?,
        record: None,
        version: None,
        release: None,
        submitter: None,
        url: None,
        evaluated: None,
    };
    let statement = Statement::new(
        subject.clone(),
        "pathogenicity".parse()?,
        Term {
            label: Text::new("source label")?,
            coding: None,
        },
        attribution.clone(),
    );
    assert_eq!(
        statement.subject.hgvs().map(HgvsText::as_str),
        Some(notation)
    );
    assert!(statement.subject.id().is_none());
    let frequency: PopulationFrequency = serde_json::from_value(json!({
        "subject": subject,
        "build": Build::GRCh38,
        "dataset": "synthetic",
        "dataset_version": "v1",
        "attribution": attribution,
        "allele_count": 1,
        "allele_number": 100,
        "allele_frequency": 0.01,
    }))?;
    assert_eq!(
        frequency.subject().hgvs().map(HgvsText::as_str),
        Some(notation)
    );
    assert_eq!(frequency.dataset().as_str(), "synthetic");
    assert_eq!(frequency.dataset_version().as_str(), "v1");
    assert_eq!(frequency.build(), Build::GRCh38);
    assert_eq!(frequency.allele_count(), Some(1));
    assert_eq!(frequency.allele_number(), Some(100));
    assert_eq!(frequency.allele_frequency().map(f64::from), Some(0.01));
    assert!(frequency.filters().is_empty());
    let coordinates = TrialCoordinates::new(40.0.try_into()?, (-80.0).try_into()?)?;
    assert_eq!(f64::from(coordinates.latitude()), 40.0);
    assert_eq!(f64::from(coordinates.longitude()), -80.0);
    Ok(())
}

#[test]
fn consumers_read_each_subject_level_without_json() -> Check {
    let gene = GeneRef::try_from(GeneSymbol::new("EGFR")?)?;
    let allele = VariantRef::allele("VCV000013961.3".parse()?);
    let protein = VariantRef::protein_hgvs(
        gene.clone(),
        "NP_005219.2:p.?".parse()?,
        Some(Text::new("source label")?),
    );
    let profile = VariantRef::profile(
        Text::new("profile:1")?,
        Text::new("source profile")?,
        vec![allele, protein].into(),
    )?;
    assert_eq!(profile.record().map(Text::as_str), Some("profile:1"));
    assert_eq!(profile.label().map(Text::as_str), Some("source profile"));
    let members: Vec<_> = profile.members().collect();
    assert_eq!(
        members
            .first()
            .and_then(VariantRef::id)
            .map(ToString::to_string),
        Some("VCV000013961.3".into())
    );
    let protein = members.get(1).expect("protein member");
    assert_eq!(protein.gene(), Some(&gene));
    assert_eq!(
        protein.hgvs().map(HgvsText::as_str),
        Some("NP_005219.2:p.?")
    );
    assert_eq!(protein.label().map(Text::as_str), Some("source label"));
    assert!(protein.members().next().is_none());
    Ok(())
}

fn hashable<T: Eq + Hash + Debug + FromStr>(text: &str) -> Check
where
    T::Err: std::error::Error + 'static,
{
    let mut ids = HashSet::new();
    assert!(
        ids.insert(text.parse::<T>()?),
        "first {text} enters the collection"
    );
    assert!(
        ids.contains(&text.parse::<T>()?),
        "parsed {text} has consistent equality and hashing"
    );
    Ok(())
}

#[test]
fn identifiers_and_notation_support_hash_collections_and_borrowed_text() -> Check {
    hashable::<VariantId>("VCV000013961")?;
    hashable::<Accession>("NM_005228.5")?;
    hashable::<AccessionBase>("NM_005228")?;
    hashable::<HgvsText>("NM_005228.5:c.2573T>G")?;
    hashable::<Chromosome>("7")?;
    hashable::<RsId>("rs1")?;
    hashable::<CaId>("CA000001")?;
    hashable::<PaId>("PA1")?;
    hashable::<ClinVarVariationId>("clinvar:1")?;
    hashable::<ClinVarAlleleId>("1")?;
    hashable::<CosmicId>("COSV1")?;
    hashable::<VrsId>("ga4gh:VA.abcdefghijklmnopqrstuvwxyz012345")?;
    hashable::<Doi>("10.1/Example")?;
    hashable::<NctId>("NCT00000001")?;
    hashable::<Curie>("MONDO:0000001")?;
    hashable::<Pmid>("1")?;
    hashable::<Pmcid>("PMC1")?;
    hashable::<HgncId>("HGNC:3236")?;
    hashable::<NcbiGeneId>("1956")?;
    hashable::<EnsemblGeneId>("ENSG00000146648")?;
    hashable::<OmimId>("131550")?;
    hashable::<UniProtAcc>("P00533")?;
    hashable::<EuropePmcId>("MED1")?;
    hashable::<SemanticScholarId>("0123456789abcdef0123456789abcdef01234567")?;
    hashable::<TrialStatus>("RECRUITING")?;
    hashable::<TrialPhase>("PHASE1")?;
    hashable::<StudyType>("INTERVENTIONAL")?;
    assert_eq!("10.1/Example".parse::<Doi>()?.as_str(), "10.1/Example");
    for text in ["NCT00000001", "NCT99999999"] {
        let id: NctId = text.parse()?;
        assert_eq!(id.text(), text);
        assert_eq!(id.to_string(), text, "display retains the source spelling");
        assert_eq!(serde_json::to_value(id)?, json!(text));
    }
    assert_eq!("MONDO:0000001".parse::<Curie>()?.as_str(), "MONDO:0000001");
    assert!("unknown".parse::<TrialStatus>().is_err());
    assert_eq!(
        "RECRUITING".parse::<TrialStatus>()?.to_string(),
        "RECRUITING"
    );
    let mut expressions = HashSet::new();
    expressions.insert(Expression::hgvs(Text::new("NM_005228.5:c.2573T>G")?)?);
    assert_eq!(expressions.len(), 1);
    Ok(())
}

fn same_identity<T: Eq + Hash + Debug + FromStr + ToString + serde::Serialize>(
    prefix: &str,
) -> Check
where
    T::Err: std::error::Error + 'static,
{
    let base = format!("{prefix}000013961");
    let mut ids = HashSet::new();
    for suffix in ["", ".3", ".4"] {
        let text = format!("{base}{suffix}");
        let id: T = text.parse()?;
        assert_eq!(id.to_string(), text, "display retains the source spelling");
        assert_eq!(
            serde_json::to_value(&id)?,
            json!(text),
            "serialization retains the source version"
        );
        ids.insert(id);
    }
    assert_eq!(
        ids.len(),
        1,
        "all versions identify the same {prefix} record"
    );
    assert!(
        !ids.contains(&format!("{prefix}000013962").parse::<T>()?),
        "different {prefix} records retain distinct identities"
    );
    Ok(())
}

#[test]
fn clinvar_accessions_compare_by_record_identity_and_preserve_source_versions() -> Check {
    same_identity::<VcvId>("VCV")?;
    same_identity::<RcvId>("RCV")?;
    same_identity::<ScvId>("SCV")?;
    assert_eq!(
        "VCV000013961.3".parse::<VcvId>()?.as_str(),
        "VCV000013961.3"
    );
    Ok(())
}

#[test]
fn source_identifiers_support_hash_collections() -> Check {
    hashable::<MondoId>("MONDO:0000001")?;
    hashable::<NcitId>("C1")?;
    hashable::<DrugId>("CHEMBL1")?;
    hashable::<ReportId>("001")?;
    hashable::<HpId>("HP:0000001")?;
    hashable::<GwasStudyId>("GCST000001")?;
    hashable::<GwasAssociationId>("association:1")?;
    hashable::<GwasTraitId>("EFO_0000001")?;
    hashable::<ClinPgxAccession>("PA1")?;
    hashable::<PgxGeneKey>("EGFR")?;
    hashable::<GtrTestId>("GTR000551465.2")?;
    hashable::<WhoProductId>("0123-456-00")?;
    hashable::<PmaNumber>("P000001")?;
    hashable::<PmaSupplement>("S001")?;
    hashable::<FdaDrugApplication>("NDA000001")?;
    hashable::<DiagnosticDeviceId>("P000001/S001")?;
    hashable::<PmaSupplementCell>("")?;
    hashable::<DeviceRecallNumber>("Z-0001-2026")?;
    hashable::<ReactomeId>("R-HSA-1")?;
    hashable::<WikiPathwaysId>("WP1")?;
    hashable::<PathwayId>("R-HSA-1")?;
    hashable::<UniProtIsoform>("P00533-1")?;
    hashable::<InterProId>("IPR000001")?;
    hashable::<ComplexPortalId>("CPX-1")?;
    hashable::<StudyId>("synthetic")?;
    hashable::<StudyProfileId>("synthetic_profile")?;
    hashable::<StudySampleListId>("synthetic_samples")?;
    hashable::<CbioportalCancerTypeId>("synthetic")?;
    hashable::<StudyResourceId>("synthetic_resource")?;
    hashable::<StudyBlobSha>("0123456789abcdef0123456789abcdef01234567")?;
    hashable::<CellosaurusAccession>("CVCL_0001")?;
    hashable::<CellLineSourceId>("synthetic:1")?;
    hashable::<ChemblCellId>("CHEMBL1")?;
    hashable::<SemanticScholarAuthorId>("1")?;
    hashable::<OrcidId>("0000-0002-1825-0097")?;
    hashable::<OrcidClientId>("APP-0123456789ABCDEF")?;
    hashable::<AuthorId>("orcid:0000-0002-1825-0097")?;
    Ok(())
}
