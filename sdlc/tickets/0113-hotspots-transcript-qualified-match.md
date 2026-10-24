# 0113 — Match a hotspot row only on the same transcript and residue

Status: OPEN.

Milestone: 2

Depends on: 0105

## Outcome

A Cancer Hotspots row is marked as matching the variant only when the source gene, the source transcript, the reference residue, the position and the alternate amino acid all equal the variant's transcript consequence on that transcript. Every other row stays a candidate with its residue-level counts, as landed. A match never claims allele-level recurrence or clinical meaning.

- Why: the landed code marks every nonempty hotspot result as having no join ID, so no input can prove an exact match. A clinical reviewer requires the source gene, a matching transcript and the reference residue, position and alternate before a row counts as matching. The shared `TranscriptConsequence` type from 0105 carries the variant side of that comparison.
- Starts from: the hotspots adapter, the shared `TranscriptConsequence` type, and the landed hotspot cases.
- Proof: a case whose variant has a consequence on the hotspot row's own transcript asserts a match. Cases with a different transcript, a different alternate or no transcript consequence assert candidates only. Existing hotspot cases pass. Then make landing-gate runs through pm job. The live capture stays in 0110.
