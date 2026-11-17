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
