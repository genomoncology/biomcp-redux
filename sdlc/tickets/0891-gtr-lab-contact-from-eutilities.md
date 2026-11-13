# 0891 — gtr-lab-contact-from-eutilities

Status: OPEN.

Milestone: 7

## Outcome

A GTR test card (and, if the cost is measured and small, a search page) carries the lab's city and street address, director, and order or test link, so a counselor can tell a family where to order a test without opening NCBI's page.

## Why

0887 gave GTR rows their gene count, methods and lab state, postcode and country from NCBI's bulk files, but city, street, director and order link are not in those files (0887 Progress). In the rare-disease runs of 2026-11-13, the code-mode agent got all of them for 215 KMT2D tests from E-utilities `esummary db=gtr` in batches of 100 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13-raw/log.md:150-168`). BioMCP's agent had none (`runs/2026-11-13/log.md` Q7).

## Proof

- Proof: a recorded case where a KMT2D test card carries `OffererLocation` address, director and order or test URL from a recorded esummary reply; the fields are absent, not invented, when the source lists none; an E-utilities failure leaves the bulk-file fields intact and says so.

## Starts from

- Starts from: 0887 (`crates/biomcp-adapters/src/gtr/`), BioMCP's existing E-utilities client and its rate limits without an NCBI key.

## Keeps

- Keeps: the bulk files as the source of the test list and counts.

## Defers

- Defers: price, turnaround and CPT codes.

## Changes

- Changes: `crates/biomcp-adapters/src/gtr/`, `crates/biomcp-types/`, `tests/cases/diagnostic/`, `docs/`
