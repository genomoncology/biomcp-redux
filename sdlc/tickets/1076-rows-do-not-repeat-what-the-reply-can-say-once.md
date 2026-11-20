# 1076 — rows-do-not-repeat-what-the-reply-can-say-once

Status: OPEN.

Milestone: 7

## Outcome

A value that is the same on every row of a list is stated once for the list, not on each row, and a reply does not spend most of its bytes on context the caller did not ask for.

## Why

The caller is usually an agent that pays per token for everything in a reply and re-reads it on every later turn. Measured for ticket 1071 on 2026-11-19, on main 991b0ab93: each protein feature row carries an `attribution` of about 150 bytes that is the same on every row (50 rows: 21,834 bytes, of which choosing columns can remove only a fifth because the attribution is required); GTR diagnostic rows repeat the test ID on every row; and `get drug imatinib indications --source opentargets` spends 43,894 of its 66,900 bytes on regulatory context, not on its indication rows. Ticket 1072 added two per-row fields to article rows, `open_access_source` and `license_not_checked`, about 45 bytes a row, that are usually the same on every row of a page.

## Proof

- Proof: the developer first scans every recorded reply for keys whose value is identical on every row of a list of ten rows or more, and for non-row content over a quarter of a reply's bytes, and reports the list with bytes per reply. That list, ranked by bytes saved across the recorded replies, sets the scope.
- Proof: for each list changed, the reply holds the same facts and is smaller by the measured amount; a case shows the value stated once and rows without it. Each fails on the base commit.

## Starts from

- Starts from: `sdlc/planning/1071-fields-choose-columns-in-sections.md` and its measuring script; the rule against dropping a value from a reply, which a hoisted value satisfies only if it is still stated.
- PM decision, 2026-11-19: this starts with the scan and a short proposal to the PM, because moving a required per-row field is a breaking change to that reply's schema and each case must be worth it. A row that differs from the list's stated value still carries its own.

## Keeps

- Keeps: no fact leaves a reply. Attribution a source's licence requires is still shown.

## Defers

- Defers: shorter default rows for agents; decide after measuring agent runs with ticket 1071 in place.

## Changes

- Changes: reply types and schemas for the lists chosen, their cases and docs.
