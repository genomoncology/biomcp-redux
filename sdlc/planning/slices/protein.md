# Protein slice proposal

This investigation starts from `85e6ba864c3f13ec50432a58670ec2af13e12857` and follows steps 1 to 4 of the [slice method](../slices.md). It proposes entity access to source records. It adds no sequence analysis, clinical classification, network inference or external bioinformatics execution.

## Evidence recipe

The bounded native investigation asks UniProt, InterPro, Complex Portal and STRING for the same human accession set: P00533 (EGFR), P15056 (BRAF), P38398 (BRCA1), P04637 (TP53), P13569 (CFTR), Q9UIF7 (MUTYH), P31749 (AKT1), P42771 and Q8N726 (the two CDKN2A protein products). Reused replies answer multiple questions; reuse is recorded rather than counted as another measurement. Each source receives one request per accession, at most 36 physical GETs, with no retries, redirects, fallback or additional pages. Each reply has a 30 second deadline and a 4 MiB admission bound. Requests are sequential with at least one second between them. A source failure consumes its slot and remains a failure.

UniProt exact record JSON proves accession, taxon, sequence, features, isoform descriptions and cross-references. InterPro uses its accession-scoped entry endpoint with page_size=25; a next pointer proves only that this first page is incomplete. Complex Portal uses its accession search with human taxon filter and number=25; search text alone never proves participant membership. STRING uses the public network endpoint with species=9606 and limit=5; returned stable protein identifiers and scores remain source statements, and a network association never becomes a physical interaction.

Record the exact URL, request count, status, headers, elapsed time, raw bytes, SHA-256, returned identifiers, native count and continuation. Preserve failures unchanged. No raw source bodies enter Git. Archive the evidence on Yellow and independently verify every archive member before deleting unique scratch. Bundle downloads are deferred unless the API evidence proves a need. No whole proteome, STRING network or InterProScan installation is part of this investigation.

The 0.9 executable remains unmeasured until a denied-network replay is verified. Source inspection and existing fixtures establish leads only. Proposed command budgets remain proposals until a measured projection over retained native replies establishes their size. The PM reviews the completed proposal on this branch before product tickets exist.
