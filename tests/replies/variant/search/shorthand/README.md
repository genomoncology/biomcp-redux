# Variant shorthand replays

MyVariant public query replies checked 2026-11-17. Fixtures retain the native candidate count and order, identifiers, source coordinate fields and one source-paired SnpEff transcript when available. They omit dbNSFP annotations and other licensed content. The V9998E provider error is synthetic.

The unique BRAF follow-up uses a projection of the exact MyVariant hg19 allele read and the existing ClinVar Variation 13961 case recording to exercise normal section attachment.

The selector request metadata now uses the SnpEff query spelling. These shorthand, paging and error cases reuse their synthetic reply bodies to keep exercising the same downstream behavior. The separate default protein selector cases record the wider source results.
