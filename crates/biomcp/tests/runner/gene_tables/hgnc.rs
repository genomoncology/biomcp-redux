use super::super::response_cases::checked_case;
use super::{Check, case, tsv};
use serde_json::json;

pub(super) fn check() -> Check {
    vocabulary()?;
    optional()?;
    locations()?;
    required()?;
    withdrawn()?;
    case(
        "gene/main",
        &["search", "gene", "MLL2"],
        |source, record| {
            if source == "hgnc" {
                tsv(record, "KMT2D", &[("alias_symbol", "MLL2")]);
                tsv(record, "H3P10", &[("alias_symbol", "MLL2")]);
            }
        },
        json!({"/result/items/*/hgnc_id": ["HGNC:7133", "HGNC:15840", "HGNC:43802"],
        "/result/items/0/match/kind": "previous_symbol"}),
        0,
    )?;
    case(
        "gene/main",
        &["get", "gene", "100874204"],
        |_, _| {},
        json!({"/result/items/0/answer/candidates/*/hgnc_id": ["HGNC:40244", "HGNC:55559"],
            "/result/items/0/answer/candidates/*/match/kind": ["entrez_id", "entrez_id"]}),
        0,
    )?;
    case(
        "gene/main",
        &["search", "gene", "--band", "9p21", "--limit", "50"],
        |_, _| {},
        json!({"/page/total": 132, "/result/items/*/symbol": {"includes": "CDKN2A"}}),
        0,
    )?;
    case(
        "gene/main",
        &["search", "gene", "BRAF proto oncogene"],
        |_, _| {},
        json!({"/result/items/*/symbol": {"includes": "BRAF"}}),
        0,
    )
}

fn optional() -> Check {
    for (field, value, problem) in [
        ("locus_type", "new provider locus", "locus_type"),
        ("entrez_id", "0", "entrez_id"),
        ("ensembl_gene_id", "bad", "ensembl_id"),
        ("uniprot_ids", "bad", "uniprot"),
        ("omim_id", "bad", "omim"),
        ("prev_symbol", "bad symbol", "previous_symbols"),
        ("alias_symbol", " ", "aliases"),
        ("mane_select", "ENST00000646891|NM_004333.6", "mane_select"),
    ] {
        let answer = checked_case(
            "gene/main",
            &["get", "gene", "BRAF"],
            |source, record| {
                if source == "hgnc" {
                    tsv(record, "BRAF", &[(field, value)]);
                }
            },
            json!({"/result/items/0/answer/gene/hgnc_id": "HGNC:1097",
            "/problems": {"count": 1},
            "/problems/*/field": {"includes": problem},
            "/problems/*/record": {"includes": "HGNC:1097"}}),
            0,
        )?;
        let mapped = match field {
            "ensembl_gene_id" => "ensembl_id",
            "uniprot_ids" => "uniprot",
            "omim_id" => "omim",
            "prev_symbol" => "previous_symbols",
            "alias_symbol" => "aliases",
            other => other,
        };
        if answer
            .pointer(&format!("/result/items/0/answer/gene/{mapped}"))
            .is_some_and(|value| !value.is_null() && !value.as_array().is_some_and(Vec::is_empty))
        {
            return Err(format!("malformed {field} survived in the gene answer").into());
        }
    }
    Ok(())
}

fn required() -> Check {
    for (field, value, input) in [
        ("hgnc_id", "HGNC:0", "BRAF"),
        ("symbol", "bad symbol", "HGNC:1097"),
        ("locus_group", "new group", "HGNC:1097"),
        ("status", "new status", "HGNC:1097"),
        ("name", " ", "HGNC:1097"),
    ] {
        case(
            "gene/main",
            &["get", "gene", input],
            |source, record| {
                if source == "hgnc" {
                    tsv(record, "BRAF", &[(field, value)]);
                }
            },
            json!({"/result/items/0/answer/resolution": "not_found",
            "/problems/*/field": {"includes": field}}),
            1,
        )?;
    }
    Ok(())
}

