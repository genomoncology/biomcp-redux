# 0882 — label-section-pages-say-they-continue

Status: COMPLETE.
Landed: e9da92745079c4b2564934c3510c8a3e19ef10bf

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

## Progress

- Measured 2026-11-13 (openFDA, compact JSON-escaped bytes of each whole field): Indications and Usage is 23,683 for KEYTRUDA 9333c79b v229, 12,307 for OPDIVO f570b9c4 v68, 6,450 for ZESTRIL 838c2d78 v2, and 3,210 to 4,281 for Tagrisso, Gleevec, Xarelto and Humira. Every label measured has 7 to 10 sections over 6,144; the largest reach 209,576 (Keytruda clinical studies). openFDA's Keytruda field holds the highlights list and the full section, which is why it is twice the 10,999 characters the raw-tools run counted for the section alone.
- A larger fragment does not fit the budget: the label section allocation is 8,192 bytes (`LABEL_BYTES`), and growing it past 50 percent needs a PM ruling (`sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md:7`), while Keytruda's indications need about three times it. So the continuation is now unmistakable instead: `label.data.remaining` counts the fragments after this reply, is absent on the last one, and comes before `text` on the command line and, by its name, in MCP's alphabetical keys. Text stays verbatim. Root-cause note: `sdlc/records/2026-11-13-label-continuation-rca.md`.
- Live on cb204d5ca (jobs j39041, j39048): Keytruda indications offsets 0 to 3 give `remaining` 3, 2, 1, then none, the last fragment ending at 1.21; replies 6,749 to 7,842 bytes. Opdivo indications 3 fragments, Zestril 4 (one short native element a reply, the first ending at a full stop), Xarelto 1 with no `remaining`.
- Case `tests/cases/drug/label-continues.yaml` replays the j39048 Keytruda reply (indications projection, provenance with the original hash); `max_bytes` 9,014 is the measured 7,838 plus 15 percent.
- Gate j39060 passed on 0a09fc2be (`make --keep-going landing-gate docs-check schema-release-check`), including `drug/label-continues`; the schema change is one optional field in drug-label and drug-sections.
