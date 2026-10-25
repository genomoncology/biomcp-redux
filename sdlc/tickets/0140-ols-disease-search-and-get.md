# 0140 — Search diseases through OLS and get an explicit OLS concept

Status: OPEN.

Milestone: 3

Depends on: 0139

## Outcome

`search disease TEXT` sends the positional text unchanged to OLS limited to MONDO by default and returns a bounded page of candidate headers in source order, with the native total, the echoed text, source and ontology, an obsolete flag where the source sends one, and an exact get action per row. `--ontology doid` and `--ontology ncit` select one other ontology. `--limit` and `--offset` page natively with every selector kept on the next action. `get disease CURIE --source ols --ontology mondo|doid|ncit` returns the exact OLS card with its definition and obsolete, `consider` and replacement facts, and `get disease CURIE synonyms|xrefs --source ols --ontology ...` returns the OLS scoped synonym and cross-reference pages with `--section-limit` and `--section-offset`.

- Why: the design settles OLS with MONDO as the default discovery route and OLS as the explicit ontology metadata source. The design measured only a three-ontology OLS query, so the MONDO-only default needs its own captured positive before any count or speed claim.
- Starts from: disease.md's settled product contract, command table, search rows of the case map and the D15 obsolete finding; the shared transport and adapter shape in `architecture.md`; 0138's types and 0139's registry.
- Rules: no local ranking, alias list, query rewriting or automatic identity choice. A rank never selects a concept, so D21 returns the source's candidates in source order. An obsolete concept keeps both `consider` values and a null replacement, and `consider` never becomes a parent or a replacement. The returned identity must equal the requested CURIE, or the record is dropped with a problem. The hierarchy section does not use OLS in this ticket.
- Children: the same job captures one OLS child-relation positive for a concept with known children, such as D18's MONDO:0019003. A positive capture is recorded in disease.md and through pm progress so a later ticket can add children to the hierarchy section. A failed or empty capture is recorded the same way, and the hierarchy section stays parents only.
- Proof: outside-in cases first, watched to fail. Before adapter code, one bounded keyless pm job captures the MONDO-only default positive first, then known positives for `--ontology doid` and `--ontology ncit`, a three-page `--limit` and `--offset` walk, exact OLS gets for MONDO:0007254, DOID:3910 and the obsolete MONDO:0007607, the OLS synonyms and xrefs pages for DOID:3910, and the child-relation request. Cases D02-mondo, D01-name, D02-id-text, D05-doid, D08-ncit, D21-ambiguity, D01-first, D01-next, D01-terminal, D18-subtypes as an OLS search, D15-obsolete/consider and OLS witnesses for D05 synonym scopes and cross-references replay the recorded replies. Recorded 429, 500, timeout and broken-body cases assert a failed outcome with source and terms, never an empty page. Each case has a size ceiling from its measured output, starting from the design's 2,400-byte search allocation. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
