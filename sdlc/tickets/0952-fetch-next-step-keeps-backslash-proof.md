# 0952 — fetch-next-step-keeps-backslash-proof

Status: COMPLETE.
Landed: f679c1fe0d7c4f8eb4e85ca69068f575534348b6

Milestone: 7

## Outcome

A replay case proves the MCP fetch tool's plain-text view keeps a backslash inside a fenced next-step block exactly as written, so removing 0938's fence copy in `crates/biomcp/src/mcp/research/text.rs` fails a test.

## Why

0938's review (2026-11-14): 0938 landed the fence copy without this proof, because no current reply put a backslash in a next step and a unit test hit the placement ratchet; before 0938 a `\\` inside a `json` typed-step fence would have collapsed to `\` and broken the JSON.

## Proof

- Proof: a replay case where an article card has a DOI and no PMID, the DOI holding a backslash (for example `10.1000/a\.b`; synthetic, since it is a shape test), calls `fetch` and asserts the `sh` fence holds `'10.1000/a\.b'`; it fails with the fence copy removed.

## Starts from

- Starts from: 0938's ticket and `crates/biomcp/tests/runner/source_text.rs` (`fetched()` helper); the DOI-only path to a PDF next step the 0938 developer named.

## Keeps

- Keeps: 0938's escaping and fetch behaviour.

## Defers

- Defers: nothing.

## Changes

- Changes: a case under `tests/cases/` and its replies; no product code unless the case finds a bug.

## Progress

- Path: a Europe PMC preprint card has a DOI and no PMID. When Europe PMC refuses its full text (fulltext `not-open`), `article/next.rs` offers the PDF step by DOI, and the view prints it as `biomcp get article '10.1000/a\.b' pdf` in an `sh` fence. In this code, refusal is the only way Europe PMC full text becomes `not-open`: a malformed core `license` is refused before any full-text request.
- Case `tests/cases/mcp/fetch-keeps-fence-backslash.yaml` calls the MCP `fetch` tool with `article PPR1064174 fulltext` and pins its whole text (the case runner has no substring check for JSON values), including the `sh` fence with `'10.1000/a\.b'`. Replies `tests/replies/article/preprint/synthetic-backslash-doi/` are synthetic (a shape test, as the ticket allows): the live PPR1064174 core request identity with a hand-written body. Its README says so.
- Measured: passes on e82004f3a (job j40956). On scratch commit f7f157cc2, which drops 0938's fence copy in `mcp/research/text.rs` and was pushed only to `scratch/0952-no-fence-copy`, it fails with the fence showing `'10.1000/a.b'` (job j40955).
- No product code changed.
