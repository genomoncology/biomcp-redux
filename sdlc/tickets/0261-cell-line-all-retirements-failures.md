# 0261 — Add the cell line all section, retire old cell line options and prove cell line failure cases

Status: OPEN.

Milestone: 4

Depends on: 0259
Depends on: 0260

## Outcome

`get cell-line CVCL_2119 all` returns each named section's bounded first page, including the one ChEMBL request, and never the assay count. Every 0.9 cell line command, option and section that cell-line.md and the cell line rulings retire or defer returns a fixed usage error that names the supported route or the reason, and makes zero requests. Cellosaurus and ChEMBL each report a failed outcome, never an empty one, on a 429, 500, timeout, broken JSON or HTTP 200 HTML body. A Cellosaurus identity failure makes the card failed, and a ChEMBL failure inside `all` makes the card partial with a problem and a fresh step.

- Why: The cell line rulings keep `all` for consistency with protein and pathway, retire name normalization and reranking, defer PharmacoDB and the HPA expression route, and the slice rules require failure and identity cases for every adapter.
- Starts from: cell-line.md's legacy accounting and the failure rows of its case map, with the cell line rulings; the existing retirement table, including the drug `cell-lines` retirement; the sections from 0259 and 0260.
- Rules: `all` adds each section's first page and never an unbounded array, and its composite ceiling comes from its measured output. `-q` and `--query` stay. `cell-line drug-response` and the `drug_response` section say PharmacoDB is deferred until it publishes a data reuse grant. `gene cell-lines` says the HPA expression route is deferred until an exact cell identity join exists. `drug cell-lines` keeps its existing retirement. No retirement rewrites the query or joins by name. No command downloads, installs or runs a screening, analysis or other bioinformatics tool.
- Proof: outside-in cases first. Cases assert `all` for CVCL_2119 and CVCL_0004 with every section's first page and its measured composite ceiling, and a disabled `chembl` source inside `all` that reads as skipped. A case per retired or deferred spelling asserts its message with zero provider requests. Synthetic 429, 500, timeout, broken JSON and HTTP 200 HTML replies for `cellosaurus` and `chembl`, labeled synthetic, assert a failed outcome with source and terms. C22 asserts a successful empty distinct from each failure, and C23 asserts no retry through a verification page. C24 asserts Cellosaurus 57.0 and ChEMBL_37 with independent dates, and a failed release read that labels retrieval time. Source disable before cache and network is asserted for each source. Each case has a size ceiling. This ticket adds no filter or exact-ID get and makes no provider call; the filters and exact-ID gets it relies on have known-positive captures through one bounded keyless pm job in 0256, 0258, 0259 and 0260. No licensed content enters the repo. Then make landing-gate runs through pm job.
- Defers: nothing.
- Changes: the cell line section code, `crates/biomcp/src/retired.rs` and its cell line module, cases and `cell-line.md`.
