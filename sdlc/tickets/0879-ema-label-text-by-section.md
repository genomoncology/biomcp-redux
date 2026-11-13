# 0879 — ema-label-text-by-section

Status: OPEN.

Milestone: 7

## Outcome

An EMA product's label text can be read by section (therapeutic indications, 4.1, at least), so a US-versus-EU indications side-by-side is one call per side.

## Why

From bench 0041 reg-affairs smoke run on BioMCP b87cb3b3e (pm job j38923), `personas/reg-affairs/runs/2026-11-13/log.md` in biomed-agent-bench (until filed: `~/workspace/data/biomed-agent-bench/0041-persona-reg-affairs/` on the M5), log.md:192-210: the EMA record gives one flat 8,884-character `indication` string with no section heading or product-information text, while the US label is numbered 1.1 to 1.10.

## Proof

- Proof: first check, live, what EMA publishes in machine-readable form for product information (EPAR/SmPC) and record it here; then a routine case reading Keytruda's 4.1 text with its version and URL, or a recorded finding that EMA offers none.

## Starts from

- Starts from: the EMA adapter and its `regulatory` section.

## Keeps

- Keeps: the current `indication` field; attribution and version.

## Defers

- Defers: other SmPC sections beyond the first proved one.

## Changes

- Changes: `crates/biomcp-adapters/src/`, `crates/biomcp-query/src/drug/`, `tests/cases/drug/`
