# 0134 — Retire old drug options and prove drug failure and identity cases

Status: COMPLETE.
Landed: 10e2c2af316130c4dbc179f1a3dc0d8d2e591582

Milestone: 3

Depends on: 0126

Depends on: 0127

Depends on: 0128

Depends on: 0129

Depends on: 0130

Depends on: 0131

Depends on: 0132

## Outcome

Every 0.9 drug option and command that drug.md retires or leaves unproven returns a fixed usage error with guidance and makes zero requests. Every kept drug source reports a failed outcome, never an empty one, on a 503, 429, timeout or broken body. A mismatched or conflicting returned identity is dropped with a problem and never enters the cache.

- Why: drug.md's accounting retires or holds back these options, and the slice rules require recorded failure cases and identity checks for every adapter.
- Starts from: drug.md cases U01 to U05, F01 and X01 and its 0.9 accounting tables; the existing retirement table and usage errors; the adapters from 0126 to 0132.
- Rules: retirements use drug.md's exact messages, such as `drug cell-lines is retired: cell-line screening is outside the cancer-care drug slice.` and `cvx sync is retired: the cancer-care drug slice has no vaccine alias bridge.` Unproven filters such as `--target`, `--indication`, `--type`, `--atc` and `--pharm-class` say they are not yet supported, without mapping a source field. Old `ema sync` and `ddinter sync` name `biomcp sync ema` and `biomcp sync ddinter`, old `who sync` says WHO is outside this question bank, and the old `interactions` get section names the DDInter section. `shortage` and `all` are absent. No retirement rewrites the query. `identity_level` comes from the endpoint or record type, never from a field's shape.
- Proof: outside-in cases first. U01 to U05 assert each message with zero provider or download requests. F01 runs synthetic 503, 429, timeout and broken-body replies for every kept drug source and asserts a failed or partial outcome with source and terms, labeled synthetic. X01 asserts D04's real identity separation plus synthetic wrong returned ID, conflicting ID and missing join field cases, with no cache admission and no product pick. Each case has a size ceiling. No provider call. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
