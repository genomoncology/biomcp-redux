# 0256 — Capture the cell line reverse get, absence and projection witnesses

Status: COMPLETE.
Landed: 28a02c46fd84b82e9adb2815a47bc130f7c00053

Milestone: 4

Depends on: none

## Outcome

One bounded keyless pm job captures the exact requests that the cell line tickets need and the 0196 investigation lacks, and `planning/slices/cell-line.md` records each one as proven, empty or failed. The witnesses are: the Cellosaurus search for the exact cross-reference `ChEMBL-Cells:CHEMBL3706573`, in the cross-reference query form the investigation proved for `DepMap:ACH-000362`; one exact cross-reference search that returns more than one record, chosen from a database the captured cards show is shared, for the ambiguous reverse get; one exact Cellosaurus card request for a well-formed accession that has no record; one exact card request for an accession that Cellosaurus lists as secondary on a captured card; the projected Cellosaurus card request that the get command will send, for CVCL_2119 and CVCL_0004, in the documented field projection form; the Cellosaurus search with the exact accession query syntax for CVCL_0005; and one ChEMBL assay request with `limit=1` for a ChEMBL cell ID whose native assay total is zero, if the captured cell records name one, or else the ChEMBL cell record for a second exact accession. Raw replies, headers, the ledger and the recipe go to the evidence store, and no product code changes.

- Why: The slice method requires each exact request a product ticket sends to have a known-positive capture first, and the investigation captured no ChEMBL-ID reverse search, no ambiguous reverse search, no absent or secondary card, no projected card and no accession-syntax search.
- Starts from: cell-line.md's direct source findings, source paging, command tree and reproducible evidence sections, with the cell line rulings in `decisions/2026-10-26-protein-pathway-study-rulings.md`, which win over the design text; the j14318, j14323 and j14325 archives on the evidence store; `make source-capture` from 0200.
- Rules: the recipe names each exact URL before the job runs, sends each row once in order with at least half a second between requests, a 25-second deadline and an 8 MiB body bound, and makes at most ten physical requests. No retry, redirect, altered query, fallback host or extra page. A 401, 403 or 429 stops that source. No credential, cookie, whole database or FTP download. PharmacoDB, DepMap downloads and HPA are not called.
- Proof: the job ledger records every attempt with its exact URL, status, headers, bytes, SHA-256, elapsed time, returned accessions, native count and paging state, and the archive path and hash appear in cell-line.md beside each witness's result, and through pm progress. Each archive member is verified independently before unique scratch is deleted. A reviewer checks that each witness names the capability it unlocks or defers. No raw source body and no licensed content enters the repo. Then make landing-gate runs through pm job.
- Defers: a failed or empty witness keeps its capability deferred. Without the ChEMBL-ID or ambiguous reverse search, `get cell-line SOURCE_ID` supports only the proven databases and names the rest as not yet supported. Without the absent card, absent versus failed rests on synthetic cases labeled synthetic. Without the secondary card, `get cell-line` accepts primary accessions only. Without the projected card, the get command sends the full card request the investigation captured and its ceiling returns to the PM. Without the accession-syntax search, a CVCL input to `search cell-line` names `get cell-line`.
- Changes: the capture recipe under `scripts/`, cell-line.md's evidence section, and the evidence store. No product code.
