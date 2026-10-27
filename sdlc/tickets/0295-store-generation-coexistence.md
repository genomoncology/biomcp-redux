# 0295 — Keep bundles when two query store generations coexist

Status: COMPLETE.
Landed: 69a380ed518e767963619ee74c684d2958ed3bc7

Milestone: 5

Depends on: 0284

## Outcome

When an older and a newer query store file both exist, BioMCP reads synced bundles from whichever file holds them, transfers them into the current file under the store's ownership checks, and deletes nothing until the transfer is verified. `store clean` never removes the only copy of a bundle.

- Why: the fresh review of 0284 found that once a pre-fix binary has run, both generations coexist, path selection hides the populated older file, and `store clean` deletes its only bundles. PM ruling 2026-10-26 (m8126 ask 17) splits this from 0284 so each keeps a focused migration policy and proof, with the same developer and fixes batched.
- Starts from: the landed 0284 single-file upgrade, `crates/biomcp/src/paths.rs`, `bundle/store.rs` and `cleanup.rs`.
- Rules: when both generations hold a bundle of the same name, the current generation's published bundle wins and the older copy is not imported; only names missing from the current generation are transferred. No timestamp precedence. PM ruling 2026-10-26 (m8126 ask 23).
- Keeps: SQLite ownership checks before any journal or schema change, the 0284 upgrade path, and the rule that an upgrade never loses synced bundles.
- Changes: store selection or transfer when generations coexist, and cleanup.
- Proof: a case with the same bundle name in both generations reads the current copy and leaves the older copy untransferred; a case writes a synthetic bundle into an older generation, creates a newer empty generation, and reads the bundle without resync; a case interrupts the transfer and shows both copies survive; a case runs `store clean` with coexisting generations and shows no bundle is lost before transfer. No source calls.
- Defers: nothing.
