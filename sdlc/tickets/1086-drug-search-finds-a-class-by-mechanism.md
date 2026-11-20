# 1086 — drug-search-finds-a-class-by-mechanism

Status: COMPLETE.
Landed: eb1273aefd4fd4f8056549efa499815fb372de9c

Milestone: 7

## Outcome

A caller can ask BioMCP for the drugs whose current FDA label describes a given mechanism of action, and is told which label field matched, so a drug class can be listed without a hand-written openFDA query.

## Why

In three traced agent runs on 2026-11-20 that had to list a drug class, every agent went straight to openFDA's label API with its own queries on the mechanism-of-action field, about ten turns each, and never asked BioMCP. Checked by hand on main 5db190f8a: `biomcp search drug "Janus kinase" --source openfda-label --limit 50` returns 44 rows by falling back to label text. It finds nine brand products and 20 generics, does not say which label field matched, and leaves out products whose mechanism section words the class differently (one label says "Janus Associated Kinases"). `search drug` has `--indication` for the indications field and nothing for mechanism of action or pharmacologic class.

## Proof

- Proof: a replayed case where a search on the mechanism-of-action field for a phrase returns each label whose field holds it, with the brand, application numbers and the matched sentence; it fails on the base commit, where the option does not exist.
- Proof: the reply says the match is on the label's wording only and that labels wording the mechanism differently are not returned.

## Starts from

- Starts from: `search drug --indication` and how it queries openFDA's `indications_and_usage`; openFDA's `mechanism_of_action` field and its `openfda.pharm_class_epc` and `pharm_class_moa` values. The developer first reports which of these openFDA fills for brand labels, and proposes the option or options to the PM before building.
- PM decision, 2026-11-20: BioMCP reports what labels say. It does not decide which drugs belong to a class and adds no synonyms of its own.

## Keeps

- Keeps: `--indication` and the plain query behave as now.

## Defers

- Defers: a way to leave generics (ANDA) out of label search rows, if the developer finds it does not fit here.

## Changes

- Changes: the drug search command, the openFDA label adapter, help text and cases.
