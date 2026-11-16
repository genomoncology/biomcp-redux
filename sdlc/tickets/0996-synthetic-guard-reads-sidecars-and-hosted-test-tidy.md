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

## Progress

- Guard (`crates/biomcp/tests/ratchets/noncommercial_replies.rs`): an Orphadata body of kind `sidecar` is now read from its `.bin` beside the record (a missing sidecar is reported), and every `JDBOR` body, inline or sidecar, must state `dbserver="synthetic"` and give each disorder (the first `Name` after each `OrphaCode`) a name starting with "Synthetic". `real_orphadata_replies_fail` gains a hand-cut case (an authored server with the name "Kabuki syndrome") and a sidecar case (a real-server `.bin`).
- Base proof, pm job j41674 on throwaway branch `proof/0996-base-guard` at 4b5132f1e (base c2ca20f4f's guard with the new test, plus a copy that runs only the sidecar case): both fail on the old guard, with "a hand-cut real Orphadata file passed" and "a real Orphadata sidecar passed".
- README (`tests/replies/disease/orphadata-synthetic/README.md`) now lists what is invented (disorder names, code 900001, estimates, geographies, PubMed IDs 90000001-90000003) and what keeps a real case's shape (900001 cites the real PubMed IDs 18442953 and 33080481; code 2322's disorder type, onset ages and inheritance come from Orphanet's real entry, Kabuki syndrome). The data is unchanged, so no case step changes.
- `typed_descriptions.rs` reads the hosted catalog with `x-biomcp-use: academic`. `typed_policy::check` no longer sets `hosted_max_use: commercial` in that environment, since that ceiling refuses the academic header; its own hosted HTTP server keeps its commercial config.
- `mcp_reply_form.rs` and `typed_followups.rs` send the second (HTTP) request only in local mode; in hosted mode the one HTTP reply is checked alone. The unhosted stdio legs in `mcp_hosted.rs` are renamed `mcp-hosted/local-stdio/*`, since they check ordinary stdio replies as the contrast to hosted refusals.
- `docs/configuration.md` names `serve --hosted --http` for the hosted reply-form default.
- Cheap job j41676 at 3efa3ecad: no docs or schemas regenerated; every `mcp-parity` (65) and `mcp-hosted` (54) trial and all 50 ratchet tests passed; `check-cheap` failed only on `cargo fmt` and `too_many_lines` in the two hosted test functions, both fixed.
- Gate `make --keep-going landing-gate docs-check schema-release-check` passed in pm job j41686 at d82df6e90 (branch merged with origin/main 616e68d22).
