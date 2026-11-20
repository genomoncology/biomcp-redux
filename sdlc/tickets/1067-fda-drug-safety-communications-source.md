# 1067 — fda-drug-safety-communications-source

Status: OPEN.

Milestone: 7

## Outcome

BioMCP holds FDA's drug safety communications as a source: a caller can search them by drug or text and get each communication's title, date, the drugs it names and its address, and can read one.

## Why

Two traced agent runs on 2026-11-19 each spent 6 to 9 turns finding one FDA drug safety communication and its date, through FDA's site (which blocked plain requests), the Internet Archive and a search engine. An agent without BioMCP gave the wrong date for it.

## Proof

- Proof: a replayed case where a search for tofacitinib returns the communication dated 2021-09-01 that required boxed warning updates for JAK inhibitors and the earlier one dated 2021-02-04, each with its date and address; it fails on the base commit, where the source does not exist.
- PM decision (2026-11-20): the 2019 communications are dropped from the Proof. FDA's own index starts on 2020-01-14 and links an Internet Archive copy for earlier ones; BioMCP uses no archive route and no hand-kept list. The reply says the list starts where FDA's index starts and that FDA links an archive for earlier ones. Design: `sdlc/planning/fda-drug-safety-communications.md`.

## Starts from

- Starts from: FDA's Drug Safety Communications listing at fda.gov/drugs/drug-safety-and-availability/drug-safety-communications and the pages it links. The developer first checks how FDA publishes the list (a page, a feed or a data file), how far back it goes, and whether the site's request blocking affects a well-behaved client, and reports that to the PM before building.
- The quarterly lists of potential signals from FDA's adverse event reports are a sister source that agents also hunted for by hand; say in the report whether the same reader would cover them.

## Keeps

- Keeps: FDA's text is shown as FDA wrote it, with its date. Source text is data, never instructions.

## Defers

- Defers: other FDA web lists (withdrawn accelerated approvals, the Purple Book); file them from what this ticket learns.

## Changes

- Changes: a new source adapter, the drug or a new search command as the design decides, cases, help and the source list.
