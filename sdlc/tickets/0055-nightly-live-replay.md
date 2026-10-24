# 0055 — Replay every public case live on a schedule

Status: OPEN.

Milestone: 1

Changes: crates/biomcp-fetch/src/, crates/biomcp-fetch/tests/quotas.rs, crates/biomcp/src/config.rs, crates/biomcp/src/grammar.rs, crates/biomcp/tests/cases.rs, crates/biomcp/tests/ratchets/source.rs, crates/biomcp/tests/runner/, sdlc/features.md, tests/cases/

## Outcome

A scheduled pm job builds main and replays every public case against the live sources, as architecture 5.5 and 9 promise, and reports drift without failing the landing gate. It catches upstream changes before users do: renamed fields, new zero counts, ignored parameters, moved endpoints, and changed bundle headers. 0.9 met each of these late (ORCID's lowercase "public", WHO's renamed headers, Europe PMC ignoring `page`, CPIC's renamed sort columns, GWAS answering 410).

- Starts from: architecture 5.5, 9 and 10.1; the runner's `--cases DIR`; `pm job`. GitHub Actions stay for public releases only.
- PM decision: 2026-10-24. The run is keyless and uses each source's public limit. Keyed cases are skipped. It never writes a recording. For each case, it reports a key field that changed, a count that drops to zero, a status that changed from complete, a new HTTP status, and a bundle whose header row changed. It also records cold timings and request counts per source (architecture 9). The report is one summary the coordinator reads; a person refreshes recordings on purpose with `--record`.
- Keeps: the landing gate, which never calls a live source.
- Proof: one run on main produces the report. A planted drift case, made by replaying a recording edited to rename a field and to return zero, appears in the report. The schedule is registered through pm.
- Defers: scoring runs over private case sets.

- Reviewed reliability boundary, 2026-10-24: Discover all public cases but run a distinct live-check mode over eligible public requests only. Explicitly skip synthetic/failure injection, local-only, keyed and external cases; never use host credentials. Keep ordinary replay assertions separate from drift comparisons; capture dates do not freeze live counts. Compare approved raw fields, positive-to-zero results, status transitions, schema failures and bundle headers through a bounded shared-transport observation surface; LegReport currently lacks all attempts/status history/bytes/durations and mapping can discard drift. Do not reconstruct HTTP or canonical identity. Use isolated cold caches and one shared limiter domain or deliberate serial pacing; independent child quotas cannot collectively enforce source rate. The edited-recording comparator proof is offline and leaves recordings unchanged. pm job has no schedule command: use a PM-named supported scheduling owner/recipe, no new flag or silent cron.

- PM ruling m4402: Jarvis owns machine schedules. This ticket delivers one bounded command for pm job and its recipe; the PM asks Jarvis to register the schedule after landing. Install no cron entry inside this ticket.

