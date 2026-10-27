# Variant filter recordings

The actual CLI emitted each request URL, source, method and recording filename into a fresh empty directory. These identities remain unchanged. Responses are synthetic identity-only projections of the actual source candidates, with native order and totals preserved. They contain only genomic IDs, rsIDs, gene symbols and numeric ClinVar IDs. Original responses and direct queries remain outside Git because component redistribution rights are not established.

On 2026-10-24 each implemented filter received one direct MyVariant query and one CLI capture. The direct query requested its source field and checked the returned literal facts. The CLI requested the ordinary compact identity projection and returned exactly the same ordered candidate IDs and total. The evidence store retains both requests, original bodies, capture dates and scripts outside Git.

The `review-status` query returned three candidates whose source records each contain `no assertion criteria provided`. The `condition` query returned one candidate whose source condition is `Carcinoma of colon (CRC)`. The `has` query returned three candidates with a ClinVar variant identifier. Source matching and region overlap determine the candidate list; BioMCP does not assign clinical meaning to it.

The separate significance request returned a record without the requested literal label and stopped before CLI capture. The missing-component request returned a valid zero and stopped before CLI capture. Both remain undecided. Neither a valid zero nor an unexpected source match is a positive filter proof. Neither request was retried with a different query.

Case ceilings measure each full replay envelope with its required component terms label and allow fifteen percent growth. The updated protein-selector envelopes also include their dbNSFP terms; their ceilings remain below the corresponding successful historical whole-card replies. These measurements use retained recordings and make no new provider calls.

The search projection now requests `chrom`, `hg19` and `hg38`. The actual CLI recorder generated the updated request identities through a local TLS peer using the unchanged retained responses. These bodies lack assembly metadata. Rows keep their native identifiers, report unknown build and omit unsafe disambiguation actions. The external proof retains the original source date, response hash and explicit reuse label; this recording made no provider calls.