fn withdrawn() -> Check {
    for (input, targets) in [
        ("A2MR", vec!["HGNC:6692"]),
        ("HGNC:9", vec!["HGNC:6692"]),
        ("CBBM", vec!["HGNC:4206", "HGNC:9936"]),
        ("ADCR", vec!["HGNC:12797"]),
    ] {
        case(
            "gene/main",
            &["get", "gene", input],
            |_, _| {},
            json!({"/result/items/0/answer/resolution": "withdrawn",
                "/result/items/0/answer/hgnc_status": "merged_split",
                "/result/items/0/answer/merged_into": targets}),
            0,
        )?;
    }
    for (field, value, input) in [
        ("STATUS", "new status", "A2MR"),
        ("HGNC_ID", "bad", "A2MR"),
        ("WITHDRAWN_SYMBOL", "bad symbol", "HGNC:9"),
    ] {
        case(
            "gene/main",
            &["get", "gene", input],
            |source, record| {
                if source == "hgnc" {
                    tsv(record, "A2MR", &[(field, value)]);
                }
            },
            json!({"/result/items/0/answer/resolution": "not_found",
            "/problems/*/field": {"includes": match field { "STATUS" => "hgnc_status", "HGNC_ID" => "hgnc_id", _ => "symbol" }}}),
            1,
        )?;
    }
    case(
        "gene/main",
        &["get", "gene", "A2MR"],
        |source, record| {
            if source == "hgnc" {
                tsv(
                    record,
                    "A2MR",
                    &[(
                        "MERGED_INTO_REPORT(S) (i.e HGNC_ID|SYMBOL|STATUS)",
                        "HGNC:6692|LRP1|Approved, bad|bad|Approved",
                    )],
                );
            }
        },
        json!({"/result/items/0/answer/merged_into": ["HGNC:6692"], "/problems": {"count": 1}}),
        0,
    )
}

fn vocabulary() -> Check {
    let rows = [
        ("gene with protein product", "gene_with_protein_product"),
        ("RNA, cluster", "rna_cluster"),
        ("RNA, long non-coding", "long_non_coding_rna"),
        ("RNA, micro", "micro_rna"),
        ("RNA, misc", "misc_rna"),
        ("RNA, ribosomal", "ribosomal_rna"),
        ("RNA, small nuclear", "small_nuclear_rna"),
        ("RNA, small nucleolar", "small_nucleolar_rna"),
        ("RNA, transfer", "transfer_rna"),
        ("RNA, vault", "vault_rna"),
        ("RNA, Y", "y_rna"),
        ("pseudogene", "pseudogene"),
        ("complex locus constituent", "complex_locus_constituent"),
        ("endogenous retrovirus", "endogenous_retrovirus"),
        ("fragile site", "fragile_site"),
        ("immunoglobulin gene", "immunoglobulin_gene"),
        ("immunoglobulin pseudogene", "immunoglobulin_pseudogene"),
        ("readthrough", "readthrough"),
        ("region", "region"),
        ("T cell receptor gene", "t_cell_receptor_gene"),
        ("T cell receptor pseudogene", "t_cell_receptor_pseudogene"),
        ("unknown", "unknown"),
        ("virus integration site", "virus_integration_site"),
    ];
    for (raw, mapped) in rows {
        case(
            "gene/main",
            &["get", "gene", "BRAF"],
            |source, record| {
                if source == "hgnc" {
                    tsv(record, "BRAF", &[("locus_type", raw)]);
                }
            },
            json!({"/result/items/0/answer/gene/locus_type": mapped}),
            0,
        )?;
    }
    case(
        "gene/main",
        &["get", "gene", "BRAF"],
        |source, record| {
            if source == "hgnc" {
                tsv(
                    record,
                    "BRAF",
                    &[("mane_select", "ENST00000646891.2|NR_186305.1")],
                );
            }
        },
        json!({"/result/items/0/answer/gene/mane/0/refseq": "NR_186305.1"}),
        0,
    )
}

fn locations() -> Check {
    for (symbol, id, name, location) in [
        ("SHOX", "HGNC:10853", "SHOX homeobox", "Xp22.33 and Yp11.32"),
        (
            "CRLF2",
            "HGNC:14281",
            "cytokine receptor like factor 2",
            "Xp22.3 and Yp11.3",
        ),
        (
            "MT-ND1",
            "HGNC:7455",
            "mitochondrially encoded NADH:ubiquinone oxidoreductase core subunit 1",
            "mitochondria",
        ),
        ("DMD", "HGNC:2928", "dystrophin", "Xp21.2-p21.1"),
        ("BRAF", "HGNC:1097", "BRAF name", "publisher location text"),
    ] {
        let answer = checked_case(
            "gene/main",
            &["get", "gene", symbol, "--disable-sources", "mygene"],
            |source, record| {
                if source == "hgnc" && symbol != "DMD" {
                    tsv(
                        record,
                        "BRAF",
                        &[
                            ("symbol", symbol),
                            ("hgnc_id", id),
                            ("name", name),
                            ("location", location),
                        ],
                    );
                }
            },
            json!({"/result/items/0/answer/gene/symbol": symbol,
                "/result/items/0/answer/gene/location": location}),
            0,
        )?;
        if answer.get("problems").is_some() {
            return Err(format!("valid HGNC location for {symbol} produced problems").into());
        }
    }
    Ok(())
}
