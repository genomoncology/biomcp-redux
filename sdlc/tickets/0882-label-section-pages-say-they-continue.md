# 0882 — label-section-pages-say-they-continue

Status: OPEN.

Milestone: 7

## Outcome

An agent that reads a drug label section gets the whole section or cannot mistake its first page for the whole: Keytruda's Indications and Usage (subsections 1.1 to 1.21) is not reported as ending at 1.10.

## Why

From the bench 0041 reg-affairs run on BioMCP b87cb3b3e (`personas/reg-affairs/runs/2026-11-13/log.md:192-201` in biomed-agent-bench): `get drug KEYTRUDA label --label-section indications_and_usage` returned fragment 0 of 4 (6,128 characters; the reply has `total 4` and a `next` page step, checked on the same build on 2026-11-13), and the agent reported "subsections 1.1 … 1.10 (10 numbered indication groups)" as the section. The raw-tools agent read the same section whole from DailyMed: 10,999 characters, through 1.21 (`runs/2026-11-13-raw/log.md:62-63`). The same misreading happened in pv-scientist's 0040 run (`sdlc/records/2026-11-12-persona-rerun-failures.md`, regression (a)). It repeats across personas, so this ticket starts with a short root-cause note (`skills/rca.md`).

## Proof

- Proof: first measure label section sizes for a few large labels (Keytruda, Opdivo, a small label) and record them; then a routine case where reading Keytruda's indications section either returns all of it or makes the continuation unmistakable in the text an agent reads first, and the RCA note names which and why.

## Starts from

- Starts from: `crates/biomcp-adapters/src/openfda_label/page.rs` (6,144-byte fragments), the label section reply, and the section-limit and reply-size budgets.

## Keeps

- Keeps: label text verbatim; reply size limits measured, not guessed; paging for sections that truly need it.

## Defers

- Defers: other paged text sections unless the same fix covers them for free.

## Changes

- Changes: `crates/biomcp-adapters/src/`, `crates/biomcp-query/src/drug/`, `crates/biomcp-types/src/`, `tests/cases/drug/`, `sdlc/records/`
