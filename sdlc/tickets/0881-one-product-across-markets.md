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

## Progress

- Measured live on 2026-11-13 (jobs j39204, j39210), dolutegravir: Drugs@FDA's name lookup picks NDA204790 (TIVICAY, sponsor VIIV HLTHCARE) among 29 matches; its earliest approved original submission is dated 2013-08-12 and its products are Prescription and Discontinued. The openFDA record has no page link; FDA's overview page `accessdata.fda.gov/scripts/cder/daf/index.cfm?event=overview.process&ApplNo=204790` answers 200. EMA (after `sync ema`) picks Tivicay EMEA/H/C/002753, status Authorised, holder ViiV Healthcare BV, `marketing_authorisation_date` 2014-01-16, with its EPAR URL. WHO PQ lists 52 rows naming dolutegravir (combinations and listings based on a US FDA approval included; 22 with a blank date cell); the earliest prequalified is HA634, ViiV Healthcare BV, 14 Oct 2014. WHO's product pages are `extranet.who.int/prequal/medicines/<reference, lower case, spaces as hyphens>`, read from WHO's own list links; `.../medicines/ha634` answers 200. Imatinib has FDA (NDA021335) and EMA (Glivec) records and no WHO PQ row.
- Design: `get drug NAME markets` is its own request (like the WHO reference get), so the reply echoes no source. The CLI and the typed `drug` tool route a get whose section is `markets` to it; `--source` with it is refused as a usage error. The reply `biomcp.drug-markets/1` has one row per market, always drugsfda, ema, who-pq: state (found, no-record, failed), record number, name, holder, the source's own status words, first approval as a date with the source's own role (`submission_status_date` of the earliest approved original, `marketing_authorisation_date`, `date_of_prequalification`), URL, match count and how the record was chosen. Each market uses its existing name lookup; WHO, which has none, takes the earliest dated row. Dates are never compared or merged. Each found row adds the call that reads its full record, and a search call when several records matched. Schemas only gain a section value, a request branch and two new schemas.
- Proof: `tests/cases/drug/markets.yaml` with replies recorded by `biomcp --record` in jobs j39235 (EMA, trimmed to the dolutegravir and imatinib medicines) and j39248 (Drugs@FDA, WHO PQ, unchanged); details in `tests/replies/drug/markets/README.md`. Dolutegravir gives NDA204790 (2013-08-12), EMEA/H/C/002753 (2014-01-16) and HA634 (2014-10-14), each with its URL; imatinib gives NDA021335 (2001-05-10) and EMEA/H/C/000406 (2001-11-07), and its WHO row is `no-record` with 0 matches. Live replies measure 3,198 and 2,711 bytes; ceilings 3,700 and 3,200. `--source` with `markets` is refused as a usage error.
- Gate j39288 failed only in the two cases where a source typed as a section (`get drug primaquine who-pq`) must point at `--source who-pq`: the section-repair step picked `markets`, since WHO PQ is one of its sources. That step now skips `markets`, as it skips `all`.
- Gate j39302 at 5ef2dca passed: `cargo fmt --check` and `make --keep-going landing-gate docs-check schema-release-check`.
- Review fixes: the reply keeps five next steps, so the sixth (WHO's list call for lamivudine, which matches 138 FDA, 4 EMA and 92 WHO records) was dropped. The markets reply now orders sync calls, then each multi-match market's list call, then record calls; at most two syncs and three list calls fit, so every market with several matches keeps its list call. A market whose source answered with no record now reports `matches: 0` in all three markets. The WHO row reuses the WHO sync step, which now looks only at WHO's own source. The case asserts `matches` and `choice` for every market and adds lamivudine, recorded in job j39400 (EMA re-trimmed to include lamivudine's medicines).
