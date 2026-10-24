# ClinGen command recordings

The ClinGen validity and dosage files were captured once through the transport in j9054 on 2026-10-24. The slices retain the four physical banner lines, original header and separator, and every published row for TP53, MLH1, MSH2, MSH6, PMS2 and EPCAM in source order. Dates, labels and report links retain their source spelling. The full captured files contained 3,714 validity rows and 1,719 dosage rows. Slice counts describe this snapshot only.

The HGNC fixtures reuse the existing constraint slices. Missing MLH1, MSH2, MSH6, PMS2 and EPCAM records derive from the official HGNC REST responses retained under hgnc-rest, captured through the transport in j9069. For each existing TSV column a REST scalar is copied verbatim, a list is joined with the published pipe separator in source order, and an absent field stays empty. No HGNC bundle was downloaded again. This derived TSV fixture is synthetic. The five-ID MyGene POST was captured in the same job. Existing TP53, MIR21 and BCR MyGene and TP53 gnomAD identities reuse the prior constraint recordings.

The all and constraint directories reuse these same ClinGen slices. ClinGen reports are public CC0 summaries.
