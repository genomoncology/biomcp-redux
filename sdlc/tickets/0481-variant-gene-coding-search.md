# 0481 — Search variants by gene and coding change

Status: OPEN.

Milestone: 5

## Outcome

`search variant --gene PTEN --coding c.333G>T` finds the ClinVar record for a gene plus coding change without a transcript, including intronic forms such as FLCN c.780-4C>G and MLH1 c.678-14_678-3del, and lists each match with its get step.

- Why: BioData's experiment 502 compared BioMCP 0.9.3 with an agent writing its own Python and curl on dated cancer questions, and the scripts won (m15051). A code triage against 1.0 at 8466b4700 found this defect still present with no ticket.
- Starts from: `search variant` options, the protein and coding change parsers in `crates/biomcp-types/src/query/`, the ClinVar adapter, and the refusal case `tests/cases/variant/identity.yaml:45`.
- Rules: send `G[GENE] AND "c.…"[VRNM]` to ClinVar E-utilities; the coding change goes through the existing HGVS coding grammar, so a malformed change is refused with a named error; never guess a transcript; more than one match lists every match, as the existing ambiguous protein refusal does; `get variant` keeps requiring a stable ID.
- Proof: a case over a recorded ClinVar reply finds PTEN c.333G>T as VariationID 3669092 and an intronic case finds its record, both failing on the base build; a malformed coding change is refused.
- Changes: the variant query type, the ClinVar adapter, help, cases and baseline.
- Defers: nothing.
