# 0884 — label-pages-fill-their-budget

Status: OPEN.

Milestone: 7

## Outcome

A drug label section comes back in as few replies as its size allows: each reply fills its byte budget across openFDA's short pieces, and the budget is large enough that a typical indications section (Keytruda's included) is one call.

## Why

0882's measurements (its Progress, 2026-11-13): Keytruda's Indications and Usage is 23,683 bytes, Opdivo's 12,307, Zestril's 6,450, others 3-4 KB; a label section gets 8,192 bytes a reply in 6,144-byte fragments, and a reply returns one openFDA piece, so Zestril's indications take four calls although they would fit in one. The reg-affairs runs of 2026-11-13 showed the cost: BioMCP's agent read one fragment of Keytruda's indications and stopped, while the raw-tools agent read the whole section in one DailyMed call.

## Proof

- Proof: measure label section sizes across a spread of labels and record the share that fit in one reply at the chosen budget; routine cases show Zestril's indications in one reply and Keytruda's in one reply, with `remaining` absent.

## Starts from

- Starts from: 0882 (`remaining` before `text`), `crates/biomcp-adapters/src/openfda_label/page.rs`, `crates/biomcp-query/src/drug/label_bound.rs`, `sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md`.
- PM decision: 2026-11-13. The label section reply budget may grow beyond half its current size, up to 32 KB of text, chosen from the measurement so most indications sections fit in one reply; the budget is a setting with that default. This ruling is the one the measured-caps decision asks for.

## Keeps

- Keeps: label text verbatim; paging and `remaining` for sections larger than the budget; other entities' budgets.

## Defers

- Defers: FDA PDF text pages (device summaries, approval letters), which 0882 also deferred.

## Changes

- Changes: `crates/biomcp-adapters/src/openfda_label/`, `crates/biomcp-query/src/drug/`, `crates/biomcp/src/` (setting), `tests/cases/drug/`, `docs/`

## Progress

- Measured 2026-11-13 from openFDA (compact JSON-escaped bytes of the whole field): 300 human prescription labels from twelve spread pages (skip 0 to 22,000 of 36,250) have Indications and Usage median 932, 95th percentile 4,537, largest 10,161; 98.3 percent fit 6,144, 99.3 percent fit 8,192 and all fit 12,288. Across their 6,380 text sections of every kind, 83.5 percent fit 6,144, 88.6 percent 8,192, 99.5 percent 28,672 and 99.7 percent 32,768. Of 25 named labels most asked about (oncology brands and common drugs), 23 fit 8,192; Opdivo (12,307) and Keytruda (23,153 by brand search; 23,683 for set 9333c79b v229) need more.
- Change: a label section reply now takes as many whole 6,144-byte fragments as fit the text budget, instead of one by default; `--section-limit` still caps the fragment count. The budget is the config setting `drug_label_bytes` (6,144 to 32,768), default 28,672: Keytruda's 23,683 plus 15 percent, rounded up to whole KiB (`crates/biomcp-query/src/context.rs`). Fragment size stays 6,144 so offsets keep their meaning. The section ceiling in `label_bound.rs` is now the ruled 32,768 plus 2,048 for identity and boundaries. Docs: `docs/configuration.md`, drug slice allocation row.
- Live on 37b8298bc (job j39092): Keytruda indications one reply, 4 fragments, no `remaining`, 25,350 bytes; Opdivo one reply, 3 fragments, 13,909; Zestril one reply, 4 fragments, 8,097; Xarelto unchanged at 5,771. Keytruda clinical studies (35 fragments) pages four fragments a reply, `remaining` 31 then 27, 26,431 bytes. With `drug_label_bytes` 6,144 Keytruda indications pages as before, `remaining` 3.
- Cases: `drug/label-fills-budget` (Keytruda, j39048 replay, max_bytes 29,153 = 25,350 plus 15 percent), `drug/label-short-pieces` (Zestril, new j39092 recording, max_bytes 9,312), and `drug/label-continues` now sets `drug_label_bytes` 6,144 to keep proving paging and `remaining`.
- The runner's whole-section check (`drug-label/published-sections`) measured 26,146 comparable and 26,257 full bytes for its largest page at the new default; `tests/runner-output-budgets.yaml` now sets `label-comparable` 30,068 (plus 15 percent) and `label-full` 34,816 (the ruled 32 KiB plus 2 KiB), in place of 8,811 and 8,192.
- Gate j39126 passed on 30cfe93a1 (`make --keep-going landing-gate docs-check schema-release-check`); no schema change.
