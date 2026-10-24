# 0066 — Finish installed platform and HTTP proof from landed main

Status: OPEN.

Milestone: 1

Depends on: 0058
Depends on: 0059
Depends on: 0073
Depends on: 0074

## Outcome

A build delegate completes the remaining installed-artifact matrix from landed main before milestone closure and repeats it before release. Code landing remains separate from platform availability.

- Why: 0058 delivers local producers and the gate; its ARM, macOS/Homebrew, Windows and HTTP execution remain unproved.
- Starts from: 0058's installed-artifact recipe and explicit platform limits; 0059's HTTP contract; machine roles and Gdux priority. Native Linux wheel, desktop, registry and installer passed j9758; x86 manylinux and amd64 runtime subchecks passed j9726. Do not repeat those unchanged proofs merely to recreate a report.
- Keeps: standard offline installation, installed executable selection without checkout fallback, candidate-byte provenance, isolated settings and caches, all-frame stdio and ordinary shared-cache concurrency proof. Publishing still needs Ian.
- Changes: the existing release gate only if a witnessed defect requires a repair; record job results through pm progress. Raw cited evidence follows the experiment storage rules.
- Proof: run ARM containers on a capable machine, macOS wheel and local Homebrew formula on M5, Windows on the existing Gator VM only when shared capacity allows, and installed HTTP after 0059. Preserve exact candidate identity and per-channel results. Platform preparation may start after 0058; the final candidate, installer, HTTP and interrupt runs wait for 0059, 0073 and 0074. Report unavailable capacity explicitly and keep milestone proof open. Turn defects into bug tickets without reopening landed code.
- Defers: publishing and external channel changes.
