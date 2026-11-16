# 1001 — hosted-reply-cache-has-a-byte-ceiling

Status: OPEN.

Milestone: 7

## Outcome

A hosted server's reply cache stays under a configured size. Writes past the limit remove the oldest rows first, expired ones before them, so a hosted server can't fill its disk. The default comes with a stated reason.

## Why

Finding E1 of `sdlc/records/2026-11-14-adversarial-review.md` was deferred "before hosted deployment". Hosting is now close to launch (hosting handoff, 2026-11-16).

On main 82360be61 there is no size setting:

- each openFDA label page can be up to 32 MiB and is kept for a day (`openfda_label/mod.rs:37-41`);
- expired rows are pruned only when a new reply is written (`biomcp-fetch/src/cache.rs:301-316`);
- hosted mode refuses `store clean` (`docs/configuration.md:139`).

So each distinct drug-name search can pin about 30 MiB for a day.

## Proof

- Proof: a cache-level test writes past a small configured limit, and the store stays under it, oldest rows removed first. A test for the opposite case shows a fresh row is never removed while it is being served.
- Proof: the setting has a default and a reason: the measured size of a typical label page against a stated disk share. It is configurable, documented in `docs/configuration.md`, and has a release-notes line.

## Starts from

- Starts from: `biomcp-fetch/src/cache.rs` prune and write, the config keys in `crates/biomcp/src/config/file.rs`, finding E1's text.

## Keeps

- Keeps: credential-scoped licensed reuse and rights checks (0503, 0504, 0619); stale-copy notices; no refusal of `store clean` changes.

## Defers

- Defers: nothing.

## Changes

- Changes: the reply cache, a config key, docs and notes, tests.

## Progress

- 2026-11-16: measured live openFDA label name pages (`openfda.brand_name:"X" OR openfda.generic_name:"X"`, limit 50, the request `search_names` sends) for twelve common names: metformin 6.7 MB, atorvastatin 8.5, lisinopril 4.0, amoxicillin 3.6, ibuprofen 1.0, sertraline 8.4, warfarin 5.8, imatinib 8.1, pembrolizumab 2.1, osimertinib 0.3, trastuzumab 2.1, olaparib 0.3. Median 3.8 MB (3.6 MiB). A name with hundreds of labels reads several pages (metformin: 397 labels, 8 pages).
- Default `reply_cache_max_bytes` = 2 GiB: a tenth of the 20 GiB disk an ECS Fargate task gets by default (the planned deployment is ECS on Linux, 0987). It holds about 560 typical label pages, or 64 pages at the 32 MiB page bound. Recorded beside `REPLY_CACHE_MAX_BYTES` in `crates/biomcp-fetch/src/lib.rs` and in `docs/configuration.md`.
- `Cache::write` now, after the existing 30-day prune and the insert, removes rows other than the one just written, expired first and then oldest by `fetched_at`, until the stored bodies fit. A reply larger than the whole ceiling is answered but not stored, and its older copy is removed. The limit counts reply bodies only, not `sync` collections. Applies to every server, not only hosted. Cache keys, credential scoping, rights checks and `store clean` are unchanged.
- Tests: `crates/biomcp-fetch/tests/cache/ceiling.rs`.
