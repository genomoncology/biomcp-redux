# 0996 — synthetic-guard-reads-sidecars-and-hosted-test-tidy

Status: OPEN.

Milestone: 7

## Outcome

The synthetic-captures guard catches real bytes in sidecar `.bin` files too, the Orphadata README says exactly what is invented, and the hosted tests 0991 moved to HTTP check the widest use tier and carry accurate names.

## Why

From the review of 0990 and 0991 (2026-11-16), which accepted both:

1. **The guard skips sidecar files (medium).** `crates/biomcp/tests/ratchets/noncommercial_replies.rs:93-101` reads only the text inside a reply file. Since 0906, a recorded multi-file sync stores each file as a sidecar `.bin` (`record.rs:510-516`, `record_sidecar.rs:97`). So `--record` of `sync orphadata` into a folder marked synthetic would commit real files and pass the guard.
2. **A hand-cut real file can pass.** A real file with only its `dbserver` changed passes the guard (low).
3. **The README overstates what is invented.** `tests/replies/disease/orphadata-synthetic/README.md:3` says every PubMed ID is invented, but disorder 900001 cites the real IDs 18442953 and 33080481. The 2322 rows also copy Kabuki syndrome's onset, inheritance and type (low).
4. **The typed-description test checks too narrow a tier.** `typed_descriptions.rs:337` sends `x-biomcp-use: commercial`, so hosted descriptions for academic-only sources go unchecked. It should send `academic` (low).
5. **Two hosted checks compare HTTP with itself.** In `mcp_reply_form.rs:99-114` and `typed_followups.rs:142-157`, the second request is wasted, and `mcp-hosted/stdio/*` trial names now test ordinary stdio (low).
6. **One docs line is stale.** `docs/configuration.md:135` says `serve --hosted` (low).

## Proof

- Proof: `real_orphadata_replies_fail` gains a `.bin` sidecar case and a hand-cut case (a disorder name that doesn't start with "Synthetic"). Both fail the guard, and both pass on the base commit.
- Proof: the README names which values are invented and which keep the real case's shape.
- Proof: items 4 to 6 are fixed and the gate passes.

## Starts from

- Starts from: the files and lines above.

## Keeps

- Keeps: every Orphadata case step; hosted HTTP coverage.

## Defers

- Defers: nothing.

## Changes

- Changes: the guard and its test, the README, the hosted tests named above, one docs line.
