# 0365 — Accept a GRCh37 genomic HGVS in variant normalize

Status: OPEN.

Milestone: 5

## Outcome

`variant normalize --source car --hgvs 'chr10:g.114758349C>T' --build GRCh37` sends the GRCh37 expression to the ClinGen Allele Registry and returns its allele, instead of `usage.arguments`. A build the source does not support returns an error that names the build.

- Why: the docs lead's second-pass product findings (2026-10-27), item G5. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: the variant normalize grammar, its build validation and the Allele Registry adapter.
- Rules: BioMCP does not lift over between builds itself; the source does. PM ruling m10791 ask 10: `chrN` is qualified from the existing accession table for the requested build with the genomic suffix unchanged; a primary accession that conflicts with explicit `--build` is refused, naming `--build` and the two supported builds; unknown and transcript accessions keep current behavior.
- Proof: a replay case for the GRCh37 expression returns the allele; an unsupported build names the build.
- Changes: grammar validation, the adapter request and cases.
- Defers: nothing.
