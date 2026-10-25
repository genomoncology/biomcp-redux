# 0126 — Search drugs and get the exact card through MyChem

Status: COMPLETE.
Landed: 6a6fe8be4d7449b543dd2cbe353f96ec5ee897ad

Milestone: 3

Depends on: 0124

Depends on: 0125

## Outcome

`search drug TEXT` with no `--source`, or with `--source mychem`, sends one literal MyChem query and returns a bounded page of native candidate headers in source order, with the native total, the echoed query and source, and exact next commands. `--limit` and `--offset` page natively. `get drug ID --source mychem` returns the exact MyChem document card with its attributed cross-references, such as CHEMBL941 on document KTUFNOKKBVMGRW-UHFFFAOYSA-N. A name never picks a product, and no fallback rewrites the query.

- Why: drug.md names MyChem as the one default search source because all twelve generic and three brand queries returned candidates. The 0116 recheck marks MyChem's exact-ID get unproven and its default search page budget unmeasured.
- Starts from: drug.md sections 2 to 4, the MyChem rows of the coverage and identity tables, and the 0116 PM recheck; the shared transport and adapter shape in `architecture.md`; 0124's types and 0125's registry entries.
- Rules: the first hit is not a drug identity. The two imatinib documents stay separate candidates, and package and structure IDs stay distinct. A cross-reference is returned only from the selected document that carries it. Only the components 0125 declares enter the response, so the default search does not fetch the full record. `identity_level` comes from the MyChem endpoint or record type, never from a field's shape.
- Proof: outside-in cases first, watched to fail. One bounded keyless pm job captures, before any adapter code, a positive exact-ID get for a selected imatinib document and the axi-cel document CHEMBL3989989, and known positives for the default search and `--limit` and `--offset` on imatinib or trastuzumab. Cases S01 (D01, D03, D19, D21), S05 (a two-page walk with no gaps or duplicates and a stable query), G01 (exact returned ID or a failed identity, no name fallback, no authorization event or label text on the card) and E01 (the exact quoted nonexistent name returns an empty page for that source only) replay the recorded replies. The job measures the default search page in CLI and MCP output and records the measured page budget in drug.md, replacing the unmeasured allocation. Each case has a size ceiling from its measured output. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
- PM ruling 2026-10-25: the default MyChem card is a small identity card. Native cross-references and product links move to `get drug ID xrefs --source mychem`, paged with `--section-offset` and `--section-limit`, with every link kept and the section named in `next`. Nothing is dropped or cut silently. The card's ceiling is its measured size plus 15 percent, and the xrefs section gets its own case and schema.
