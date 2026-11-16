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

- 2026-11-15: Changed. `Cargo.toml`, `Cargo.lock` and `deny.toml` pin BioData d979c98, BioData origin/main, which `git merge-base --is-ancestor` shows is on main (it is main's tip and 0025's landed commit). BioData commits since 8bcd245 and their effect on BioMCP: 4936f05, 37333d1, eb854e2, 03c9701, 7623077, cc9c362, 8a33405, 0d0d86d, a46d953, c836f8e, 84e80ad and 1580d0a touch only BioData's `sdlc/` (ideas, tickets, decisions, pm.json), so none. f463170 (0017) adds a cargo-shear section to BioData's crate manifest and a BioData gate script: none. 98acab1 (0019) moves plain records onto one object-only reader, so their human-readable JSON readers refuse a positional array; schemas and serialized bytes are unchanged and BioMCP reads no BioData record from an array: none. afc3df0 (0024) makes `ProteinChange::Same` hold `Option<ProteinRange>`, adds `ProteinRangeError` and parses protein range identities such as `p.Ala2_Gly3=`; BioMCP never names `ProteinChange::Same` or the removed refusal code, and HGVS change values have no schema, so it compiles unchanged and a range identity now parses where it was refused. d979c98 (0025) is the recall-number widening BioMCP already pinned as 7b138af (same `diagnostic/id.rs`). In pm job j41503 the build, `make schema-generate`, `make docs-generate` and `cargo fmt` changed no file, and `make check-cheap` and `make schema-release-check` passed, so no reply schema changed and no schema review or code adaptation was needed.
