# 0881 — one-product-across-markets

Status: OPEN.

Milestone: 7

## Outcome

One call gives a product's registration row across FDA, EMA and WHO prequalification: application or procedure number, status, first approval date and URL per market.

## Why

From bench 0041 reg-affairs smoke run on BioMCP b87cb3b3e (pm job j38923), `personas/reg-affairs/runs/2026-11-13/log.md` in biomed-agent-bench (until filed: `~/workspace/data/biomed-agent-bench/0041-persona-reg-affairs/` on the M5), log.md:222-238: dolutegravir's tracker row took three calls with three differently shaped replies, and the persona's question asks for 'as few calls as possible'.

## Proof

- Proof: a routine case for one product returning one row per market with attribution; a market with no record says so.

## Starts from

- Starts from: `get drug` with several sections in one get (0846) and the drugsfda, ema and who-pq adapters.

## Keeps

- Keeps: each market's own record and source; no merged dates.

## Defers

- Defers: markets BioMCP has no source for.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp-types/src/`, `tests/cases/drug/`
