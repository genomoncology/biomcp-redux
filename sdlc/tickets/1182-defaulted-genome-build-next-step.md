# 1182 — defaulted-genome-build-next-step

Status: COMPLETE.
Landed: 15a5fe045a2113143cfed7063d364f8b3dd65831

Milestone: 7

## Outcome

When a single-base coordinate such as `7-140453136-A-T` is looked up without `--build` and found absent, the reply says that GRCh38 was assumed and offers the same call with `--build GRCh37` as a next step.

## Why

The 2026-11-24 adversarial review ran `biomcp get variant 7-140453136-A-T` (an hg19 position) and got a clean `absent`; with `--build GRCh37` it resolves. The reply echoes the build, but nothing says it was a default.

## Proof

- Proof: CLI and typed MCP replay cases: the absent reply for the hg19 coordinate carries the defaulted-build note and the `--build GRCh37` next step; with `--build GRCh38` given, it carries neither.

## Starts from

- Starts from: variant coordinate parsing and the get-variant next-step builder.

## Keeps

- Keeps: Found GRCh38 coordinates reply as today.

## Defers

- Defers: Guessing the build automatically.

## Changes

- Changes: variant get next steps and cases.
