//! Public gene entities keep source identity, ambiguity and complete prose.
use biodata::*;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;
use std::fmt::Debug;
type Check = Result<(), Box<dyn std::error::Error>>;
fn round_trip<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: &T) -> Check {
    assert_eq!(
        serde_json::from_value::<T>(serde_json::to_value(value)?)?,
        *value
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
        mane_select: None,
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
    entity.entrez_id = Some(EntrezGeneId::new(673)?);
    entity.ensembl_id = Some("ENSG00000157764.12".parse()?);
    entity.locus_type = Some(LocusType::GeneWithProteinProduct);
    entity.previous_symbols = vec![GeneSymbol::new("BRAF1")?].into_boxed_slice();
    entity.aliases = vec![Text::new("alias with spaces / symbols")?].into_boxed_slice();
    entity.uniprot = vec!["P15056".parse()?].into_boxed_slice();
    entity.omim = vec!["164757".parse()?].into_boxed_slice();
    entity.mane_select = Some(ManeSelect::new(
        "NM_004333.6".parse()?,
        "ENST00000646891.2".parse()?,
    )?);
    let prose = "Complete source summary. ".repeat(1000);
    entity.summary = Some(Summary {
        text: Text::new(&prose)?,
        source: SummarySource::RefSeq,
        dated: Some("2026-10".parse()?),
    });
    entity.spans = vec![GenomicInterval {
        build: Build::Grch38,
        contig: Contig::new("7")?,
        span: Span0::new(140719327, 140924929)?,
        strand: Some(Strand::Reverse),
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
        "mane_select",
        "summary",
    ] {
        assert!(absent.get(field).is_none());
    }
    assert_eq!(
        serde_json::to_value(GeneResolution::NotFound {})?,
        json!({"resolution":"not_found"})
    );
    assert!(
        serde_json::from_value::<GeneResolution>(json!({"resolution":"not_found","gene":{}}))
            .is_err()
    );
    assert!(serde_json::from_value::<Gene>(json!({})).is_err());
    round_trip(&header()?)?;
    Ok(())
}
#[test]
fn mane_requires_curated_versioned_transcripts_at_its_boundary() -> Check {
    let valid = ManeSelect::new("NM_004333.6".parse()?, "ENST00000646891.2".parse()?)?;
    assert_eq!(valid.refseq().version(), Some(6));
    assert_eq!(valid.ensembl().version(), Some(2));
    round_trip(&valid)?;
    for (refseq, ensembl) in [
        ("NM_004333", "ENST00000646891.2"),
        ("NM_004333.6", "ENST00000646891"),
        ("XM_004333.6", "ENST00000646891.2"),
        ("XR_004333.6", "ENST00000646891.2"),
    ] {
        assert!(ManeSelect::new(refseq.parse()?, ensembl.parse()?).is_err());
        assert!(
            serde_json::from_value::<ManeSelect>(json!({"refseq":refseq,"ensembl":ensembl}))
                .is_err()
        );
    }
    assert!(ManeSelect::new("NR_123456.1".parse()?, "ENST00000646891.2".parse()?).is_ok());
    assert!(
        serde_json::from_value::<ManeSelect>(
            json!({"refseq":"NM_004333.6","ensembl":"ENST00000646891.2","extra":true})
        )
        .is_err()
    );
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
    for build in [Build::Grch37, Build::Grch38, Build::Unstated] {
        for strand in [None, Some(Strand::Forward), Some(Strand::Reverse)] {
            let interval = GenomicInterval {
                build,
                strand,
                contig: "7".parse()?,
                span: Span0::new(0, 0)?,
            };
            round_trip(&interval)?;
            let json = serde_json::to_value(interval)?;
            assert_eq!(json.get("strand").is_none(), strand.is_none());
        }
    }
    for value in [
        json!({"contig":"7","span":{"start":0,"end":1}}),
        json!({"build":"GRCh38","contig":"7","span":{"start":2,"end":1}}),
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
