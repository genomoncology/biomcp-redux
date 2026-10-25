# 0129 — Search openFDA labels by brand name and read named label sections

Status: COMPLETE.
Landed: 95dd306b78b7cd15267f944a94a26e4c48493e6c

Milestone: 3

Depends on: 0124
Depends on: 0126
Depends on: 0125

## Outcome

`search drug Gleevec --source openfda-label` sends openFDA's brand-name field only and returns label candidates with SPL set ID and version. `get drug SETID --source openfda-label` returns the label identity card, and `get drug SETID label --source openfda-label --label-section boxed_warning` returns that native field's original text with SPL set, version, effective time and retrieval metadata. Long fields continue with the shipped `--section-offset` and `--section-limit`, and the continuation rebuilds every original character in order.

- Why: the bank's label questions (D04, D10 to D20) need named label fields, and the card carries no label text. The 0116 recheck limits search to the brand-name field until a generic-name positive case exists, and reuses the shipped continuation spelling.
- Starts from: drug.md sections 2 to 4, the label field size table and cases L01 and L02, with the 0116 recheck amendments 2 and 6; the six captured SPL set queries; 0124 and 0125.
- Rules: `--label-section` takes `boxed_warning`, `dosage_and_administration`, `drug_interactions`, `indications_and_usage` or `warnings_and_cautions`. A field the record lacks is unavailable on that record, never an empty clinical answer. No clinical sentence selection and no summary. Captured revisions stay explicit, so Xeloda's 2026-02-05 SPL never stands in for the October 2025 PDF. Tagrisso's brand query keeps its actual NOT_FOUND. Search on generic names waits for a captured generic positive. `identity_level` comes from the endpoint or record type, never from a field's shape.
- Proof: outside-in cases first. One bounded keyless pm job captures brand-name known positives for Gleevec, Herceptin and Enhertu, exact SPL set gets for the six captured sets, and a long dosing or indication field. Cases S04 for `openfda-label` (D04 keeps Herceptin and Enhertu separate), L01 (each named section over D10 to D20) and L02 (D09, D11, D19 continuation that rebuilds the original text) replay the recorded replies with size ceilings. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
- PM ruling 2026-10-25: DrugHeader gains an optional native version beside the SPL set ID, omitted when absent. The header carries no label text and no event. Label paging waits until this ticket compares the shipped continuation helper.
