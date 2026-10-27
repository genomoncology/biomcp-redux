# 0342 — Report bundle and sync sources truthfully in health

Status: COMPLETE.
Landed: fbb56637a9f02002b20ee9cdff09de0db70a33d8

Milestone: 5

## Outcome

`biomcp health` reports each source by a probe that matches how BioMCP reaches it. Sources that work, such as cpic, clinpgx, gtr, who-ivd, who-pq, fda-cdx, fda-pma, fda-510k and vaers, no longer show `failed`/`rejected` with latency 0. A bundle source reports its local bundle state and, when online, its publisher's reachability. `syncing` appears only while a sync runs.

- Why: the docs lead's product findings report on main a01667d (2026-10-27), rechecked by the PM on the release candidate build from hosted run 40542150253. The PM's live health run on the release candidate shows those nine sources as `failed` with reason `rejected` and latency 0 while their commands and syncs work. A user reading health would turn off working sources.
- Starts from: the health command, the source registry's probe definitions and the bundle state reader.
- Rules: health makes at most one bounded request per source and never downloads a bundle.
- Proof: replay cases for each named source show a non-failed state; a case with a sync lock left by a finished process does not show `syncing`; a real publisher failure still shows `failed` with its reason.
- Changes: health probes, registry entries and cases.
- Defers: diagnosing slow live sources such as InterPro and GWAS timeouts, which goes to 0363.
