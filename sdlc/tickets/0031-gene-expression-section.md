# 0031 — Add the expression section from the Human Protein Atlas

Status: OPEN.

Milestone: 1

## Outcome

`get gene <input> expression` returns HPA's RNA tissue and cancer specificity with their tissue-level values and the protein's subcellular location, fetched by HGNC's Ensembl ID, proven by gene.md case 18 under its ceilings.

Depends on: 0026

- Starts from: 0026's section pattern; gene.md sections 2 (example 17), 3 (HPA row), 5 (`hpa` renamed `expression`, GTEx dropped), 6 and 8; biodata `Gene.ensembl_id`.
- Changes: `crates/biodata/src/source.rs`, `crates/biodata/src/query/gene.rs`, `crates/biodata/src/response/gene.rs`, `crates/biodata/src/response/gene/expression.rs`, `crates/biodata/schemas/**`, `crates/biomcp-adapters/src/lib.rs`, `crates/biomcp-adapters/src/quotas.rs`, `crates/biomcp-adapters/src/hpa/**`, `crates/biomcp-query/src/gene/sections.rs`, `crates/biomcp-query/src/gene/sections/expression.rs`, `tests/cases/gene/sections/expression-*.yaml`, `tests/replies/gene/expression/**`, `tests/replies/gene/all/hpa/**`, `ROADMAP.md`.
- Checked boundaries, 2026-10-24: `GET https://www.proteinatlas.org/ENSG00000066405.json` returned 200 `application/json`, 7,457 bytes, one object of 119 keys with spaces in their names, including `"Gene": "CLDN18"`, `"Ensembl": "ENSG00000066405"`, `"RNA tissue specificity": "Group enriched"`, `"RNA tissue distribution": "Detected in some"`, `"RNA tissue specific nTPM": {"lung": "237.8", "stomach 1": "684.2"}`, `"RNA cancer specificity": "Group enriched"`, `"RNA cancer specific pTPM": {"Pancreatic Adenocarcinoma (TCGA)": "178.0", "Stomach Adenocarcinoma (TCGA)": "285.4"}`, `"Subcellular main location": ["Cell Junctions"]`, `"Subcellular additional location": null` and `"Protein class": ["Predicted membrane proteins"]`. Numbers are strings and absent values are `null`. The stomach tissue is named "stomach 1". An unknown Ensembl ID and MIR21's ENSG00000284190 each returned 404 `text/html`. No field or header names the HPA release.
- Unchecked: HPA's published rate limit and license text beyond gene.md's CC BY-SA 4.0 row. Declare a conservative local quota `hpa` of 1 request per second and 1 in flight in quotas.rs.
- PM decision: 2026-10-24. One request per resolved card with an Ensembl ID. `Section<Expression>` holds `rna_tissue_specificity`, `rna_tissue_distribution`, the tissue-specific nTPM pairs, `rna_cancer_specificity`, the cancer-specific pTPM pairs, `subcellular_main` and `subcellular_additional`, in that order; numbers parse to decimals, and a bad number drops that pair with a problem. Tissue names are kept as HPA writes them. A 404 is `Empty` with no record, because architecture 3.5 makes the source's own miss an answer and keeps `NotApplicable` for the record's facts; this overrides gene.md section 5's "not applicable" cell. `NotApplicable` when HGNC lists no Ensembl ID. No release field, because the reply carries none. Prognostics, single-cell, brain, blood and antibody fields are left out, because no example needs them.
- Keeps: 0026's section behavior. No GTEx, no cell-line expression. Files under 300 lines.
- Proof: Record live once into `tests/replies/gene/expression/` with a trimmed HGNC slice holding CLDN18 and MIR21, and add TP53's reply to `tests/replies/gene/all/hpa/`. `get gene CLDN18 expression` (case 18) asserts `symbol` CLDN18, `expression` state data, a tissue name matching `^stomach`, under 3,500 bytes, with `/result/items/0/sections/expression` under 150 tokens; it runs through MCP with parity. `get gene MIR21 expression` asserts state empty over the recorded 404. The five failure cases of architecture 5.5 on HPA assert partial status, `expression` unavailable with its reason, identity present and a later offline step. Mark edited recordings synthetic. Then make landing-gate runs through pm job after current main. The landing commit ticks Human Protein Atlas in ROADMAP.md.
- Defers: cell-line expression goes to the cell line slice.
- Evidence: Builds in parallel with 0027–0030 and 0032; the shared registration lines are listed in 0026's Evidence.
