# 0260 — Add the cell line ChEMBL section and the assay count command

Status: OPEN.

Milestone: 4

Depends on: 0258

## Outcome

`get cell-line CVCL_2119 chembl` sends one ChEMBL cell request on the exact `cellosaurus_id` and returns every matching record with its ChEMBL cell ID, native name, organism and taxon, tissue, description and optional CLO and EFO identifiers. `cell-line assay-count CHEMBL3706573` sends one ChEMBL assay request with the exact `cell_chembl_id`, `limit=1` and `offset=0`, and returns the cell ID and the native assay total with source attribution and the ChEMBL release. The ChEMBL section offers one assay-count step per returned record within the next-step bound.

- Why: Questions C14 to C17 ask whether ChEMBL holds an exact record for a Cellosaurus accession and how many ChEMBL assays name that record.
- Starts from: cell-line.md's ChEMBL findings, response layers, command tree, case map and measured budget, with the cell line rulings; the j14318 captures and 0256's witnesses; the existing ChEMBL adapter in `crates/biomcp-adapters/src/chembl/` and its CC BY-SA 3.0 source registration; 0258's dispatch registration; 0257's types.
- Rules: the drug slice's ChEMBL behavior stays unchanged. The assay count is never relabeled as activities, experiments or drug sensitivity, and no activity, metric or treatment interpretation is computed. No assay listing or activity download. ChEMBL status metadata is a separate cacheable provenance read, and its failure labels retrieval time and adds a problem. `all` never sends the assay request.
- Proof: outside-in cases first, watched to fail. Cases reuse the j14318 and 0256 captures where the request matches exactly, and one bounded keyless pm job captures once, with zero retries, any exact request a case needs that those archives lack. C15 asserts MOLM-13's CHEMBL3706573 with EFO and CLO absent and taxon 9606; C17 asserts K-562's CHEMBL3308378 with name K562, EFO_0002067 and CLO_0007059 and no name join; CVCL_9999 asserts a successful empty with total zero, distinct from failure; C16 asserts 1,408 assays for CHEMBL3706573 with the first row's cell ID checked; 0256's zero-total or second witness asserts its outcome. A synthetic record with a different `cellosaurus_id` and a synthetic assay row with a different cell ID assert the drop, the problem and no cache admission. Recorded 429, 500, timeout, broken JSON and HTTP 200 HTML replies assert a failed outcome with source and terms. A disabled `chembl` source asserts the skip before cache and network. The job measures the section and the command in CLI and MCP output, starting from the design ceilings of 373 bytes for ChEMBL metadata and 63 for the assay count payload, and each case gets a ceiling at measured size plus 15 percent, recorded in cell-line.md. No licensed content enters the repo. Then make landing-gate runs through pm job.
- Defers: nothing.
- Changes: the ChEMBL adapter, the cell line section and command code, cases and `cell-line.md`.

- PM ruling 2026-10-26 (m7318): `cell-line assay-count` help caps are measured size plus 15 percent from j15217: 478 bytes CLI, 477 MCP and 531 Markdown. Global caps stay unchanged.
