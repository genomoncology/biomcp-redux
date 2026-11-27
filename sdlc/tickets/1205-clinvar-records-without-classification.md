# 1205 — clinvar-records-without-classification

Status: OPEN.

Milestone: 7

## Outcome

`get variant clinvar:<N> clinvar` and `variant assertions clinvar:<N> --source clinvar` return the submissions of a ClinVar record that has no aggregate classification, and a record that exists only inside a haplotype or genotype returns an honest card that names the records including it. Neither reads as empty or as a source failure.

## Why

ClinVar holds 249,528 records (5.3% of 4,714,028) whose review status is "no classification provided"; each has real submissions, and all of them read as `complete` and `empty` today.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case 1: `biomcp get variant clinvar:4517766 clinvar` returns `status: complete` with the `clinvar` section `empty`, and `biomcp variant assertions clinvar:4517766 --source clinvar` returns `absent`, total 0. ClinVar's esummary for 4517766 lists one SCV and one RCV. Eleven of 11 such records tried behaved this way.
- Cause: the mapper reads rows only from the three classification groups (germline, somatic clinical impact, oncogenicity) and skips a record with none, with no problem.
- Failing case 2: `biomcp get variant clinvar:242709 clinvar` returns `failed`, source reason `bad-reply`. ClinVar answers normally; the record is an included-only variation (715 such records by ClinVar's own count), and the raw decoder requires a classified record.
- No fixture holds a record of either kind; the cases lean on variation 13961.

## Proof

- Proof: outside-in cases on recorded replies: 4517766 returns its SCV and RCV rows with the classification each submitter gave or "no classification provided"; 242709 returns a card that says it is included-only and names the including records, with status not `failed`.
- Proof: the ClinVar census in the reach census job counts, by review status and variation type, the share of sampled records whose rows are returned, before and after.

## Starts from

- Starts from: `crates/biomcp-adapters/src/clinvar/map.rs`, `crates/biomcp-adapters/src/clinvar/raw.rs`, `crates/biomcp-adapters/src/clinvar/subject.rs`.

## Keeps

- Keeps: classified records and their row counts are unchanged; unmapped classification terms (drug response, risk factor, other) keep their source text.

## Defers

- Defers: the subject of genotype and diplotype records (596 records return rows with no genotype subject), which follows ticket 0475's haplotype work; file it when a case needs one.

## Changes

- Changes: `crates/biomcp-adapters/src/clinvar/`, variant cases and replies.
