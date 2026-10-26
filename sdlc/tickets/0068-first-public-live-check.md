# 0068 — Run the first public live diagnostic from landed main

Status: OPEN.

Milestone: 1

Depends on: 0055

## Outcome

A build delegate runs the landed live-check command once over every eligible public case and reports source drift. The PM gives the bounded recipe to Jarvis for scheduling after 0069 and 0070 land, without repeating the whole public run first. The first run, j9937 at c589955, stays as evidence.

- Why: 0055 proves the diagnostic offline; its first real source run and machine schedule remain milestone work.
- Starts from: landed 0055, its live-check recipe, explicit per-case provenance and the PM ruling that Jarvis owns schedules.
- Keeps: keyless isolated settings, explicit synthetic/keyed/local/external skips, shared source pacing, bounded child and whole-run budgets, no recording writes, no live landing gate and no cron installation.
- Changes: no product changes are expected. Record the exact main revision, job result and evidence location through pm progress. A witnessed product or source defect becomes its own bug ticket.
- Proof: run one bounded pm job from landed main using the existing live-check command. Preserve its owned summary outside job TMPDIR and checkout before cleanup; move cited raw evidence to the configured evidence store. Read every reported drift/error and distinguish source outages from implementation defects without clinical inference. Report eligible/skipped totals, affected cases and request/timing totals. Do not retry the whole source run or refresh recordings automatically.
- Defers: private cases, real credentials, clinical interpretation, schedule installation by this delegate and publishing.
- PM ruling 2026-10-25: the reviewed manual replacement j13095 from eb9bb25c meets the report criterion: 487 cases discovered, 97 checked, 390 skipped, 11 drifted, 0 errors, wrapper failed on drift, five files and manifest verified in the evidence store, scratch removed. The schedule configuration is confirmed. The ticket closes when the first automatic scheduled run produces a report the same way, which proves the corrected dispatch. Closing claims neither healthy live sources nor resolved drift; the 11 drifted cases wait on 0180's witnesses, and no baseline refresh or further provider run is authorized here.

## Reviewed automatic result

The automatic scheduled run j14800 at `8f854c56c1747d2101d3e6272fd1f972450717e7` produced its report on 2026-10-26. Review verified all four file hashes in the Yellow evidence directory `data/biomcp-redux/0068-scheduled/20261026T080118Z-owirvcp0`. The report SHA-256 is `a953f4291adb9858a02a83f11c8dd929c97612de5adb0738c0720db13c97a51b`; the manifest SHA-256 is `fa12a6c643a78afe7afe0de64aac76ffa1a1bfb41d3627bc20a34173a8996915`.

The run discovered 563 cases, checked 97, skipped 466, reported 12 drifts and no errors, and took 613368 ms. The runner exited successfully; the wrapper failed on drift as expected. This result proves automatic dispatch and report preservation. It does not establish healthy sources or resolve drift. The bounded diagnosis went to the PM. PM message m6851 authorized recording this reviewed result and landing the evidence with the next batch. No additional provider run or baseline refresh forms part of this completion.
