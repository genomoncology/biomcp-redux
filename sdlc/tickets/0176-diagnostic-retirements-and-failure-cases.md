# 0176 — Retire old diagnostic options and prove diagnostic failure and identity cases

Status: OPEN.

Milestone: 4

Depends on: 0172
Depends on: 0173
Depends on: 0174
Depends on: 0175
Depends on: 0181

## Outcome

Every 0.9 diagnostic command, option and section that diagnostic.md retires, renames or defers returns a fixed usage error that names the supported route or the reason, and makes zero requests. `fda-pma` and `fda-510k` each report a failed outcome, never an empty one, on a 429, 500, timeout, broken body or HTTP 200 error body, a missing or stale bundle reports its own outcome and names its sync, and a mismatched returned identity is dropped with a problem and never enters the cache.

- Why: diagnostic.md's accounting retires or defers these options, and the slice rules require failure and identity cases for every adapter and bundle.
- Starts from: diagnostic.md's complete legacy surface accounting, deferred selectors and the empty, failure and deferred rows of the case map; the existing retirement table and usage errors, including the gene and disease `diagnostics` retirements; the adapters and bundles from 0172 to 0175.
- Rules: `--source all` and a missing source name the named sources. `-g` names `--gene`, `-t` names `--type`, `-l` names `--limit`, and `-d` and `--disease` name `--condition-id` with a MedGen CUI for GTR or an exact WHO product card. `--manufacturer`, a GTR laboratory selector and WHO type, marker and discovery search say the selector is deferred. `search diagnostic --source fda-pma --query` and free-text 510(k) search say they are deferred, and the 510(k) message names `get diagnostic K-NUMBER --source fda-510k` from 0181. `--full` and the `all` section say to name one section. The `regulatory` section on GTR or WHO names `get diagnostic ID --source fda-pma` and `search diagnostic --source fda-cdx --device-id`. `gtr sync` names `biomcp sync gtr` and `who-ivd sync` names `biomcp sync who-ivd`. `list diagnostic` names generated help. `-j` and `--json` name `--format json`. A read never downloads or refreshes a bundle. No retirement rewrites the query.
- Proof: outside-in cases first. A case per retired or deferred spelling asserts its message with zero provider or download requests. Synthetic 429, 500, timeout, broken body and HTTP 200 error replies for `fda-pma` and `fda-510k`, labeled synthetic, assert a failed outcome with source and terms. Missing and stale bundle cases for `gtr`, `who-ivd` and `fda-cdx` assert their own outcome, the named sync and zero requests. A synthetic wrong returned PMA or supplement number, and a wrong returned K number, each assert the drop, the problem and no cache admission. Q18's source-local zero and P000000's and K080653's native absence stay distinct from every failure. Each case has a size ceiling. This ticket adds no filter or exact-ID get and makes no provider call; the filters and exact-ID gets it relies on have known-positive captures through one bounded keyless pm job in 0172 to 0175 and 0181. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
