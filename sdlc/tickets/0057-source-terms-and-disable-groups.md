# 0057 — Carry each source's terms and switch sources off by group

Status: COMPLETE.
Landed: 1c19718b3da289e906c415f2cc4e68f84ff85d4c

Milestone: 1

## Outcome

Each source's `SourceSpec` names its terms of use: the license or terms name, the publisher's page, and a closed `Use` set (`open`, `attribution`, `noncommercial`, `restricted`). `--disable-sources` and the config's `disable_sources` accept source names and the groups `noncommercial` and `restricted`. Every source honors them, keyed or not. `biomcp health` lists every source with its terms and whether it is switched off. A user asked for this switch in 0.9 (issue 290), and 0.9's switch covered only about eight sources while `health` claimed all of them were off.

- Starts from: architecture 4 (`SourceSpec`) and 8; the license notes in each slice design; the existing `--disable-sources`.
- PM decision: 2026-10-24. Terms come from the publisher's own page, and the slice that adds a source records them. A source with no recorded terms fails the ratchet test. A disabled source reports `skipped` with reason `disabled`, sends no request, and never reads as empty.
- Proof: a ratchet case refuses a planted source with no terms. A runner case with `--disable-sources noncommercial` shows each affected source skipped with reason `disabled`, and its recording proves no request was sent. `health` output lists every registered source with its terms and state, and its schema is checked in.
- Defers: per-section terms.

- Reviewed reliability boundary, 2026-10-24: There is no existing disabled-source setting or health command. Derive one adapter-owned implemented-source/terms registry from SourceSpec declarations, not every biodata Source variant or a duplicate terms table. Source and quota identities differ; disabling PubMed must not disable PubTator solely for shared host/provider. Enforce disabled before keys, quota, replay, reply/bundle caches and sync; stored pools cannot restore disabled rows or hydrate them. Preserve skipped versus empty/unavailable through CLI/MCP and required resolvers. Prove no request with seeded caches and stub counters. Terms use publisher evidence and reviewed categories, never inferred legal rights. Record CLI/config combination explicitly. The amended0059 contract requires bounded source reachability/latency for biomcp health, with approved probe declarations and disabled/no-key behavior; HTTP health separately checks local owned storage.

- PM ruling m4402: combine configured and CLI-disabled source sets by union. A command can disable additional sources and cannot re-enable a source disabled by configuration.
