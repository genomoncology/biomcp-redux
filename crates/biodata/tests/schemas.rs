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
        VariantGet => ("variant-get", "biodata.variant-get/1"),
        Envelope<VariantCard> => ("variant-card", "biomcp.variant_card/1"),
        HttpHealth => ("http-health", "biomcp.http-health/1"),
        ArticleEvidence => ("article-evidence", "biodata.article-evidence/1"),
        ArticleRecommend => ("article-recommend", "biodata.article-recommend/1"),
        Envelope<ArticleEvidenceResult> => ("article-evidence-result", "biomcp.article_evidence/1"),
        Envelope<ArticleRecommendations> => ("article-recommendations", "biomcp.article_recommendations/1"),
        ArticlePdf => ("article-pdf", "biodata.article-pdf/1"),
        Envelope<ArticlePdfResult> => ("article-pdf-result", "biomcp.article_pdf/1"),
        ArticleSupplement => ("article-supplement", "biodata.article-supplement/1"),
        Envelope<ArticleSupplementResult> => ("article-supplement-result", "biomcp.article_supplement/1"),
        Envelope<Health> => ("health", "biomcp.health/1"),
        Markdown => ("markdown", "biodata.markdown/1"),
        ArticleText => ("article-text", "biodata.article-text/2"),
        Envelope<ArticleTextResult> => ("article-text-result", "biomcp.article_text/2"),
        ArticleEdges => ("article-edges", "biodata.article-edges/2"),
        Envelope<ArticleEdgePage> => ("article-edge-page", "biomcp.article_edges/2"),
        ArticleGet => ("article-get", "biodata.article-get/2"),
        Article => ("article", "biodata.article/1"),
        Envelope<ArticleCards> => ("article-cards", "biomcp.article_cards/2"),
        ArticleSearch => ("article-search", "biodata.article-search/1"),
        ArticleHeader => ("article-header", "biodata.article_header/1"),
        Envelope<ArticlePage> => ("article-page", "biomcp.article_page/1"),
        EuropePmcId => ("europepmc-id", "biodata.europepmc-id/1"),
        Pmid => ("pmid", "biodata.pmid/1"),
        Pmcid => ("pmcid", "biodata.pmcid/1"),
        Doi => ("doi", "biodata.doi/1"),
        Envelope<Help> => ("help", "biomcp.help/1"),
        Envelope<KeyHelp> => ("key", "biomcp.key/1"),
        Envelope<RegisteredKeys> => ("registered-keys", "biomcp.registered-keys/1"),
        ErrorEnvelope => ("error", "biomcp.error/1"),
        GeneGet => ("gene-get", "biodata.gene-get/1"),
        GeneSearch => ("gene-search", "biodata.gene-search/1"),
        Request => ("request", "biodata.request/2"),
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

#[test]
fn consumed_queries_stay_inside_the_flat_command_grammar() {
    for schema in [
        generate::<GeneGet>(),
        generate::<GeneSearch>(),
        generate::<ArticleSearch>(),
    ] {
        assert_eq!(schema["type"], "object");
        assert!(schema.get("oneOf").is_none());
        if schema.get("anyOf").is_some() {
            assert!(
                compatibility::selectors(&schema)
                    .expect("selector union")
                    .is_some()
            );
        }
        for property in schema["properties"]
            .as_object()
            .expect("query properties")
            .values()
        {
            flat_value(&schema, property);
        }
    }
}
#[expect(
    clippy::expect_used,
    reason = "The grammar contract must fail on unresolved generated schema references"
)]
fn flat_value(root: &Value, node: &Value) {
    if let Some(reference) = node.get("$ref").and_then(Value::as_str) {
        flat_value(
            root,
            root.pointer(reference.strip_prefix('#').expect("local reference"))
                .expect("resolved reference"),
        );
    } else if let Some(branches) = node.get("anyOf").and_then(Value::as_array) {
        assert_eq!(branches.len(), 2);
        assert_eq!(
            branches
                .iter()
                .filter(|branch| branch["type"] == "null")
                .count(),
            1
        );
        for branch in branches.iter().filter(|branch| branch["type"] != "null") {
            flat_value(root, branch);
        }
    } else if node["type"] == "array" {
        let items = node.get("items").expect("array items");
        assert_ne!(items["type"], "array");
        flat_value(root, items);
    } else {
        assert!(
            ["string", "integer", "boolean"].contains(&node["type"].as_str().expect("scalar type"))
        );
    }
}
