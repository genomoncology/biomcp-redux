# 0981 — biomcp-pins-current-biodata-main

Status: OPEN.

Milestone: 7

## Outcome

BioMCP pins a BioData commit that is on BioData main and includes 0025's recall number forms, with every reply-schema change from BioData's other commits since 8bcd245 reviewed under the 1.0.0 rules.

## Why

0978 (landed 2026-11-15, BioMCP 2020c47d3) pins BioData 7b138af, a ticket-branch commit kept reachable only by the tag `biomcp-pin-7b138af` and pm's `refs/pm/tips/0025/…`, because BioData's `pm ticket land` rebuilds the commit (landed as d979c98) and BioData main was 15 commits past the old pin (variant statements, object inputs, protein range identity, the dependency gate, `diagnostic/gtr.rs` and `regulatory.rs` rewrites). The 0978 review asked that moving to main be its own ticket with its own gate and schema review.

## Proof

- Proof: `Cargo.toml` pins a commit that `git merge-base --is-ancestor` shows is on BioData main and contains d979c98; `make --keep-going landing-gate docs-check schema-release-check` passes; any schema change is a reviewed entry in `release/schema-reviews.json` or a 1.0.0-allowed change.

## Starts from

- Starts from: `Cargo.toml:40`, BioData main log from 8bcd245, `sdlc/decisions/2026-11-11-consumers-build-changes-and-pin-commits.md` in BioData.

## Keeps

- Keeps: 1.0.0 schema rules; if a BioData change would break a reply schema, stop and ask the PM.

## Defers

- Defers: nothing.

## Changes

- Changes: the BioData pin, regenerated schemas and docs, any adaptations BioData's changes need.

## Progress
