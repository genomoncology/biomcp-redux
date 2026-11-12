# 0856 — trial-search-widens-variant-spellings

Status: OPEN.

Milestone: 7

## Outcome

A trial search for a protein change finds the trials that write the same change another way (V600E, Val600Glu, p.V600E, and where the design review says so the broader codon form V600), says in the reply which spellings it searched, and searches the literal text only with `--exact-term`, as drug names do since 0830.

## Why

Bench 0037's blind persona rerun (2026-11-12) found this the largest silent undercount left after 0830: patient-navigator's `--variant V600E` found 1 of 3 matched BRAF/MEK plus immunotherapy trials where `V600` found all 3; "exon 19 deletion" also lost trials; life-sciences-ci's `--variant G12C` missed CodeBreaK 202. 0830 deferred condition and protein-change spellings to a separate ticket that was never written.

## Proof

- Proof: first measure, on live ClinicalTrials.gov, how many trials each spelling finds for BRAF V600E, KRAS G12C and EGFR exon 19 deletion, and which spellings trial records actually use; put the table here before choosing.
- Proof: a routine case with recorded replies shows `--variant V600E` finding the trials written as V600E, Val600Glu and (if chosen) V600, with the searched spellings listed, and `--exact-term` returning the literal count; continuation pages keep page 1's spellings.

## Starts from

- Starts from: trial search's `--variant`/`--criteria` paths in `crates/biomcp-query/src/trial/`, the variant normalizers BioMCP already has (`variant_normalize`, HGVS protein forms), and 0830's widening pattern (one disclosure notice, frozen names, never fewer than literal, totals not summed).
- PM decision: 2026-11-12. Design review first: whether widening a specific change (V600E) to its codon (V600) is right is a clinical-meaning question, since V600K patients match V600 trials too. Default to exact change in its spellings; codon widening only if the review shows trials mostly write the codon and the reply says so. Consult expert-assist if this stays unsettled.

## Keeps

- Keeps: literal search with `--exact-term`; no result presented as widened when it was not; native totals never summed per spelling; existing trial row fields and paging.

## Defers

- Defers: condition-name spellings (a separate ticket if the measurement shows a comparable gap); NCI trial search stays literal with its notice.

## Changes

- Changes: `crates/biomcp-query/src/trial/`, `tests/cases/trial/`
