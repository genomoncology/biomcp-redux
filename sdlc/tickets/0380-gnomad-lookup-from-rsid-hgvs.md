# 0380 — Look up gnomAD from an rsID or GRCh38 HGVS

Status: COMPLETE.
Landed: 9985825a8596fab0ab4e00cf1811fe97e1b80bfd

Milestone: 5

Depends on: 0364

## Outcome

gnomAD lookups work from an rsID and from a GRCh38 genomic HGVS, such as BRAF V600E's rsID, by resolving to gnomAD's own variant ID through a sourced join, instead of failing with `no_join_id`. A lookup that cannot be joined says which identifier it needs and offers the next step that gets it.

- Why: regression against 0.9.3 found by experiment 499's rerun of six 0.9.3 rare disease witnesses and one published case on release/1.0 at ff2e9b64 (Jarvis m10854, 2026-10-27); public identifiers only. 1.0 works only from a ClinGen allele ID; 0.9.3 works from an rsID. Ian's ruling that 1.0 ships every known fix.
- Starts from: 0364's sourced ClinGen allele join for variant population, which `get variant ... population` must reuse (PM ruling m10791 ask 19: this ticket covers only that reuse, the GRCh38 HGVS path and missing-identity recovery); the gnomAD adapter's join, the variant identity resolver and the MyVariant and Allele Registry identifiers.
- Rules: the join uses identifiers from source records only; when an rsID maps to several alleles, every allele is listed and BioMCP does not pick one.
- Proof: replay cases for BRAF V600E by rsID and by GRCh38 HGVS return gnomAD data; a multi-allelic rsID lists candidates; an unjoinable input names the needed identifier.
- Changes: the gnomAD join, variant resolution and cases.
- Defers: nothing.
