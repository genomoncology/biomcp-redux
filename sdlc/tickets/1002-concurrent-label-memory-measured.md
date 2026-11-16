# 1002 — concurrent-label-memory-measured

Status: COMPLETE.
Landed: b76b57bf9c4ad2ed7aa1fcf2f6e2be3552c0aa87

Milestone: 7

## Outcome

The hosted server's peak memory with all its call slots reading large drug labels at once is measured and stated. If it exceeds a stated share of a hosted task's memory, it is bounded.

## Why

Finding E2 of `sdlc/records/2026-11-14-adversarial-review.md` was deferred "before hosted deployment, measurement first". Its static estimate is about 0.7 to 1 GiB transient at 8 calls, from 32 MiB label pages held in several copies. The estimate was never measured. Hosting is now close to launch.

## Proof

- Proof: one `pm job` runs the existing load harness, or a runner test, over hosted HTTP. It runs 8 concurrent `get drug` label calls on the largest recorded labels and records peak resident memory. The numbers and the command go in Progress and in `docs/configuration.md`, so hosting can size its tasks.
- Proof: only if the peak is too high, a bound (for example, how many label reads may run at once) is added with a test. Otherwise, Progress says why none is needed.

## Starts from

- Starts from: `openfda_label/mod.rs:37`, `crates/biomcp/src/config.rs:19`, the load harness under `scripts/` or `crates/biomcp/tests/`.

## Keeps

- Keeps: reply shape and caps.

## Defers

- Defers: nothing.

## Changes

- Changes: a measurement record, docs, and a bound only if one is needed.

## Progress

Command: `make hosted-label-memory` (`scripts/hosted_label_memory.py`), in a `pm job` on the M5 (macOS, Apple silicon, release build). Each scenario starts a fresh `biomcp serve --hosted --http` over replayed openFDA label pages, sends `get drug metformin label --label-section indications_and_usage` from every client at once, and reads peak resident memory from `wait4`'s `ru_maxrss` (bytes on macOS), checked against `ps` sampled every 20 ms. Large pages are served from sidecar files, so nothing goes live. The recorded pages hold only metformin's skip=50 page whole (9.1 MB); the other seven were trimmed, so "live-sized" scenarios pad each page back to its recorded live byte size (54 MB per lookup), and "near bound" also scales the largest page to 27 MB, under the 32 MiB page bound. Caller keys give each caller its own openFDA bucket; keyless calls share one bucket with 4 reads in flight. An idle server holds 22 MiB.

Peak resident memory (MiB):

| Scenario | Before, j41739 at 2bf1f2530 | After, j41741 |
|---|---|---|
| 1 call, recorded pages | 77 | 78 |
| 8 calls, keyless, recorded | 264 | 267 |
| 8 calls, 8 keys, recorded | 258 | 261 |
| 1 call, live-sized | 168 | 153 |
| 8 calls, keyless, live-sized | 667 | 398 |
| 8 calls, 8 keys, live-sized | 733 | 491 |
| 32 calls, 8 keys (4 each), live-sized | 2,057 | 952 |
| 8 calls, 8 keys, near bound | 1,109 | 1,139 |
| 32 calls, 8 keys, near bound | 3,064 | 2,261 |

Problem line: half of a 1 GiB task, 512 MiB. A core process should leave the other half for whatever shares its task and for headroom before an out-of-memory kill, since these peaks are transient and arrive together.

Finding: the name lookup kept every 50-label page whole until it chose a label, so a lookup held all of its pages (54 MB of label text for metformin, several times that once decoded). Fix: `search_names` keeps only names, identity and indications as each page arrives (`openfda_label/search.rs`, unit test `name_pages_keep_only_what_the_label_choice_reads`). Replies are unchanged: the 14 label cases pass (j41741). At the default 8 calls, live-sized peaks are now 398-491 MiB, under the line.

A count of large label pages decoded at once (3 by default, as `drug_label_pages_at_once`) was built and measured in j41758 and then removed. With it, 32 live-sized calls peaked at 926 MiB against 1,038 without it, and 32 near-bound calls at 1,395 against 1,717. 8 keyless live-sized calls rose to 533 MiB. A page's bytes are already in memory when its turn starts, so waiting calls still hold them. A count that also covers the network read would hold each turn for the page's download, about 1 s live: 32 metformin lookups read 256 pages, about 85 s at 3 at once, far past openFDA's 15 s deadline. Calls waited little for a turn in replay (32 live-sized calls took 1.28 s at the median with the count, 1.06 s without), because replay downloads take no time.

Bound: the call limit. At 32 calls or with pages near the bound the peak passes half of a 1 GiB task, so a deployment sizes `http_max_calls` to its task's memory from these numbers; docs/configuration.md states them. Linux allocates differently from macOS, so a Linux task can peak higher or lower.
