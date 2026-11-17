# 1041 — label-choice-prefers-the-application-holder

Status: OPEN.

Milestone: 7

## Outcome

When several openFDA labels carry one drug name, `get drug NAME label` picks the application holder's own label over repackagers' copies. For ELIQUIS it picks Bristol-Myers Squibb's SPL set e9481622-7cc6-418a-acb6-c5450daae9b0, not a RemedyRepack or Aphena copy, and the reply still lists the others as alternatives.

## Why

The pt-committee pair run on 2026-11-16 (bench ticket 0059) read ELIQUIS from SPL 7be1f4c1, a RemedyRepack copy. The bench recheck on 2026-11-17 (bench commit 82f1607) found nine ELIQUIS labels in openFDA, eight of them from repackagers. The current BMS label is e9481622 version 39, effective 2026-08-17, and it covers both NDA202155 and NDA220073. The ranking in `crates/biomcp-adapters/src/openfda_label/map.rs:198-220` counts any label with an NDA or BLA number and a brand name as `LabelHolder::Brand`, which includes repackagers. Its tie-breaks are numbered indications, then the newest revision, and the RemedyRepack copy's revision (2026-09-11) is newer than BMS's. A repackager's copy can lag the holder's text, and two pt-committee facts turned on which label was read.

## Proof

- Proof: a replayed case for `get drug ELIQUIS label` expects set id e9481622 and lists the repackager labels as alternatives. It fails on the base commit.
- Proof: a second drug with repackager copies (found live) picks its holder's label.

## Starts from

- Starts from: `LabelChoice` in `openfda_label/map.rs`. The developer reads openFDA's label fields (for example `openfda.manufacturer_name` and the SPL's labeler) and Drugs@FDA's sponsor, and chooses the simplest signal that tells holder from repackager. The choice and its reason go in Progress.

## Keeps

- Keeps: the label's sections, its text and the alternatives list.

## Defers

- Defers: nothing.

## Changes

- Changes: the label ranking, a case and its recording, and one release-notes line.

## Progress

- Source check (live openFDA label API, 2026-11-17): the signal is `openfda.is_original_packager`. openFDA sets it to `[true]` on a label filed by the product's original packager and leaves it out of a repackager's copy, which instead carries `openfda.original_packager_product_ndc`. Across all of openFDA, 65,010 labels carry the flag and 21,995 carry an original-packager NDC; no label carries both, and none carries `false`. Per drug: ELIQUIS has 9 labels, only BMS's e9481622 (manufacturer "E.R. Squibb & Sons, L.L.C.", NDA202155 and NDA220073, 2026-08-17) is flagged; the 8 copies (RemedyRepack 7be1f4c1 of 2026-09-11, Aphena 41a133ef of 2026-09-24, Cardinal, A-S Medication, Proficient Rx) are not. XARELTO: only Janssen's 10db92f9 of 7 is flagged. JARDIANCE: only Boehringer's faf3dd6a of 9. ENTRESTO: only Novartis's 000dc81d of 4. JANUVIA: only Merck's f85a48d0 (2024-11-14) of 9, while five A-S Medication copies are dated 2026-07-08. FARXIGA: only AstraZeneca's 72ad22ae of 3; a RemedyRepack copy is newer. KEYTRUDA, ZOLOFT and LIPITOR have holder labels only, all flagged. For generic names the flag also marks the ANDA maker's own label (atorvastatin: Zydus, Alembic, Radha flagged; Bryant Ranch, RemedyRepack, A-S Medication not).
- Why not the other signals: `openfda.manufacturer_name` is the labeler and needs a list of repackager names; Drugs@FDA's sponsor for NDA202155 is "BRISTOL MYERS SQUIBB" while the label's labeler is "E.R. Squibb & Sons", so matching them needs fuzzy name rules and a second source call. The flag is on the label record already read, so the choice costs no extra request.
- Change: the label ranking puts the original packager's label before repackaged copies within each holder tier (after brand-name NDA/BLA, other NDA/BLA, other), ahead of numbered indications and revision date. The reason line says whether the chosen label is from its original packager and how many of the others are repackaged copies.
