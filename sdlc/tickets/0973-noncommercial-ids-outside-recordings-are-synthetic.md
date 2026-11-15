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

- Starts from: 0920's review list (plus the "replaced" notes 0920 left in tests/replies/orcid/README.md and the two WHO IVD READMEs, which still name the real IDs; say "a real ID" instead): tests/native/orcid/*, tests/native/author*/*, crates/biomcp-types/tests/author_contracts.rs, crates/biomcp/src/config/credential_reads_tests.rs, crates/biomcp-types/tests/drug_contracts.rs:397-404, crates/biomcp-types/tests/response.rs:350, scripts/orcid_native.py, scripts/author_capture_plan.json, sdlc/planning/slices/author.md.

## Keeps

- Keeps: what each test proves; the ORCID health check.

## Defers

- Defers: rewriting git history (Ian's call).

## Changes

- Changes: those test, script and planning files.

## Progress

- Contract and unit tests now use invented IDs: ORCID 0000-0000-0000-0925 (as 0920's replies do), 0000-0000-0000-0926 as its bad checksum, 0000-0000-0000-909X for the X check digit and 0000-0000-0000-9014/9022 for legacy client IDs (all outside ORCID's issued blocks); Semantic Scholar author 123; DDInter9001/9002 and DDInter9099; ClinPGx PA900000001/PA900000002 in place of real annotation accessions. Files: `author_contracts.rs`, `credential_reads_tests.rs`, `drug_contracts.rs`, `response.rs`, `pgx_contracts.rs`, `mapped_refusals/stored.rs`, `mapped_refusals/documents.rs`, `runner/enum_case.rs`. ORCID's published fictitious test iD 0000-0002-1825-0097 stays.
- `tests/native/` holds live checks that must reach real records, so they keep their real IDs, and each such file now carries a one-line `live_check` saying why. The two ORCID owners are renamed `orcid-380-groups.yaml` and `orcid-2-groups.yaml`; `scripts/orcid_native.py` reads their iDs from those files instead of naming them. `scripts/author_capture_plan.json` (a live capture plan) carries a `live_inputs` reason. The ORCID health probe in `orcid/decl.rs` keeps its iD with a comment.
- `sdlc/planning/slices/author.md` names no researcher and no real ORCID or Semantic Scholar author ID; records are named by their PMID or ORCID group count. `sdlc/planning/slices/drug.md` no longer names real DDInter IDs. The ORCID, DDInter and two WHO IVD READMEs say "a real ID" instead of naming the IDs they replaced.
- Proof: `noncommercial_replies::ids` in the `ratchets` test scans `crates/**/*.rs`, `tests/` (except `tests/replies`), `scripts` and `sdlc/planning` for checksum-valid ORCID iDs outside the invented 0000-0000 block, DDInter IDs below 9000 and Semantic Scholar author IDs of 4-17 digits. Only two things excuse one: the ORCID health probe iD in `orcid/decl.rs` (a fixed allowlist of that file and that iD), and a `live_check` or `live_inputs` field naming a "real record" in a JSON file under `tests/native` or `scripts`. Code comments never excuse one (review fix: a `//` comment saying "real record" used to exempt its whole file, and `mapped_refusals/documents.rs` already had one). A Python copy of the rule found all 23 offending files on main and none on this branch.
- `tests/cases/pgx/drug-id-refusal-shows-the-form.yaml` now refuses the invented PA900000003 instead of a real ClinPGx accession.
- Left as is: real ClinPGx annotation and WHO IVD product IDs in `sdlc/planning/slices/pharmacogenomics.md` and `diagnostic.md` (capture records, outside this ticket's list); small Semantic Scholar placeholders such as 123 and 901.
- Gate `make --keep-going landing-gate docs-check schema-release-check` passed in pm job j41372 at 343f7a734. The earlier check job j41366 ran the touched contract, refusal and ratchet tests (all passed) and supplied the `cargo fmt` changes; its two failures were wrongly written test filters in that job command, not test failures.
