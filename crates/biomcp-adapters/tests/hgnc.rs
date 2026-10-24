//! Temporary HGNC public adapter contracts until command cases own these paths.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Checked fixture setup must fail immediately on a malformed shape"
)]
use biomcp_adapters::hgnc::{self, HgncBundle};
use biomcp_fetch::{Fetch, FetchConfig, Find, KeyKind, Mode, ReplayCache};
use std::{path::PathBuf, time::Duration};
const COMPLETE: &str = "0887a33cc809657a2a564d65e8cbfcddf45adc3810a8bfb0653e03b418cf2953.json";
const WITHDRAWN: &str = "22878e5e68ac51896f2dc0b9b73db2d72851de7db5d4136bf37693b87ce5a4d6.json";
fn captured() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/hgnc")
}
fn replay(dir: PathBuf) -> Fetch {
    Fetch::open(FetchConfig {
        mode: Mode::Replay { dir },
        ..Default::default()
    })
    .unwrap()
}
#[test]
fn captured_single_band_prefixes_return_headers_and_snapshot_totals() {
    let fetch = replay(captured());
    let leg = fetch.leg(&hgnc::SPEC, fetch.deadline(Duration::from_secs(120)));
    for (prefix, total) in [("9p21", 132)] {
        let headers = hgnc::in_band(&leg, prefix).unwrap();
        assert!(headers.problems.is_empty());
        assert_eq!(headers.value.len(), total);
        for symbol in ["CDKN2A", "CDKN2B", "MTAP"] {
            assert!(headers.value.iter().any(|g| g.symbol.as_str() == symbol));
        }
        for header in headers.value {
            let band = header.location.unwrap();
            assert!(band.as_str().starts_with(prefix));
            assert_eq!(header.r#match.unwrap().text.as_str(), band.as_str());
        }
    }
    assert!(hgnc::in_band(&leg, "%_").unwrap().value.is_empty());
}
#[test]
fn captured_source_field_tables_and_words_keep_checked_source_values() {
    let fetch = replay(captured());
    let leg = fetch.leg(&hgnc::SPEC, fetch.deadline(Duration::from_secs(120)));
    let all = hgnc::lookup(&leg, Find::Prefix(KeyKind::Id, "HGNC:")).unwrap();
    assert!(all.problems.is_empty(), "{:?}", all.problems);
    let mut mapped: Vec<_> = all
        .value
        .iter()
        .filter_map(|g| g.locus_type.map(|v| v.to_string()))
        .collect();
    mapped.sort();
    mapped.dedup();
    assert_eq!(
        mapped,
        [
            "complex_locus_constituent",
            "endogenous_retrovirus",
            "fragile_site",
            "gene_with_protein_product",
            "immunoglobulin_gene",
            "immunoglobulin_pseudogene",
            "long_non_coding_rna",
            "micro_rna",
            "misc_rna",
            "pseudogene",
            "readthrough",
            "region",
            "ribosomal_rna",
            "rna_cluster",
            "small_nuclear_rna",
            "small_nucleolar_rna",
            "t_cell_receptor_gene",
            "t_cell_receptor_pseudogene",
            "transfer_rna",
            "unknown",
            "vault_rna",
            "virus_integration_site",
            "y_rna"
        ]
    );
    let braf = hgnc::lookup(&leg, Find::Words("BRAF proto oncogene")).unwrap();
    assert!(braf.value.iter().any(|g| g.symbol.as_str() == "BRAF"));
    let fale = hgnc::lookup(&leg, Find::Exact(KeyKind::Symbol, "FALEC")).unwrap();
    assert_eq!(
        fale.value[0]
            .mane_select
            .as_ref()
            .unwrap()
            .refseq()
            .to_string(),
        "NR_186305.1"
    );
    // Xref families cannot collide with one another or with an unnamespaced key.
    for key in ["673", "ensembl:673", "entrez:ENSG00000157764"] {
        assert!(
            hgnc::lookup(&leg, Find::Exact(KeyKind::Xref, key))
                .unwrap()
                .value
                .is_empty()
        );
    }
}
#[test]
fn typed_bundle_sync_populates_only_the_owned_replay_cache_for_offline_use() {
    let cache = ReplayCache::temporary().unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay { dir: captured() },
        replay_cache: Some(cache.clone()),
        ..Default::default()
    })
    .unwrap();
    let complete = hgnc::sync(&fetch, HgncBundle::Complete).unwrap();
    let withdrawn = hgnc::sync(&fetch, HgncBundle::Withdrawn).unwrap();
    assert_eq!(complete.name, "hgnc");
    assert_eq!(complete.rows, 170);
    assert_eq!(withdrawn.name, "hgnc-withdrawn");
    assert_eq!(withdrawn.rows, 5);
    drop(fetch);
    let offline = Fetch::open(FetchConfig {
        mode: Mode::Offline,
        replay_cache: Some(cache),
        ..Default::default()
    })
    .unwrap();
    let leg = offline.leg(&hgnc::SPEC, offline.deadline(Duration::from_secs(5)));
    assert_eq!(
        hgnc::lookup(&leg, Find::Exact(KeyKind::Symbol, "BRAF"))
            .unwrap()
            .value
            .len(),
        1
    );
}
#[expect(
    clippy::disallowed_methods,
    reason = "Read captured public text and write explicitly synthetic fixtures into a case-owned directory"
)]
fn synthetic(file: &str, symbol: &str, changes: &[(&str, &str)]) -> tempfile::TempDir {
    let mut recording: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(captured().join("hgnc").join(file)).unwrap())
            .unwrap();
    let text = recording["exchanges"][0]["body"]["value"].as_str().unwrap();
    let mut lines = text.lines();
    let header = lines.next().unwrap();
    let names: Vec<_> = header.split('\t').collect();
    let symbol_col = names
        .iter()
        .position(|v| matches!(*v, "symbol" | "WITHDRAWN_SYMBOL"))
        .unwrap();
    let mut body = vec![header.to_owned()];
    for line in lines {
        let mut cells: Vec<_> = line.split('\t').map(str::to_owned).collect();
        if cells[symbol_col] == symbol {
            for (field, value) in changes {
                let i = names.iter().position(|v| v == field).unwrap();
                cells[i] = (*value).into();
            }
        }
        body.push(cells.join("\t"));
    }
    recording["exchanges"][0]["body"]["value"] = format!("{}\n", body.join("\n")).into();
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("hgnc")).unwrap();
    std::fs::write(
        dir.path().join("hgnc").join(file),
        serde_json::to_vec(&recording).unwrap(),
    )
    .unwrap();
    dir
}
#[test]
fn synthetic_unknown_optional_fields_leave_the_gene_and_visible_problems() {
    let edits = [
        ("locus_type", "new provider locus", "locus_type"),
        ("entrez_id", "0", "entrez_id"),
        ("ensembl_gene_id", "bad", "ensembl_id"),
        ("uniprot_ids", "bad", "uniprot"),
        ("omim_id", "bad", "omim"),
        ("prev_symbol", "bad symbol", "previous_symbols"),
        ("alias_symbol", " ", "aliases"),
        ("mane_select", "ENST00000646891|NM_004333.6", "mane_select"),
    ];
    for (field, value, problem) in edits {
        let dir = synthetic(COMPLETE, "BRAF", &[(field, value)]);
        let fetch = replay(dir.path().into());
        let leg = fetch.leg(&hgnc::SPEC, fetch.deadline(Duration::from_secs(120)));
        let mapped = hgnc::lookup(&leg, Find::Exact(KeyKind::Id, "HGNC:1097")).unwrap();
        assert_eq!(mapped.value.len(), 1);
        assert_eq!(mapped.problems.len(), 1, "{field}");
        assert_eq!(mapped.problems[0].field.as_ref(), problem);
        assert_eq!(
            mapped.problems[0].record.as_ref().unwrap().as_str(),
            "HGNC:1097"
        );
        let value = serde_json::to_value(&mapped.value[0]).unwrap();
        let field = match field {
            "ensembl_gene_id" => "ensembl_id",
            "uniprot_ids" => "uniprot",
            "omim_id" => "omim",
            "prev_symbol" => "previous_symbols",
            "alias_symbol" => "aliases",
            other => other,
        };
        assert!(value[field].is_null() || value[field].as_array().is_some_and(Vec::is_empty));
    }
}
#[test]
fn synthetic_bad_required_identity_fields_drop_the_record_with_problems() {
    for (field, value, kind, key) in [
        ("hgnc_id", "HGNC:0", KeyKind::Symbol, "BRAF"),
        ("symbol", "bad symbol", KeyKind::Id, "HGNC:1097"),
        ("locus_group", "new group", KeyKind::Id, "HGNC:1097"),
        ("status", "new status", KeyKind::Id, "HGNC:1097"),
        ("name", " ", KeyKind::Id, "HGNC:1097"),
    ] {
        let dir = synthetic(COMPLETE, "BRAF", &[(field, value)]);
        let fetch = replay(dir.path().into());
        let leg = fetch.leg(&hgnc::SPEC, fetch.deadline(Duration::from_secs(120)));
        let mapped = hgnc::lookup(&leg, Find::Exact(kind, key)).unwrap();
        assert!(mapped.value.is_empty());
        assert_eq!(mapped.problems.len(), 1);
        assert_eq!(mapped.problems[0].field.as_ref(), field);
    }
}
#[test]
fn synthetic_withdrawn_mapping_rejects_identity_and_keeps_valid_target_siblings() {
    for (field, value, expected) in [
        ("STATUS", "new status", 0),
        ("HGNC_ID", "bad", 0),
        ("WITHDRAWN_SYMBOL", "bad symbol", 0),
        (
            "MERGED_INTO_REPORT(S) (i.e HGNC_ID|SYMBOL|STATUS)",
            "HGNC:6692|LRP1|Approved, bad|bad|Approved",
            1,
        ),
    ] {
        let dir = synthetic(WITHDRAWN, "A2MR", &[(field, value)]);
        let fetch = replay(dir.path().into());
        let leg = fetch.leg(&hgnc::SPEC, fetch.deadline(Duration::from_secs(120)));
        let input = biodata::GeneInput::new(if field == "WITHDRAWN_SYMBOL" {
            "HGNC:9"
        } else {
            "A2MR"
        })
        .unwrap();
        let rows = hgnc::withdrawn(&leg, &input).unwrap();
        assert_eq!(rows.value.len(), expected);
        assert_eq!(rows.problems.len(), 1);
        if expected == 1 {
            assert_eq!(rows.value[0].merged_into.len(), 1);
            assert_eq!(rows.value[0].merged_into[0].get(), 6692);
        }
    }
}
