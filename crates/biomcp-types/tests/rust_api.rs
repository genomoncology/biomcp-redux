//! Public Rust consumers read typed values without serializing them to JSON.
use biodata::*;
use serde_json::json;
use std::{collections::HashSet, fmt::Debug, hash::Hash, str::FromStr};
#[path = "rust_api/ids.rs"]
mod ids;
type Check = Result<(), Box<dyn std::error::Error>>;

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
    for claim in [statement] {
        assert_eq!(claim.subject.hgvs().map(HgvsText::as_str), Some(notation));
        assert!(claim.subject.id().is_none());
    }
    let frequency = PopulationFrequency::new(
        subject,
        Build::GRCh38,
        Text::new("synthetic")?,
        Text::new("v1")?,
        attribution,
    )
    .with_counts(Some(1), Some(100), None, None)?
    .with_frequencies(Some(0.01.try_into()?), None)?;
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

fn hashable<T: Eq + Hash + Debug + FromStr>(text: &str)
where
    T::Err: Debug,
{
    let mut ids = HashSet::new();
    assert!(ids.insert(text.parse::<T>().expect("typed identifier")));
    assert!(ids.contains(&text.parse::<T>().expect("typed identifier")));
}

#[test]
fn identifiers_and_notation_support_hash_collections_and_borrowed_text() -> Check {
    hashable::<VariantId>("VCV000013961");
    hashable::<Accession>("NM_005228.5");
    hashable::<AccessionBase>("NM_005228");
    hashable::<HgvsText>("NM_005228.5:c.2573T>G");
    hashable::<Chromosome>("7");
    hashable::<RsId>("rs1");
    hashable::<CaId>("CA000001");
    hashable::<PaId>("PA1");
    hashable::<ClinVarVariationId>("clinvar:1");
    hashable::<ClinVarAlleleId>("1");
    hashable::<CosmicId>("COSV1");
    hashable::<VrsId>("ga4gh:VA.abcdefghijklmnopqrstuvwxyz012345");
    hashable::<Doi>("10.1/Example");
    hashable::<NctId>("NCT00000001");
    hashable::<Curie>("MONDO:0000001");
    hashable::<Pmid>("1");
    hashable::<Pmcid>("PMC1");
    hashable::<HgncId>("HGNC:3236");
    hashable::<NcbiGeneId>("1956");
    hashable::<EnsemblGeneId>("ENSG00000146648");
    hashable::<OmimId>("131550");
    hashable::<UniProtAcc>("P00533");
    hashable::<EuropePmcId>("MED1");
    hashable::<SemanticScholarId>("0123456789abcdef0123456789abcdef01234567");
    hashable::<TrialStatus>("RECRUITING");
    hashable::<TrialPhase>("PHASE1");
    hashable::<StudyType>("INTERVENTIONAL");
    assert_eq!("10.1/Example".parse::<Doi>()?.as_str(), "10.1/Example");
    for text in ["NCT00000001", "NCT99999999"] {
        let id: NctId = text.parse()?;
        assert_eq!(id.text(), text);
        assert_eq!(id.to_string(), text);
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
        assert_eq!(id.to_string(), text);
        assert_eq!(serde_json::to_value(&id)?, json!(text));
        ids.insert(id);
    }
    assert_eq!(ids.len(), 1);
    assert!(!ids.contains(&format!("{prefix}000013962").parse::<T>()?));
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
