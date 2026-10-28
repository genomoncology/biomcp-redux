# 0378 — Show transcript, c. and p. notation in variant search and the variant card

Status: COMPLETE.
Landed: 6a73afa171c89f45478008acd05bcd76be074fdf

Milestone: 5

## Outcome

Variant search items and the variant card show the source's transcript identifiers with their c. and p. HGVS notation, and say which transcript matched when a search used protein or coding notation. A published protein notation such as HSD17B4 His540 can be matched to its record, and a numbering difference such as His540 against His515 is visible as two transcripts, not hidden.

- Why: regression against 0.9.3 found by experiment 499's rerun of six 0.9.3 rare disease witnesses and one published case on release/1.0 at ff2e9b64 (Jarvis m10854, 2026-10-27); public identifiers only. 0.9.3 said which transcript matched; 1.0 shows no transcript, c. or p. notation. Ian's ruling that 1.0 ships every known fix.
- Starts from: the variant search and card types, the MyVariant and ClinVar transcript fields, and 0.9.3's transcript match output in the parity inventory.
- Rules: notation comes from the source records only; BioMCP does not compute or translate HGVS between transcripts. MANE Select is labelled only when the source labels it.
- Proof: replay cases for HSD17B4 His540 and His515 show both transcripts and which matched; a card shows c. and p. notation per transcript; the schema baseline is regenerated.
- Changes: variant types, adapters, renderers, cases and the baseline.
- Defers: nothing.
