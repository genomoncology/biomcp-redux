# 0144 — Retire old disease options and prove disease failure and identity cases

Status: OPEN.

Milestone: 3

Depends on: 0140

Depends on: 0141

Depends on: 0142

Depends on: 0143

## Outcome

Every 0.9 disease option, section and command that disease.md retires returns a fixed usage error that names the supported route or the dropped reason, and makes zero requests. `get disease CURIE genes --source opentargets` is refused as deferred with zero requests. Every kept disease source reports a failed outcome, never an empty one, on a failure, timeout or broken body, and a mismatched returned identity is dropped with a problem and never enters the cache.

- Why: disease.md's 0.9 accounting retires these options, and the slice rules require recorded failure and identity cases for every adapter.
- Starts from: disease.md's case map rows for retired filters, retired get sections, retired convenience commands, retired phenotype ranking, help and shared output, and empty and source failures; its 0.9 accounting table; the existing retirement table and usage errors; the adapters from 0140 to 0143.
- Rules: retired filters `--inheritance`, `--phenotype`, `--onset` and `--no-fallback` map no source field and never expand a clinical phrase. `-q` and `--query` recover to positional text, `-l` recovers to `--limit`, and old `--source mondo|doid|mesh` recovers to `--source ols --ontology`. Free-text get and `--name` return search guidance and never a guess. Retired sections `pathways`, `diagnostics`, `variants`, `models`, `prevalence`, `funding`, `civic`, `disgenet` and `all` give the design's dropped reason; `clinical_features` names `phenotypes`; the name-driven `survival` section names `search disease --source seer` and the site survival get. `disease trials`, `disease articles` and `disease drugs` name the explicit trial, article and drug commands with no hidden resolution or fanout. `search phenotype` is retired with no scored ranking. `list disease` names generated help, `-j` and `--json` name `--format json`, and `--no-cache` follows the shared retirement policy. No retirement rewrites the query.
- Proof: outside-in cases first. D01-retired-query, D13-retired-inheritance, D14-retired-phenotype, D18-retired-onset, D23-retired-fallback, D21-name-get, D05-retired-pathways, D12-retired-diagnostics, D22-retired-variants, D14-retired-models, D19-retired-prevalence, D13-retired-funding, D08-retired-civic/disgenet/all, D20-explicit-site-recovery, D08-trials-recovery, D24-articles-recovery, D11-drugs-recovery and D13/D14-retired-ranking assert each message with zero provider requests, plus cases for `-l`, old `--source mondo`, `clinical_features`, `list disease` and `-j`. A D11 case asserts the deferred Open Targets refusal. D02-literal-zero versus exact-get asserts that the exact get still returns the concept and no search repair occurs. D24-source-failed, D24-timeout and D24-broken-body use recorded failure controls through OLS search, and a synthetic wrong returned ID for each kept adapter asserts the drop, the problem and no cache admission. D24-GraphQL-data-errors waits with the deferred Open Targets section, since no kept disease source uses GraphQL. Each case has a size ceiling. This ticket adds no filter or exact-ID get and makes no provider call; the filters and exact-ID gets it relies on have known-positive captures through one bounded keyless pm job in 0140 to 0143. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
