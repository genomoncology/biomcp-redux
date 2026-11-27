# Variant shorthand replays

MyVariant public query replies checked 2026-11-17. Fixtures retain the native candidate count and order, identifiers, source coordinate fields and one source-paired SnpEff transcript when available. They omit dbNSFP annotations and other licensed content. The V9998E provider error is synthetic.

The default BRAF get and its exact follow-up use synthetic GRCh38 projections with the allele coordinates from the public ClinGen Allele Registry record CA123643. The section attachment reuses the existing ClinVar Variation 13961 recording. Explicit GRCh37 gets reuse the hg19 search bodies under an explicit assembly selector.

The selector request metadata now uses the SnpEff query spelling. These shorthand, paging and error cases reuse their synthetic reply bodies to keep exercising the same downstream behavior. The separate default protein selector cases record the wider source results.
