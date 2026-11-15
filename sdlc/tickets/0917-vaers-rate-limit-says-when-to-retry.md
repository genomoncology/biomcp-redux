# 0917 — vaers-rate-limit-says-when-to-retry

Status: COMPLETE.
Landed: 866742331f00f387f231e4b58dcd6e76384969ad

Milestone: 7

## Outcome

When CDC WONDER rate-limits a VAERS table, BioMCP tries once more after a short wait, within the source's time limit. If that also fails, the reply says when to retry.

## Why

In the pharmacovigilance persona's round 2 (bench `personas/pv-scientist/runs/2026-11-13-r2/`, log.md:35), the VAERS age table failed with `reason:"rate-limited"` and the notice "retry after the reported delay". It gave no delay. The agent's own retry about 25 s later worked. The persona asked for "a queued retry or a `--retry-after` hint in `next`" so the tables work "in a live committee meeting".

## Proof

- Proof:
  - a synthetic WONDER rate-limit reply followed by a recorded success, where the call succeeds;
  - two rate-limit replies, where the reply fails and names a retry time.

## Starts from

- Starts from: the VAERS adapter; 0909's command budget for slow sources.

## Keeps

- Keeps: honest failure when the retry fails; 0909's budgets.

## Defers

- Defers: retries for other sources.

## Changes

- Changes: the VAERS adapter, `tests/cases/adverse-event/`.

## Progress

- 2026-11-15 measured live on the M5 (j41108): CDC WONDER refuses a VAERS table request with HTTP 429 and no `Retry-After` header. Its body says "API/XML requests must have at least 15 seconds between consecutive requests". Six back-to-back requests drew one 200 then five 429s; polling every 6 s, a request succeeded about 16-20 s after the last accepted one, and the 429s in between did not restart the clock. BioMCP sent the VAERS POST once (a POST is not retried), so the refusal failed the table at once with no retry time.
- Changed: an endpoint may now declare `throttle_retry(wait)` in the transport. A 429 on such an endpoint, even a POST, is sent once more after the reply's retry time, or `wait` when it names none, keeping the same local quota admission because the source refused the request unread. If the wait would pass the command budget, or the second request is refused too, the source fails `rate-limited` with that retry time, which the reply's notice carries as `retry_after_seconds`. The VAERS table request declares a 15-second wait, inside its 60-second command budget (0909). No other source uses it. This reuses the existing retry loop and its budget checks rather than adding pacing: the refusals come from separate processes, which no in-process pacing (`Quota::default_rate`) can space.
- Proof: `crates/biomcp-fetch/tests/retries.rs` `throttle_retry_sends_a_refused_post_once_more_and_names_its_wait` (429 then 200 succeeds after the wait under a refuse-wait quota; 429 twice fails `rate-limited` with the wait) and `a_post_without_throttle_retry_is_sent_once`. Live (j41113, 3e7d5d6dd): an MMR age table started a second after a seriousness table drew a 429, waited, and completed in 18.8 s. Cases `tests/cases/adverse-event/vaers-rate-limited-retry.yaml` replays that recording; `vaers-rate-limited-twice.yaml` replays the recorded 429 twice and expects `rate-limited` with `retry_after_seconds: 15`. `docs/cautions.md` explains the limit.
- Gate j41129 on 24a113adb (branch merged with origin/main) passed `make --keep-going landing-gate docs-check schema-release-check`. Earlier gates failed only on formatting and a function-length lint (j41116, j41123); the twice-refused case first expected the wrong MCP error flag (j41116).
