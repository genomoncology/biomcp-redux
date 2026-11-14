# ClinVar gene count recordings

The ClinVar and MyGene exchanges were recorded live through the transport in j40643 on 2026-11-14 with `get gene BRCA1 clinvar --record`, against ClinVar Build261108-1750.3 (last updated 2026/11/09 21:36). They are the sixteen E-utilities searches, the einfo reply and the one esummary of the six BRCA1 records whose review status search listed them twice, kept whole.

The HGNC exchange is a slice of the complete-set recording in ../../trial/variant-codon/hgnc: the published header and the BRCA1 (HGNC:1100) row, unchanged, with the original exchange metadata. Every other row is dropped, so this slice is synthetic.

Before building, the same counts were checked against all 16,172 BRCA1 esummary records fetched from E-utilities: each classification and review bucket matches the per-record germline fields exactly.
