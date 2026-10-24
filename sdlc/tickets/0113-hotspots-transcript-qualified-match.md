# 0113 — Match a hotspot row only on the same transcript and residue

Status: OPEN.

Milestone: 2

Depends on: 0105

## Outcome

A Cancer Hotspots row is marked as matching the variant only when the source gene, the source transcript, the reference residue, the position and the alternate amino acid all equal the variant's transcript consequence on that transcript. Every other row stays a candidate with its residue-level counts, as landed. A match never claims allele-level recurrence or clinical meaning.

- Why: the landed code marks every nonempty hotspot result as having no join ID, so no input can prove an exact match. A clinical reviewer requires the source gene, a matching transcript and the reference residue, position and alternate before a row counts as matching.
- First step, a clinical source-mapping review with no code: which transcript, with or without a version, Cancer Hotspots publishes for each row, and which variant source BioMCP already calls states a consequence on that same transcript. The review cites source documentation and the retained captures, and makes no provider call.
- If the review finds such a source: this ticket adds the shared `TranscriptConsequence` type to biodata as the plan section 3 defines it, with a versioned `Accession`, maps that source onto it, and builds the match. A hotspot transcript without a version, or a consequence the source does not tie to a transcript, never matches and stays a candidate. No transcript version is invented and no arrays are paired by position.
- If the review finds no such source: the ticket closes without code. Hotspot rows stay residue-level candidates, as the slice design first said, and 0110's Hotspots capture proves a positive candidate row instead of a match.
- Starts from: the hotspots adapter, plan section 3's `TranscriptConsequence` row, and the landed hotspot cases.
- Proof: the review is recorded through pm progress. If code follows, a case whose variant has a consequence on the hotspot row's own versioned transcript asserts a match, and cases with a different transcript, a versionless transcript, a different alternate or no transcript consequence assert candidates only. Existing hotspot cases pass. Then make landing-gate runs through pm job. The live capture stays in 0110.
