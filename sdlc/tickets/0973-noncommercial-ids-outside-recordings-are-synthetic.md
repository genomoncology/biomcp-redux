# 0973 — noncommercial-ids-outside-recordings-are-synthetic

Status: OPEN.

Milestone: 7

## Outcome

Outside `tests/replies`, test code, fixtures, planning files and scripts no longer pair real noncommercial-source IDs with test content: the contract and unit tests use invented IDs (as 0920 did in recordings), and any live-call test input that must name a real record says why in one line. The ORCID health check in `crates/biomcp-adapters/src/orcid/decl.rs:8` may keep its real probe iD, with a comment saying why.

## Why

0920 review (2026-11-15) found real ORCID iDs (0000-0002-4299-3018, 0000-0002-7433-2740), real Semantic Scholar author IDs (2060978, 9845886, 2118426986) and real DDInter IDs (DDInter1, DDInter2 paired with invented names) outside `tests/replies`, where 0920's ratchet doesn't look; one planning file names a researcher beside his iD.

## Proof

- Proof: a grep in the ratchet or a script over crates/*/tests, tests/native, scripts and sdlc/planning shows none of those real IDs remain except the health-check probe and named, justified live-call inputs.

## Starts from

- Starts from: 0920's review list: tests/native/orcid/*, tests/native/author*/*, crates/biomcp-types/tests/author_contracts.rs, crates/biomcp/src/config/credential_reads_tests.rs, crates/biomcp-types/tests/drug_contracts.rs:397-404, crates/biomcp-types/tests/response.rs:350, scripts/orcid_native.py, scripts/author_capture_plan.json, sdlc/planning/slices/author.md.

## Keeps

- Keeps: what each test proves; the ORCID health check.

## Defers

- Defers: rewriting git history (Ian's call).

## Changes

- Changes: those test, script and planning files.
