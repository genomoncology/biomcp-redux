# Variant slice rulings

Date: 2026-10-24

Decided by the PM on review of the variant design at 62d54717 (`planning/slices/variant.md`), within `2026-10-24-no-text-heuristics.md`, `2026-10-24-biodata-builds-its-own-parts.md`, `2026-10-24-keyed-sources-run-only-with-the-user-s-key.md`, `2026-10-24-article-keeps-every-source-one-option-away.md` and the slice rules learned from 0.9.

The design's recommendations on its five questions are accepted, with these choices settled:

1. **Get takes identifiers by their published form.** `get variant INPUT` accepts an rsID, a ClinGen CAid, a ClinVar Variation ID, or reference-qualified HGVS such as `NM_004333.6:c.1799T>A`. BioMCP tells these apart by each identifier's published grammar, as `get article` tells a PMID from a DOI. It never reads meaning from free text. HGVS goes unchanged to the ClinGen Allele Registry by default, and `--resolver mutalyzer` or `--resolver variantvalidator` is one option away. A chromosome HGVS form needs `--build`. Gene and protein shorthand such as `BRAF V600E` is search input: `get variant` refuses it with a usage error that names `search variant --gene BRAF --protein V600E`.
2. **Every candidate comes back.** An rsID with several alternate alleles, a protein search or a resolver reply with several records returns all candidates with their selection steps. BioMCP never takes the first one.
3. **The card is compact.** The default card holds identity and MyVariant's compact source-attributed facts. Every other source is a named section that calls only its own source.
4. **Published spellings are kept as given.** When a paper's spelling fails the resolver's reference check, the output shows the paper's input and the resolver's rejection side by side. BioMCP never repairs notation.
5. **Cohort recurrence needs a named cohort.** cBioPortal recurrence takes an explicit study, profile and sample set and shows the denominator and the study's terms. There is no default cohort.
6. **Resolver services stay; structure and predictions wait.** The ClinGen Allele Registry, Mutalyzer and VariantValidator are explicit resolver services. `variant structure`, variant-specific UniProt and InterPro, and AlphaGenome are deferred. `variant structure` returns a retirement message that names `get gene <SYMBOL> protein`.
7. **Advanced filters need proof.** Each 0.9 MyVariant filter, such as `--significance`, `--max-frequency` or `--min-cadd`, is ported only with a known-positive case against MyVariant's own query on its capture date. Until then it stays undecided in the ledger. CGI is deferred.
8. **Owners.** `search gwas` belongs to the GWAS slice; the variant `gwas` section stays here. `variant trials` and `variant articles` belong to the trial and article slices. `variant cspec` is deferred until a harvested question needs it.
9. **One spelling for each option.** `predict` and `predictions` become one `predict` section. `population-details` becomes `variant population <ID> --detail`. Changed 0.9 spellings return retirement messages that name the 1.0 form.
