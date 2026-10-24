# 0057 — Carry each source's terms and switch sources off by group

Status: OPEN.

Milestone: 1

## Outcome

Each source's `SourceSpec` names its terms of use: the license or terms name, the publisher's page, and a closed `Use` set (`open`, `attribution`, `noncommercial`, `restricted`). `--disable-sources` and the config's `disable_sources` accept source names and the groups `noncommercial` and `restricted`. Every source honors them, keyed or not. `biomcp health` lists every source with its terms and whether it is switched off. A user asked for this switch in 0.9 (issue 290), and 0.9's switch covered only about eight sources while `health` claimed all of them were off.

- Starts from: architecture 4 (`SourceSpec`) and 8; the license notes in each slice design; the existing `--disable-sources`.
- PM decision: 2026-10-24. Terms come from the publisher's own page, and the slice that adds a source records them. A source with no recorded terms fails the ratchet test. A disabled source reports `skipped` with reason `disabled`, sends no request, and never reads as empty.
- Proof: a ratchet case refuses a planted source with no terms. A runner case with `--disable-sources noncommercial` shows each affected source skipped with reason `disabled`, and its recording proves no request was sent. `health` output lists every registered source with its terms and state, and its schema is checked in.
- Defers: per-section terms.
