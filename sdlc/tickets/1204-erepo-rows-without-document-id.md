# 1204 — erepo-rows-without-document-id

Status: OPEN.

Milestone: 7

## Outcome

`variant assertions <CAid> --source erepo` and `get variant <CAid> erepo` return every ClinGen Evidence Repository classification the source returns for that allele, including the ones whose record has no summary document ID. A row without a document simply has no document link.

## Why

About 31% of the repository's expert-panel classifications come back as zero rows today, so an agent reads a classified variant as unclassified.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp variant assertions CA16609653 --source erepo` returns `status: partial`, `total: 0` and one problem, `record value.unknown`. The source's own API (`/evrepo/api/classifications?carId=CA16609653`) returns one Phenylketonuria expert-panel classification with its criteria.
- Of 13,968 repository records in the paged summary, 4,384 have no `PCERDocID`; the adapter's raw decoder requires it. Eighteen of 18 such allele IDs tried live dropped their row. For `CA024693` the source holds two classifications and one is returned.
- The existing cases use TP53 records, which all carry a document ID.

## Proof

- Proof: an outside-in case on a recorded reply for CA16609653 returns one row with the panel, classification and criteria, and no `value.unknown` problem; a recorded two-row allele returns both rows.
- Proof: the Evidence Repository census in the reach census job counts rows returned over rows held for a release sample and records the before and after share.

## Starts from

- Starts from: `crates/biomcp-adapters/src/erepo/raw.rs` (the required document field), `crates/biomcp-adapters/src/erepo/` mapping, and the repository's classifications API.

## Keeps

- Keeps: rows that carry a document keep their link and shape; a row the decoder truly cannot read still raises a problem.

## Defers

- Defers: the 46 repository curations with no ClinGen allele ID (exon-level deletions and duplications, for example the one under ClinVar variation 217107), which need a lookup by ClinVar ID; file it when a case needs one.

## Changes

- Changes: `crates/biomcp-adapters/src/erepo/`, variant assertion cases and replies.
