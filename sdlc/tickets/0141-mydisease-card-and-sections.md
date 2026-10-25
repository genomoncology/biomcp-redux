# 0141 — Get the exact disease card, synonyms, cross-references and parents through MyDisease

Status: OPEN.

Milestone: 3

Depends on: 0139
Depends on: 0140

## Outcome

`get disease CURIE` with no `--source`, or with `--source mydisease`, returns the exact MyDisease card for a MONDO or DOID ID with the requested ID and returned ID side by side, the proven alias relation when they differ, the complete source definition when one exists, component terms and the available sections. `get disease CURIE synonyms`, `xrefs` and `hierarchy` return one-source pages with `--section-limit` and `--section-offset`. The hierarchy section shows parents only.

- Why: the design keeps MyDisease for exact annotation after the caller selects an identifier, and trial ticket 0137 resolves `--condition-id` through this resolver, so 0137 depends on this ticket.
- Starts from: disease.md's command table, explicit cross-reference mapping table, card, synonym, cross-reference and hierarchy rows of the case map, and the 0117 recheck amendment 3; the retained D05, D08 and D13 MyDisease records; the shared transport; 0138's types and 0139's registry.
- Rules: get never searches by name and never chooses a candidate. A returned ID that differs from the requested ID is accepted only when the returned record's own cross-reference proves the alias, as DOID:3910 returns MONDO:0005061; otherwise the record is dropped with a problem and never enters the cache. A request outside MONDO and DOID returns an unsupported usage error before I/O, with no implicit routing. A card without a definition never borrows one. Multiple cross-references to one namespace, such as D13's DOID:3012 and DOID:0111503, stay visible choices with their source predicate, and ICD editions keep their own namespace. The request projects only the declared component fields, never `fields=all`. The card makes no Open Targets, MyChem or ClinicalTrials.gov request. `consider` never appears as a parent.
- Proof: outside-in cases first, watched to fail. Before adapter code, one bounded keyless pm job captures positive exact gets for MONDO:0007254, MONDO:0005105, MONDO:0018875, MONDO:0800444 and DOID:3910, and known positives for section paging on each section. Cases D02-exact, D05-explicit-DOID-alias, D02-card, D08-card, D15-parent-no-definition, D03/D04-separate-concepts, D08-synonyms-first/terminal, D12-historical-scope, D15-synonym-types, D02-ICD-namespace, D05-EFO/DOID/NCIt/OncoTree, D08-crossrefs, D13-multiple-DOID, D25-conflict, D05-parents, D06-broad-parent, D07-parent and D15-consider-not-parent replay the recorded replies. Recorded 429, 500, timeout and broken-body cases assert a failed outcome with source and terms. Each case has a size ceiling from its measured output, starting from the design's allocations of 1,850 bytes for the card, 950 for synonyms, 1,300 for cross-references and 1,350 for hierarchy. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
