//! Public schema contract tests; file I/O stays outside the pure library.
#![cfg(feature = "schema")]
#[path = "schemas/compatibility.rs"]
mod compatibility;
#[path = "schemas/compatibility_cases.rs"]
mod compatibility_cases;
#[path = "schemas/values.rs"]
mod values;

use biodata::*;
use schemars::{JsonSchema, generate::SchemaSettings};
use serde_json::Value;
use std::{fs, path::Path};

struct Contract {
    name: &'static str,
    id: &'static str,
    generate: fn() -> Value,
}

#[expect(
    clippy::expect_used,
    reason = "Schema test setup must serialize a generated schema"
)]
fn generate<T: JsonSchema>() -> Value {
    serde_json::to_value(
        SchemaSettings::draft2020_12()
            .into_generator()
            .into_root_schema_for::<T>(),
    )
    .expect("schema serializes")
}

fn registry() -> Vec<Contract> {
    macro_rules! contracts {
        ($($ty:ty => ($name:literal, $id:literal)),+ $(,)?) => {
            vec![$(Contract { name: $name, id: $id, generate: generate::<$ty> }),+]
        };
    }
    contracts! {
        ErrorEnvelope => ("error", "biomcp.error/1"),
        GeneGet => ("gene-get", "biodata.gene-get/1"),
        GeneSearch => ("gene-search", "biodata.gene-search/1"),
        Request => ("request", "biodata.request/1"),
        Envelope<GeneCards> => ("gene-cards", "biomcp.gene_cards/1"),
        Envelope<GenePage> => ("gene-page", "biomcp.gene_page/1"),
        Envelope<SyncResult> => ("sync", "biomcp.sync/1"),
        HgncId => ("hgnc-id", "biodata.hgnc-id/1"),
        PartialDate => ("partial-date", "biodata.partial-date/1"),
        Position1 => ("position1", "biodata.position1/1"),
        Span0 => ("span0", "biodata.span0/1"),
        Text => ("text", "biodata.text/1"),
        Source => ("source", "biodata.source/1"),
        Reason => ("reason", "biodata.reason/1"),
        ProblemCode => ("problem-code", "biodata.problem-code/1"),
        Problem => ("problem", "biodata.problem/1"),
        GeneInput => ("gene-input", "biodata.gene-input/1"),
        Gene => ("gene", "biodata.gene/1"),
        GeneHeader => ("gene-header", "biomcp.gene_header/1"),
        GeneResolution => ("gene-resolution", "biodata.gene-resolution/1"),
        EntrezGeneId => ("entrez-gene-id", "biodata.entrez-gene-id/1"),
        EnsemblGeneId => ("ensembl-gene-id", "biodata.ensembl-gene-id/1"),
        GeneSymbol => ("gene-symbol", "biodata.gene-symbol/1"),
        UniProtAcc => ("uniprot-acc", "biodata.uniprot-acc/1"),
        OmimId => ("omim-id", "biodata.omim-id/1"),
        RefSeqTranscript => ("refseq-transcript", "biodata.refseq-transcript/1"),
        EnsemblTranscript => ("ensembl-transcript", "biodata.ensembl-transcript/1"),
        Cytoband => ("cytoband", "biodata.cytoband/1"),
        GenomicInterval => ("genomic-interval", "biodata.genomic-interval/1"),
        GeneStatus => ("gene-status", "biodata.gene-status/1"),
        LocusGroup => ("locus-group", "biodata.locus-group/1"),
        LocusType => ("locus-type", "biodata.locus-type/1"),
        MatchKind => ("match-kind", "biodata.match-kind/1"),
        Match => ("gene-match", "biodata.gene-match/1"),
        ManeSelect => ("mane-select", "biodata.mane-select/1"),
        Summary => ("gene-summary", "biodata.gene-summary/1"),
        SummarySource => ("summary-source", "biodata.summary-source/1"),
        HgncStatus => ("hgnc-status", "biodata.hgnc-status/1"),
        Build => ("build", "biodata.build/1"),
        Strand => ("strand", "biodata.strand/1"),
        Contig => ("contig", "biodata.contig/1"),
    }
}

#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "Schema contract tests own file and environment I/O outside the pure library"
)]
fn checked_in_schemas_match_types() {
    let update = std::env::var_os("UPDATE_SCHEMAS").is_some_and(|value| value == "1");
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas");
    // Collect and check the whole registry before writing any contract.
    let pending: Vec<_> = registry()
        .into_iter()
        .map(|contract| {
            let mut new = (contract.generate)();
            new["$id"] = contract.id.into();
            compatibility::check(&new, &new)
                .unwrap_or_else(|error| panic!("{}: {error}", contract.name));
            let path = root.join(format!("{}.json", contract.name));
            if path.exists() {
                let old: Value =
                    serde_json::from_str(&fs::read_to_string(&path).expect("read contract"))
                        .expect("parse contract");
                compatibility::check(&old, &new)
                    .unwrap_or_else(|error| panic!("{}: {error}", contract.name));
            } else {
                assert!(update, "missing {}", path.display());
            }
            let bytes = format!(
                "{}\n",
                serde_json::to_string_pretty(&new).expect("serialize schema")
            );
            (path, bytes)
        })
        .collect();
    if update {
        fs::create_dir_all(&root).expect("create schema directory");
    }
    for (path, bytes) in pending {
        if update {
            fs::write(&path, bytes).expect("write contract");
        } else {
            assert_eq!(
                fs::read_to_string(&path).expect("read contract"),
                bytes,
                "schema drift at {}; review compatibility before UPDATE_SCHEMAS=1",
                path.display()
            );
        }
    }
}
