# 0055 — Replay every public case live on a schedule

Status: OPEN.

Milestone: 1

## Outcome

A scheduled pm job builds main and replays every public case against the live sources, as architecture 5.5 and 9 promise, and reports drift without failing the landing gate. It catches upstream changes before users do: renamed fields, new zero counts, ignored parameters, moved endpoints, and changed bundle headers. 0.9 met each of these late (ORCID's lowercase "public", WHO's renamed headers, Europe PMC ignoring `page`, CPIC's renamed sort columns, GWAS answering 410).

- Starts from: architecture 5.5, 9 and 10.1; the runner's `--cases DIR`; `pm job`. GitHub Actions stay for public releases only.
- PM decision: 2026-10-24. The run is keyless and uses each source's public limit. Keyed cases are skipped. It never writes a recording. For each case, it reports a key field that changed, a count that drops to zero, a status that changed from complete, a new HTTP status, and a bundle whose header row changed. It also records cold timings and request counts per source (architecture 9). The report is one summary the coordinator reads; a person refreshes recordings on purpose with `--record`.
- Keeps: the landing gate, which never calls a live source.
- Proof: one run on main produces the report. A planted drift case, made by replaying a recording edited to rename a field and to return zero, appears in the report. The schedule is registered through pm.
- Defers: scoring runs over private case sets.
