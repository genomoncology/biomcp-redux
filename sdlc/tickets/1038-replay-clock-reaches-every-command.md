# 1038 — replay-clock-reaches-every-command

Status: OPEN.

Milestone: 7

## Outcome

Every command that reads a source runs on 1035's replay clock, legs read one after another inside a worker add up, and `Leg::dependency` keeps the looked-up source's own time limit live.

## Why

1035's review (2026-11-17) accepted it with these gaps:
- Commands started from the plain deadline get no command clock: gene get (`crates/biomcp-query/src/gene/get.rs:20`), drug aliases, resolve and markets, gene search, health. Nothing checks that new code uses the command deadline.
- Two legs used one after another inside one worker each start at the command's time (`crates/biomcp-fetch/src/deadline.rs:79-83`, `124-132`), e.g. a variant section `fan_out` job.
- `Leg::dependency` (`crates/biomcp-fetch/src/lib.rs:512-515`) no longer applies the source's own limit; a user-set PubMed or PMC OA limit under 3 s is not enforced there, live.
- Drop folds worker time only on the command's own thread (`deadline.rs:124-131`, `352-362`); fold unconditionally.

## Proof

- Proof: a replayed gene get case shows HGNC then section time adding, failing on the base commit; a check fails on any new command that bypasses the command deadline; a test shows `Leg::dependency` honours a configured source limit below its cap.

## Starts from

- Starts from: 1035's clock in `crates/biomcp-fetch/src/deadline.rs`.

## Keeps

- Keeps: live deadlines and pacing.

## Defers

- Defers: nested commands sharing an outer command's clock.

## Changes

- Changes: the listed commands' deadline start, worker leg chaining, `Leg::dependency`, one check and cases.

## Progress

- `Leg::dependency` (live fix, `crates/biomcp-fetch/src/lib.rs`): the dependency's end is again the earliest of the calling leg's end, `cap`, and the dependency source's own limit (the user's `source_deadlines` setting, else its declared one). 1035 had dropped the last. Live this matches the end before 1035 exactly; nothing else live changes. Test: `dependency_honours_a_configured_source_limit_below_its_cap` (a 2 s PubMed reply fails a 1 s user limit inside a 10 s cap, and passes a 5 s one, its time charged to the caller).
- Worker legs (`crates/biomcp-fetch/src/deadline.rs`): each worker thread keeps its own time on the command clock, so legs one after another inside a worker add up, and workers side by side still start together. A command dropped on any thread folds every worker's time. Test: `command_legs_charge_their_replay_time_one_after_another` gains two workers that each read two 3 s replies in turn (about 6 s, not 3 s), with the command dropped off its own thread.
- Commands: gene get and health now start on `command_deadline` (one clock; live end unchanged). Gene search opens one command per read (HGNC, then ClinGen), each with its own budget as live, so replay charges each in turn. The drug orchestrators (aliases search/get/with_aliases/spelling, resolve get/discover/chembl, markets get, search-names resolve) open no leg: they run nested commands, which already charge shared replay time one after another, and read the time charged. Moving them onto a command clock would freeze their budget, since nested commands do not join an outer command's clock (deferred). They keep the plain deadline with a stated exemption.
- Check: `crates/biomcp-query/clippy.toml` disallows `Fetch::deadline`, so a new command in biomcp-query that bypasses the command deadline fails clippy unless it states why.
- Case: `tests/cases/gene/sections/sequential-sources.yaml` replays gene/protein's CDKN2A recordings with HGNC at 18 s and UniProt at 2.5 s (`tests/replies/gene/sequential`): together they pass the 20 s command budget, so the protein section's UniProt read fails with deadline-exceeded.
- j42269 (at 1128eb7c5): `biomcp-fetch` and `biomcp-query` tests, all `gene/` (28) and `protein/` (19) cases and health cases pass, including the new gene case. Base check: on main's `crates/` the gene case fails (UniProt completes, since its 2.5 s starts at the command's start). check-cheap found one test over 80 lines and the drug exemptions missing from the ratchet's lint-exception list; fixed in 2e4a304fe by splitting the worker test and listing the four drug files.
- j42271 (at 2e4a304fe): check-cheap passes with no fmt change. Base check on main's `biomcp-fetch/src`: the dependency test fails (the 2 s reply passes the 1 s limit) and the worker test fails (0 s charged when the command is dropped off its thread, before it reads the clock again).
- Gate: j42272 `make --keep-going landing-gate docs-check schema-release-check` passed at d1a485e23 (the branch merged with origin/main 7211c90e3).
- Land gate j42276 failed `worker_legs_one_after_another_add_up` once with 9.001 s, not about 6 s. Cause: the replay quota limiter (`crates/biomcp-fetch/src/quota/limiter.rs`) moved every request's clock up to the source's latest admission. One worker's second request, admitted at 3 s on its clock, then made the other worker's first request, still at 0 s on its own clock, wait until 3.001 s. That wait depended on which thread reached the limiter first. Fix: in replay, a request whose clock is behind the latest admission takes the first earlier gap that is at least one token's interval from every recent admission (the last 64 are logged), drawing no credit. With no such gap it waits as before. Requests in order, and all of live, are unchanged. The test now runs the two workers one after the other in real time, the order that failed, so it fails on the old limiter every time.
- j42278 (at cdda701dc, merged with origin/main ba530df61): check-cheap and all `biomcp-fetch` tests pass; the worker test passed 300 of 300 runs, and the whole `command_budget` group 100 of 100. On the old limiter the worker test fails 3 of 3 runs with 9.001 s. The only fmt change was in the limiter's gap search, committed after.
- Gate: j42279 `make --keep-going landing-gate docs-check schema-release-check` passed at 4d95ff950 (merged with origin/main ba530df61).
- Review changes to the limiter: it now keeps the latest admission dropped from its 64-entry log, so an early slot comes at least one interval after it. An early slot is also taken only when it ends at least one interval before the latest admission (`gap + interval <= latest`). Every early slot is checked against every admission near it, and 64 only bounds the search. Early slots apply only to burst-1 rates, which every declared rate is today; a larger burst always waits, as before, so it cannot double replay's rate. Test: `early_replay_slots_keep_clear_of_forgotten_admissions` (70 admissions 2 s apart, then a late request at the start, never lands within 1 s of any of them).
- j42280 (at d488496c1, merged with origin/main e69c9e6ad): check-cheap and all `biomcp-fetch` tests pass. The worker test passed 300 of 300 runs, the `command_budget` group 100 of 100, and the new limiter test 100 of 100. On the limiter at 7a0db13d7 the new test fails: the late request lands 0 s from a forgotten admission. The only fmt change was one assert in that test, committed after.
