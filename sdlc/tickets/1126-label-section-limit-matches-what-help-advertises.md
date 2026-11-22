# 1126 — label-section-limit-matches-what-help-advertises

Status: COMPLETE.
Landed: e0307e8e1f27d84afcef88ef60908e14ce159850

Milestone: 7

## Outcome

A current-label read accepts every `--section-limit` value its help advertises, or the help states the limit the openFDA label read accepts. A read that fails says it failed: it never reports the source as OK while returning an error.

## Why

Found by the developer of tickets 1108 and 1091 on 2026-11-21: current-label help advertises `--section-limit` up to 200, but an openFDA label read with `--section-limit 51` fails while reporting the source as OK. Help that teaches a refused value costs an agent a turn, and a failure marked OK can be read as a complete answer.

## Proof

- Proof: a replayed openFDA label read with `--section-limit 51` either succeeds or is refused before any source request with the accepted range named; fails on the base commit.
- Proof: no reply carries an error and a source status of OK together for this path.

## Starts from

- Starts from: the label read's section-limit validation and the openFDA label adapter's page size; reproduce first with `biomcp get drug Ibrance label --source openfda-label --label-section dosage_and_administration --section-limit 51`.

## Keeps

- Keeps: Drugs@FDA's own section-limit rule ("above 50 reads no supplement letters").

## Defers

- Defers: nothing.

## Changes

- Changes: label read validation or paging, its help, and cases.
